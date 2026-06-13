use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::inventory::Inventory;
use crate::domain::traits::repository::InventoryRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{CreateInventoryRequest, UpdateInventoryRequest};
use crate::presentation::dto::response::{InventoryResponse, MessageResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_inventory_response(inv: Inventory) -> InventoryResponse {
    InventoryResponse {
        id: inv.id,
        owner_id: inv.owner_id,
        land_id: inv.land_id,
        category: inv.category,
        name: inv.name,
        quantity: inv.quantity.to_f64().unwrap_or(0.0),
        unit: inv.unit,
        created_at: inv.created_at,
    }
}

/// GET /api/inventory — semua inventori milik petani yang login
pub async fn list_inventory(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<InventoryResponse>>, AppError> {
    let items = state.inventory_repo.find_all_by_owner(auth.user_id).await?;
    Ok(Json(items.into_iter().map(to_inventory_response).collect()))
}

/// GET /api/inventory/available — inventori yang relevan untuk lahan (Pupuk, Bibit, Alat Pertanian)
pub async fn list_inventory_for_land(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<InventoryResponse>>, AppError> {
    let items = state.inventory_repo
        .find_by_owner_and_categories(auth.user_id, &["Pupuk", "Bibit", "Alat Pertanian"])
        .await?;
    Ok(Json(items.into_iter().map(to_inventory_response).collect()))
}

/// POST /api/inventory — buat inventori baru (milik petani, independen dari lahan)
pub async fn create_inventory(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateInventoryRequest>,
) -> Result<Json<InventoryResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let valid_categories = ["Pupuk", "Bibit", "Alat Pertanian", "Pakan", "Lainnya"];
    if !valid_categories.contains(&body.category.as_str()) {
        return Err(AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: "Kategori tidak valid. Harus: Pupuk, Bibit, Alat Pertanian, Pakan, atau Lainnya".to_string(),
            details: None,
        });
    }

    let inventory = Inventory {
        id: Uuid::new_v4(),
        owner_id: auth.user_id,
        land_id: None, // inventori bebas, tidak terikat lahan saat dibuat
        category: body.category,
        name: body.name,
        quantity: bigdecimal::BigDecimal::try_from(body.quantity).map_err(|e| AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: format!("Invalid quantity: {}", e),
            details: None,
        })?,
        unit: body.unit,
        created_at: None,
    };

    let created = state.inventory_repo.create(&inventory).await?;
    Ok(Json(to_inventory_response(created)))
}

/// PUT /api/inventory/:id — update inventori
pub async fn update_inventory(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateInventoryRequest>,
) -> Result<Json<InventoryResponse>, AppError> {
    let mut inventory = state.inventory_repo.find_by_id(id).await?.ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Inventori tidak ditemukan".to_string(),
        details: None,
    })?;

    if inventory.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke inventori ini".to_string(),
            details: None,
        });
    }

    if let Some(name) = body.name { inventory.name = name; }
    if let Some(category) = body.category { inventory.category = category; }
    if let Some(quantity) = body.quantity {
        inventory.quantity = bigdecimal::BigDecimal::try_from(quantity).map_err(|e| AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: format!("Invalid quantity: {}", e),
            details: None,
        })?;
    }
    if let Some(unit) = body.unit { inventory.unit = unit; }

    let updated = state.inventory_repo.update(&inventory).await?;
    Ok(Json(to_inventory_response(updated)))
}

/// DELETE /api/inventory/:id — hapus inventori
pub async fn delete_inventory(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    let inventory = state.inventory_repo.find_by_id(id).await?.ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Inventori tidak ditemukan".to_string(),
        details: None,
    })?;

    if inventory.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke inventori ini".to_string(),
            details: None,
        });
    }

    state.inventory_repo.delete(id).await?;
    Ok(Json(MessageResponse { message: "Inventori berhasil dihapus".to_string() }))
}

/// GET /api/lands/:land_id/inventory — inventori yang dipakai di lahan tertentu (via junction)
pub async fn list_inventory_by_land(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
) -> Result<Json<Vec<InventoryResponse>>, AppError> {
    let items = state.inventory_repo.find_by_land(land_id).await?;
    Ok(Json(items.into_iter().map(to_inventory_response).collect()))
}
