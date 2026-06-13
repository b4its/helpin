use axum::{
    extract::{Path, Query, State},
    Json,
};
use bigdecimal::ToPrimitive;
use uuid::Uuid;
use validator::Validate;

use crate::application::livestock::{create_livestock, delete_livestock, list_livestock, update_livestock};
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{
    CreateLivestockRequest, LivestockFilterParams, UpdateLivestockRequest,
};
use crate::presentation::dto::response::{LivestockResponse, MessageResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_livestock_response(l: crate::domain::entities::livestock::Livestock) -> LivestockResponse {
    LivestockResponse {
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
    }
}

pub async fn create(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateLivestockRequest>,
) -> Result<Json<LivestockResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = create_livestock::CreateLivestockInput {
        owner_id: auth.user_id,
        tag_id: body.tag_id,
        category: body.category,
        breed: body.breed,
        weight: body.weight,
        gender: body.gender,
        age_months: body.age_months,
        pen_id: body.pen_id,
    };

    let livestock = create_livestock::create_livestock(input, &state.livestock_repo).await?;
    // Perbarui kondisi kandang otomatis jika ternak langsung dimasukkan ke kandang
    if let Some(pen_id) = livestock.pen_id {
        crate::application::livestock::pen_condition::refresh_pen_condition(&state.pool, pen_id).await;
    }
    Ok(Json(to_livestock_response(livestock)))
}

pub async fn list(
    auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<LivestockFilterParams>,
) -> Result<Json<Vec<LivestockResponse>>, AppError> {
    let input = list_livestock::ListLivestockInput {
        owner_id: auth.user_id,
        category: params.category,
        search: params.search,
    };

    let items = list_livestock::list_livestock(input, &state.livestock_repo).await?;
    let response: Vec<LivestockResponse> = items.into_iter().map(to_livestock_response).collect();
    Ok(Json(response))
}

pub async fn get_by_id(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<LivestockResponse>, AppError> {
    use crate::domain::traits::repository::LivestockRepository;
    let livestock = state
        .livestock_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "Ternak tidak ditemukan".to_string(),
            details: None,
        })?;

    if livestock.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke ternak ini".to_string(),
            details: None,
        });
    }

    Ok(Json(to_livestock_response(livestock)))
}

pub async fn update(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLivestockRequest>,
) -> Result<Json<LivestockResponse>, AppError> {
    let input = update_livestock::UpdateLivestockInput {
        id,
        owner_id: auth.user_id,
        tag_id: body.tag_id,
        category: body.category,
        breed: body.breed,
        weight: body.weight,
        gender: body.gender,
        health_status: body.health_status,
        pen_id: body.pen_id,
        age_months: body.age_months,
    };

    let livestock = update_livestock::update_livestock(input, &state.livestock_repo).await?;
    // Perbarui kondisi kandang otomatis (okupansi bisa berubah)
    if let Some(pen_id) = livestock.pen_id {
        crate::application::livestock::pen_condition::refresh_pen_condition(&state.pool, pen_id).await;
    }
    Ok(Json(to_livestock_response(livestock)))
}

pub async fn delete(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    delete_livestock::delete_livestock(id, auth.user_id, &state.livestock_repo).await?;
    Ok(Json(MessageResponse {
        message: "Ternak berhasil dihapus".to_string(),
    }))
}
