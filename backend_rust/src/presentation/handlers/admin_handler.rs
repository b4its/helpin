//! Handler panel Admin: manajemen pengguna, karyawan, supplier, aktivitas, statistik.

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;

use crate::application::admin::activity::{self, ActivityCtx};
use crate::application::auth::register;
use crate::domain::traits::repository::UserRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::infrastructure::middleware::client_meta::ClientMeta;
use crate::presentation::dto::response::MessageResponse;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn db_err(e: sqlx::Error) -> AppError {
    AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    }
}

async fn actor_name(state: &AppState, id: Uuid) -> String {
    state
        .user_repo
        .find_by_id(id)
        .await
        .ok()
        .flatten()
        .map(|u| u.name)
        .unwrap_or_else(|| "Unknown".to_string())
}

#[derive(Debug, FromRow, Serialize)]
pub struct UserRow {
    id: Uuid,
    name: String,
    email: String,
    role: String,
    phone: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// ===================== PENGGUNA (semua user) =====================

/// GET /api/admin/users — semua pengguna + role (untuk view admin).
pub async fn list_users(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserRow>>, AppError> {
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT id, name, email, role, phone, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(db_err)?;
    Ok(Json(rows))
}

// ===================== KARYAWAN =====================

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
}

/// GET /api/admin/employees — daftar karyawan (role = karyawan).
pub async fn list_employees(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserRow>>, AppError> {
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT id, name, email, role, phone, created_at FROM users WHERE role = 'karyawan' ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(db_err)?;
    Ok(Json(rows))
}

/// POST /api/admin/employees — buat karyawan baru.
pub async fn create_employee(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<CreateEmployeeRequest>,
) -> Result<Json<Value>, AppError> {
    if body.name.trim().is_empty() || body.email.trim().is_empty() || body.password.len() < 8 {
        return Err(AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: "Nama, email wajib & password minimal 8 karakter".to_string(),
            details: None,
        });
    }

    let out = register::register(
        register::RegisterInput {
            name: body.name.clone(),
            email: body.email.clone(),
            password: body.password,
            role: "karyawan".to_string(),
        },
        &state.user_repo,
        &state.jwt_service,
        &state.crypto_service,
    )
    .await?;

    if let Some(phone) = &body.phone {
        let _ = sqlx::query("UPDATE users SET phone = $2 WHERE id = $1")
            .bind(out.user_id)
            .bind(phone)
            .execute(&state.pool)
            .await;
    }

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(
        &state.mongo,
        &state.blockchain_service,
        &meta,
        ActivityCtx {
            title: "Penambahan Karyawan".to_string(),
            action_type: "CREATED".to_string(),
            table_affected: "users".to_string(),
            description: format!("Karyawan baru '{}' ({}) ditambahkan", body.name, body.email),
            username: actor,
            old_data: json!({}),
            new_data: json!({ "id": out.user_id, "name": body.name, "email": body.email, "role": "karyawan" }),
        },
    )
    .await;

    Ok(Json(json!({ "id": out.user_id, "message": "Karyawan dibuat" })))
}

/// PUT /api/admin/employees/:id
pub async fn update_employee(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateEmployeeRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let old = sqlx::query_as::<_, UserRow>(
        "SELECT id, name, email, role, phone, created_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(db_err)?
    .ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Karyawan tidak ditemukan".to_string(),
        details: None,
    })?;

    if let Some(name) = &body.name {
        let _ = sqlx::query("UPDATE users SET name = $2, updated_at = NOW() WHERE id = $1")
            .bind(id).bind(name).execute(&state.pool).await;
    }
    if let Some(phone) = &body.phone {
        let _ = sqlx::query("UPDATE users SET phone = $2, updated_at = NOW() WHERE id = $1")
            .bind(id).bind(phone).execute(&state.pool).await;
    }
    if let Some(pwd) = &body.password {
        if pwd.len() >= 8 {
            use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
            use rand::rngs::OsRng;
            let salt = SaltString::generate(&mut OsRng);
            if let Ok(hash) = Argon2::default().hash_password(pwd.as_bytes(), &salt) {
                let _ = sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
                    .bind(id).bind(hash.to_string()).execute(&state.pool).await;
            }
        }
    }

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Update Karyawan".to_string(),
        action_type: "UPDATED".to_string(),
        table_affected: "users".to_string(),
        description: format!("Data karyawan '{}' diperbarui", old.name),
        username: actor,
        old_data: json!({ "name": old.name, "phone": old.phone }),
        new_data: json!({ "name": body.name, "phone": body.phone }),
    }).await;

    Ok(Json(MessageResponse { message: "Karyawan diperbarui".to_string() }))
}

/// DELETE /api/admin/employees/:id
pub async fn delete_employee(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    let old = sqlx::query_as::<_, UserRow>(
        "SELECT id, name, email, role, phone, created_at FROM users WHERE id = $1",
    )
    .bind(id).fetch_optional(&state.pool).await.map_err(db_err)?;

    sqlx::query("DELETE FROM users WHERE id = $1 AND role = 'karyawan'")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(db_err)?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Penghapusan Karyawan".to_string(),
        action_type: "DELETED".to_string(),
        table_affected: "users".to_string(),
        description: format!("Karyawan '{}' dihapus", old.as_ref().map(|u| u.name.clone()).unwrap_or_default()),
        username: actor,
        old_data: old.map(|u| json!({ "name": u.name, "email": u.email })).unwrap_or(json!({})),
        new_data: json!({}),
    }).await;

    Ok(Json(MessageResponse { message: "Karyawan dihapus".to_string() }))
}

// ===================== SUPPLIER =====================

#[derive(Debug, FromRow, Serialize)]
pub struct SupplierRow {
    id: Uuid,
    name: String,
    contact: Option<String>,
    commodity: Option<String>,
    address: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SupplierRequest {
    pub name: String,
    pub contact: Option<String>,
    pub commodity: Option<String>,
    pub address: Option<String>,
}

pub async fn list_suppliers(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<SupplierRow>>, AppError> {
    let rows = sqlx::query_as::<_, SupplierRow>(
        "SELECT id, name, contact, commodity, address, created_at FROM suppliers ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(db_err)?;
    Ok(Json(rows))
}

pub async fn create_supplier(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<SupplierRequest>,
) -> Result<Json<SupplierRow>, AppError> {
    if body.name.trim().is_empty() {
        return Err(AppError { status: axum::http::StatusCode::BAD_REQUEST, message: "Nama supplier wajib diisi".to_string(), details: None });
    }
    let row = sqlx::query_as::<_, SupplierRow>(
        r#"INSERT INTO suppliers (id, name, contact, commodity, address) VALUES ($1,$2,$3,$4,$5)
           RETURNING id, name, contact, commodity, address, created_at"#,
    )
    .bind(Uuid::new_v4())
    .bind(&body.name)
    .bind(&body.contact)
    .bind(&body.commodity)
    .bind(&body.address)
    .fetch_one(&state.pool)
    .await
    .map_err(db_err)?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Penambahan Supplier".to_string(),
        action_type: "CREATED".to_string(),
        table_affected: "suppliers".to_string(),
        description: format!("Supplier '{}' ditambahkan", body.name),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "name": body.name, "contact": body.contact, "commodity": body.commodity }),
    }).await;

    Ok(Json(row))
}

pub async fn update_supplier(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<SupplierRequest>,
) -> Result<Json<SupplierRow>, AppError> {
    let old = sqlx::query_as::<_, SupplierRow>(
        "SELECT id, name, contact, commodity, address, created_at FROM suppliers WHERE id = $1",
    ).bind(id).fetch_optional(&state.pool).await.map_err(db_err)?
    .ok_or_else(|| AppError { status: axum::http::StatusCode::NOT_FOUND, message: "Supplier tidak ditemukan".to_string(), details: None })?;

    let row = sqlx::query_as::<_, SupplierRow>(
        r#"UPDATE suppliers SET name=$2, contact=$3, commodity=$4, address=$5, updated_at=NOW() WHERE id=$1
           RETURNING id, name, contact, commodity, address, created_at"#,
    )
    .bind(id).bind(&body.name).bind(&body.contact).bind(&body.commodity).bind(&body.address)
    .fetch_one(&state.pool).await.map_err(db_err)?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Update Supplier".to_string(),
        action_type: "UPDATED".to_string(),
        table_affected: "suppliers".to_string(),
        description: format!("Supplier '{}' diperbarui", old.name),
        username: actor,
        old_data: json!({ "name": old.name, "contact": old.contact, "commodity": old.commodity, "address": old.address }),
        new_data: json!({ "name": body.name, "contact": body.contact, "commodity": body.commodity, "address": body.address }),
    }).await;

    Ok(Json(row))
}

pub async fn delete_supplier(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    let old = sqlx::query_as::<_, SupplierRow>(
        "SELECT id, name, contact, commodity, address, created_at FROM suppliers WHERE id = $1",
    ).bind(id).fetch_optional(&state.pool).await.map_err(db_err)?;

    sqlx::query("DELETE FROM suppliers WHERE id = $1").bind(id).execute(&state.pool).await.map_err(db_err)?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Penghapusan Supplier".to_string(),
        action_type: "DELETED".to_string(),
        table_affected: "suppliers".to_string(),
        description: format!("Supplier '{}' dihapus", old.as_ref().map(|s| s.name.clone()).unwrap_or_default()),
        username: actor,
        old_data: old.map(|s| json!({ "name": s.name, "contact": s.contact })).unwrap_or(json!({})),
        new_data: json!({}),
    }).await;

    Ok(Json(MessageResponse { message: "Supplier dihapus".to_string() }))
}

// ===================== AKTIVITAS =====================

/// GET /api/admin/activities — daftar log aktivitas (dari MongoDB).
pub async fn list_activities(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Value>>, AppError> {
    let items = state.mongo.list_activities(200).await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Gagal membaca aktivitas: {}", e),
        details: None,
    })?;
    Ok(Json(items))
}

// ===================== STATISTIK DASHBOARD =====================

/// GET /api/admin/stats — ringkasan untuk dashboard admin.
pub async fn stats(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let count = |q: &'static str| {
        let pool = state.pool.clone();
        async move { sqlx::query_scalar::<_, i64>(q).fetch_one(&pool).await.unwrap_or(0) }
    };
    let total_users = count("SELECT COUNT(*) FROM users").await;
    let total_products = count("SELECT COUNT(*) FROM products").await;
    let total_suppliers = count("SELECT COUNT(*) FROM suppliers").await;
    let total_employees = count("SELECT COUNT(*) FROM users WHERE role='karyawan'").await;
    let total_pos = count("SELECT COUNT(*) FROM pos_transactions").await;
    let balance = state.financial_repo.get_balance().await.unwrap_or(0);
    let income = sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(amount),0)::bigint FROM financial_records WHERE record_type='pemasukan'").fetch_one(&state.pool).await.unwrap_or(0);
    let expense = sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(amount),0)::bigint FROM financial_records WHERE record_type='pengeluaran'").fetch_one(&state.pool).await.unwrap_or(0);

    Ok(Json(json!({
        "total_users": total_users,
        "total_products": total_products,
        "total_suppliers": total_suppliers,
        "total_employees": total_employees,
        "total_pos_transactions": total_pos,
        "balance": balance,
        "total_income": income,
        "total_expense": expense,
    })))
}
