//! Handler analisa pakan per kandang.

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use crate::application::livestock::feed_analysis;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

async fn ensure_pen_owner(state: &AppState, pen_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let owner = sqlx::query_scalar::<_, Uuid>("SELECT owner_id FROM pens WHERE id = $1")
        .bind(pen_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Database error: {}", e),
            details: None,
        })?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "Kandang tidak ditemukan".to_string(),
            details: None,
        })?;
    if owner != user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke kandang ini".to_string(),
            details: None,
        });
    }
    Ok(())
}

/// POST /api/pens/:id/feed-analysis — analisa pakan untuk ternak di kandang.
pub async fn analyze(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(pen_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    ensure_pen_owner(&state, pen_id, auth.user_id).await?;
    let result = feed_analysis::analyze_pen_feed(&state.pool, &state.mongo, &state.config, pen_id, auth.user_id)
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: e.to_string(),
            details: None,
        })?;
    Ok(Json(result))
}

/// GET /api/pens/:id/feed-analysis — riwayat/last analisa pakan kandang.
pub async fn list_analysis(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(pen_id): Path<Uuid>,
) -> Result<Json<Vec<Value>>, AppError> {
    ensure_pen_owner(&state, pen_id, auth.user_id).await?;
    let items = state.mongo.list_docs("pen_feed_analysis", pen_id).await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Gagal membaca analisa pakan: {}", e),
        details: None,
    })?;
    Ok(Json(items))
}
