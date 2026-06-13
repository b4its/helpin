use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use uuid::Uuid;
use validator::Validate;

use crate::application::livestock::pen_condition;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{CreatePenRequest, UpdatePenRequest};
use crate::presentation::dto::response::{LivestockResponse, MessageResponse, PenResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

use crate::domain::entities::pen::Pen;

const PEN_COLS: &str =
    "id, owner_id, name, capacity, pen_type, location, condition, created_at";

fn to_pen_response(p: Pen, occupancy: i64) -> PenResponse {
    PenResponse {
        id: p.id,
        owner_id: p.owner_id,
        name: p.name,
        capacity: p.capacity,
        pen_type: p.pen_type,
        location: p.location,
        condition: p.condition,
        occupancy,
        created_at: p.created_at,
    }
}

fn db_err(e: sqlx::Error) -> AppError {
    AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    }
}

pub async fn create(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreatePenRequest>,
) -> Result<Json<PenResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let id = Uuid::new_v4();
    // Kondisi awal di-generate otomatis (kandang kosong saat dibuat)
    let initial_condition = pen_condition::generate_condition(0, body.capacity, None);

    let pen = sqlx::query_as::<_, Pen>(&format!(
        r#"
        INSERT INTO pens (id, owner_id, name, capacity, pen_type, location, condition)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING {PEN_COLS}
        "#
    ))
    .bind(id)
    .bind(auth.user_id)
    .bind(&body.name)
    .bind(body.capacity)
    .bind(&body.pen_type)
    .bind(&body.location)
    .bind(&initial_condition)
    .fetch_one(&state.pool)
    .await
    .map_err(db_err)?;

    Ok(Json(to_pen_response(pen, 0)))
}

pub async fn list(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<PenResponse>>, AppError> {
    let pens = sqlx::query_as::<_, Pen>(&format!(
        "SELECT {PEN_COLS} FROM pens WHERE owner_id = $1 ORDER BY created_at DESC"
    ))
    .bind(auth.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(db_err)?;

    let mut response = Vec::with_capacity(pens.len());
    for pen in pens {
        // Refresh kondisi agar selalu mencerminkan okupansi & kesehatan terkini
        pen_condition::refresh_pen_condition(&state.pool, pen.id).await;
        let occupancy = pen_condition::count_occupancy(&state.pool, pen.id).await;
        let condition =
            pen_condition::generate_condition(occupancy, pen.capacity, pen_condition::avg_health(&state.pool, pen.id).await);
        let mut pr = to_pen_response(pen, occupancy);
        pr.condition = Some(condition);
        response.push(pr);
    }
    Ok(Json(response))
}

pub async fn update(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePenRequest>,
) -> Result<Json<PenResponse>, AppError> {
    let existing = sqlx::query_as::<_, Pen>(&format!(
        "SELECT {PEN_COLS} FROM pens WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(db_err)?
    .ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Kandang tidak ditemukan".to_string(),
        details: None,
    })?;

    if existing.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke kandang ini".to_string(),
            details: None,
        });
    }

    let name = body.name.unwrap_or(existing.name);
    let capacity = body.capacity.unwrap_or(existing.capacity);
    let pen_type = body.pen_type.unwrap_or(existing.pen_type);
    let location = body.location.or(existing.location);

    let pen = sqlx::query_as::<_, Pen>(&format!(
        r#"
        UPDATE pens SET name = $2, capacity = $3, pen_type = $4, location = $5
        WHERE id = $1
        RETURNING {PEN_COLS}
        "#
    ))
    .bind(id)
    .bind(&name)
    .bind(capacity)
    .bind(&pen_type)
    .bind(&location)
    .fetch_one(&state.pool)
    .await
    .map_err(db_err)?;

    pen_condition::refresh_pen_condition(&state.pool, id).await;
    let occupancy = pen_condition::count_occupancy(&state.pool, id).await;
    let condition = pen_condition::generate_condition(
        occupancy,
        pen.capacity,
        pen_condition::avg_health(&state.pool, id).await,
    );
    let mut pr = to_pen_response(pen, occupancy);
    pr.condition = Some(condition);
    Ok(Json(pr))
}

pub async fn delete(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    let existing = sqlx::query_as::<_, Pen>(&format!(
        "SELECT {PEN_COLS} FROM pens WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(db_err)?
    .ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Kandang tidak ditemukan".to_string(),
        details: None,
    })?;

    if existing.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke kandang ini".to_string(),
            details: None,
        });
    }

    // Lepas ternak dari kandang sebelum hapus (hindari FK error)
    let _ = sqlx::query("UPDATE livestock SET pen_id = NULL WHERE pen_id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    sqlx::query("DELETE FROM pens WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(db_err)?;

    Ok(Json(MessageResponse {
        message: "Kandang berhasil dihapus".to_string(),
    }))
}

/// GET /api/pens/:id/livestock — daftar ternak di dalam kandang
pub async fn list_livestock(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<LivestockResponse>>, AppError> {
    use crate::domain::entities::livestock::Livestock;
    let rows = sqlx::query_as::<_, Livestock>(
        r#"
        SELECT id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics, created_at, updated_at
        FROM livestock WHERE pen_id = $1 AND owner_id = $2
        "#,
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(db_err)?;

    let response: Vec<LivestockResponse> = rows
        .into_iter()
        .map(|l| LivestockResponse {
            id: l.id,
            owner_id: l.owner_id,
            tag_id: l.tag_id,
            category: l.category,
            breed: l.breed,
            weight: l.weight.to_f64().unwrap_or(0.0),
            gender: l.gender,
            health_status: l.health_status,
            health_score: l.health_score,
            pen_id: l.pen_id,
            age_months: l.age_months,
            entry_date: l.entry_date,
            created_at: l.created_at,
            updated_at: l.updated_at,
        })
        .collect();
    Ok(Json(response))
}
