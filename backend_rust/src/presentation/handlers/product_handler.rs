use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;
use validator::Validate;

use crate::application::admin::activity::{self, ActivityCtx};
use crate::application::ecommerce::manage_product;
use crate::application::finance::manage_kas;
use crate::domain::entities::product::{Category, Product};
use crate::domain::traits::repository::UserRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::infrastructure::middleware::client_meta::ClientMeta;
use crate::presentation::dto::request::{CreateProductRequest, UpdateProductRequest};
use crate::presentation::dto::response::{CategoryResponse, MessageResponse, ProductResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_product_response(p: Product) -> ProductResponse {
    ProductResponse {
        id: p.id,
        seller_id: p.seller_id,
        category_id: p.category_id,
        name: p.name,
        price: p.price,
        stock: p.stock,
        unit: p.unit,
        product_type: p.product_type,
        location: p.location,
        image_url: p.image_url,
        description: p.description,
        nutrition: p.nutrition,
        purchase_price: p.purchase_price,
        expired_at: p.expired_at,
        entry_date: p.entry_date,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

async fn actor_name(state: &AppState, id: Uuid) -> String {
    state.user_repo.find_by_id(id).await.ok().flatten().map(|u| u.name).unwrap_or_else(|| "Unknown".to_string())
}

/// Admin & karyawan boleh mengelola produk milik siapa pun.
fn is_privileged(auth: &AuthUser) -> bool {
    matches!(auth.role.as_str(), "admin" | "karyawan")
}

/// Cari category_id berdasarkan nama; buat baru jika belum ada (upsert).
async fn resolve_category_id(pool: &sqlx::PgPool, name: &str) -> Option<Uuid> {
    let n = name.trim();
    if n.is_empty() {
        return None;
    }
    if let Ok(Some(id)) =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM categories WHERE LOWER(name) = LOWER($1) LIMIT 1")
            .bind(n)
            .fetch_optional(pool)
            .await
    {
        return Some(id);
    }
    let id = Uuid::new_v4();
    match sqlx::query("INSERT INTO categories (id, name, item_count) VALUES ($1, $2, 0)")
        .bind(id)
        .bind(n)
        .execute(pool)
        .await
    {
        Ok(_) => Some(id),
        Err(_) => None,
    }
}

/// Estimasi kandungan gizi rule-based dari nama/kategori produk (mode offline).
fn estimate_nutrition(name: &str, category: &str) -> Value {
    let n = format!("{} {}", name, category).to_lowercase();
    // (calories per 100g, protein, carbohydrate, fat, sugar, fiber, sodium_mg)
    let (cal, prot, carb, fat, sugar, fiber, sodium) = if n.contains("mie") || n.contains("pasta") || n.contains("nasi") || n.contains("beras") || n.contains("roti") {
        (350.0, 8.0, 70.0, 5.0, 3.0, 2.0, 400.0)
    } else if n.contains("susu") || n.contains("yogurt") || n.contains("keju") {
        (120.0, 7.0, 9.0, 6.0, 8.0, 0.0, 90.0)
    } else if n.contains("daging") || n.contains("ayam") || n.contains("ikan") || n.contains("telur") || n.contains("sapi") {
        (200.0, 22.0, 1.0, 12.0, 0.0, 0.0, 70.0)
    } else if n.contains("sayur") || n.contains("buah") || n.contains("hijau") {
        (45.0, 2.0, 9.0, 0.3, 5.0, 3.5, 15.0)
    } else if n.contains("minuman") || n.contains("teh") || n.contains("kopi") || n.contains("jus") || n.contains("soda") {
        (60.0, 0.5, 14.0, 0.0, 13.0, 0.0, 10.0)
    } else if n.contains("snack") || n.contains("keripik") || n.contains("biskuit") || n.contains("coklat") {
        (480.0, 6.0, 60.0, 24.0, 25.0, 2.0, 350.0)
    } else if n.contains("pupuk") || n.contains("pakan") || n.contains("bibit") || n.contains("alat") {
        // produk pertanian/peternakan — gizi tidak relevan
        return json!({ "type": "non-pangan", "note": "Produk non-konsumsi, analisa gizi tidak berlaku" });
    } else {
        (250.0, 6.0, 40.0, 8.0, 10.0, 2.0, 200.0)
    };
    let grade = if fat > 20.0 || sugar > 22.0 || sodium > 400.0 { "C" }
        else if prot >= 15.0 || fiber >= 3.0 { "A" } else { "B" };
    json!({
        "per_100g": {
            "calories_kcal": cal, "protein_g": prot, "carbohydrate_g": carb,
            "fat_g": fat, "sugar_g": sugar, "fiber_g": fiber, "sodium_mg": sodium
        },
        "nutri_grade": grade,
        "source": "ensemble"
    })
}

async fn analyze_nutrition(config: &crate::config::Config, name: &str, category: &str) -> Value {
    if !config.openrouter_api_key.is_empty() {
        if let Ok(v) = call_openrouter_nutrition(&config.openrouter_api_key, name, category).await {
            return v;
        }
    }
    estimate_nutrition(name, category)
}

async fn call_openrouter_nutrition(api_key: &str, name: &str, category: &str) -> anyhow::Result<Value> {
    let client = reqwest::Client::new();
    let prompt = format!(
        r#"Kamu ahli gizi. Berikan estimasi kandungan gizi per 100g untuk produk "{}" (kategori: {}). JSON saja:
{{"per_100g":{{"calories_kcal":<n>,"protein_g":<n>,"carbohydrate_g":<n>,"fat_g":<n>,"sugar_g":<n>,"fiber_g":<n>,"sodium_mg":<n>}},"nutri_grade":"A|B|C","highlights":["..."],"source":"openrouter"}}"#,
        name, category
    );
    let body = json!({
        "model": "google/gemini-2.0-flash-001",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3, "max_tokens": 600
    });
    let resp = client.post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body).timeout(std::time::Duration::from_secs(15)).send().await?;
    if !resp.status().is_success() { return Err(anyhow::anyhow!("status {}", resp.status())); }
    let result: Value = resp.json().await?;
    let content = result["choices"][0]["message"]["content"].as_str().ok_or_else(|| anyhow::anyhow!("no content"))?;
    let s = if content.contains("```json") { content.split("```json").nth(1).and_then(|x| x.split("```").next()).unwrap_or(content) }
        else if content.contains("```") { content.split("```").nth(1).unwrap_or(content) } else { content };
    Ok(serde_json::from_str(s.trim())?)
}

pub async fn create(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    // Auto-analisa kandungan gizi jika tidak diberikan
    let nutrition = match &body.nutrition {
        Some(n) if !n.is_null() => Some(n.clone()),
        _ => Some(analyze_nutrition(&state.config, &body.name, body.product_type.as_deref().unwrap_or("")).await),
    };

    let stock = body.stock;
    let purchase_price = body.purchase_price.unwrap_or(0);
    let name = body.name.clone();

    // Tentukan category_id: pakai yang dikirim, atau resolusi dari nama kategori (product_type)
    let category_id = match body.category_id {
        Some(c) => Some(c),
        None => resolve_category_id(&state.pool, body.product_type.as_deref().unwrap_or("")).await,
    };

    let input = manage_product::CreateProductInput {
        seller_id: auth.user_id,
        category_id,
        name: body.name,
        price: body.price,
        stock: body.stock,
        unit: body.unit,
        product_type: body.product_type,
        location: body.location,
        image_url: body.image_url,
        description: body.description,
        nutrition,
        purchase_price: body.purchase_price,
        expired_at: body.expired_at,
        entry_date: body.entry_date,
    };

    let product = manage_product::create_product(input, &state.product_repo).await?;

    // Auto-catat pengeluaran berdasarkan harga beli * stok
    if purchase_price > 0 && stock > 0 {
        let total_cost = purchase_price * stock as i64;
        let _ = manage_kas::record_expense(
            manage_kas::RecordExpenseInput {
                amount: total_cost,
                category: Some("Pembelian Produk".to_string()),
                description: Some(format!("Pembelian stok produk '{}' ({} x {})", name, stock, purchase_price)),
            },
            &state.financial_repo,
        ).await;
    }

    // Activity log
    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Penambahan Produk".to_string(),
        action_type: "CREATED".to_string(),
        table_affected: "products".to_string(),
        description: format!("Produk '{}' ditambahkan (stok {}, harga beli {})", name, stock, purchase_price),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "id": product.id, "name": name, "price": product.price, "purchase_price": purchase_price, "stock": stock }),
    }).await;

    Ok(Json(to_product_response(product)))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let products = manage_product::list_products(&state.product_repo).await?;
    let response: Vec<ProductResponse> = products.into_iter().map(to_product_response).collect();
    Ok(Json(response))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = manage_product::get_product(id, &state.product_repo).await?;
    Ok(Json(to_product_response(product)))
}

pub async fn update(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    // Resolusi category_id dari product_type jika tidak dikirim eksplisit
    let category_id = match body.category_id {
        Some(c) => Some(c),
        None => match &body.product_type {
            Some(pt) => resolve_category_id(&state.pool, pt).await,
            None => None,
        },
    };

    let input = manage_product::UpdateProductInput {
        id,
        seller_id: auth.user_id,
        category_id,
        name: body.name.clone(),
        price: body.price,
        stock: body.stock,
        unit: body.unit,
        product_type: body.product_type,
        location: body.location,
        image_url: body.image_url,
        description: body.description,
        nutrition: body.nutrition,
        purchase_price: body.purchase_price,
        expired_at: body.expired_at,
        entry_date: body.entry_date,
    };

    let product = manage_product::update_product(input, &state.product_repo, is_privileged(&auth)).await?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Update Produk".to_string(),
        action_type: "UPDATED".to_string(),
        table_affected: "products".to_string(),
        description: format!("Produk '{}' diperbarui", product.name),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "id": product.id, "name": product.name, "price": product.price, "stock": product.stock }),
    }).await;

    Ok(Json(to_product_response(product)))
}

pub async fn delete(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    let old = manage_product::get_product(id, &state.product_repo).await.ok();
    manage_product::delete_product(id, auth.user_id, &state.product_repo, is_privileged(&auth)).await?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Penghapusan Produk".to_string(),
        action_type: "DELETED".to_string(),
        table_affected: "products".to_string(),
        description: format!("Produk '{}' dihapus", old.as_ref().map(|p| p.name.clone()).unwrap_or_default()),
        username: actor,
        old_data: old.map(|p| json!({ "name": p.name, "price": p.price })).unwrap_or(json!({})),
        new_data: json!({}),
    }).await;

    Ok(Json(MessageResponse { message: "Produk berhasil dihapus".to_string() }))
}

pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryResponse>>, AppError> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, icon, item_count FROM categories ORDER BY name",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let response: Vec<CategoryResponse> = categories
        .into_iter()
        .map(|c| CategoryResponse { id: c.id, name: c.name, icon: c.icon, item_count: c.item_count })
        .collect();
    Ok(Json(response))
}
