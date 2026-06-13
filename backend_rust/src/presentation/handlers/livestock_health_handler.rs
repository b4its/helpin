//! Handler kondisi kesehatan ternak (generate detail + riwayat).

use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::application::livestock::health_generate::{generate_health, ProvidedVitals};
use crate::domain::traits::repository::LivestockRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

#[derive(Debug, Deserialize, Default)]
pub struct GenerateHealthRequest {
    pub heart_rate: Option<i32>,
    pub body_temp: Option<f64>,
    pub respiratory_rate: Option<i32>,
    pub symptoms: Option<String>,
    pub notes: Option<String>,
}

/// POST /api/livestock/:id/health — generate/catat kondisi kesehatan ternak.
/// Jika biometrik tidak dikirim, akan di-generate otomatis (online/offline).
pub async fn record_health(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    body: Option<Json<GenerateHealthRequest>>,
) -> Result<Json<Value>, AppError> {
    let body = body.map(|b| b.0).unwrap_or_default();
    let provided = ProvidedVitals {
        heart_rate: body.heart_rate,
        body_temp: body.body_temp,
        respiratory_rate: body.respiratory_rate,
        symptoms: body.symptoms,
        notes: body.notes,
    };

    let result = generate_health(
        &state.pool,
        &state.config,
        &state.livestock_repo,
        id,
        auth.user_id,
        provided,
    )
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: e.to_string(),
        details: None,
    })?;

    Ok(Json(result))
}

/// GET /api/livestock/:id/health-history — riwayat kondisi kesehatan ternak.
pub async fn health_history(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Value>>, AppError> {
    // Verifikasi kepemilikan
    let livestock = state.livestock_repo.find_by_id(id).await?.ok_or_else(|| AppError {
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

    let rows = sqlx::query_as::<_, crate::domain::entities::health_record::HealthRecord>(
        r#"
        SELECT id, livestock_id, symptoms, body_temp, heart_rate, respiratory_rate, notes, health_score, recorded_at
        FROM health_records WHERE livestock_id = $1 ORDER BY recorded_at DESC LIMIT 50
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let response: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            json!({
                "id": r.id,
                "livestock_id": r.livestock_id,
                "symptoms": r.symptoms,
                "body_temp": r.body_temp.and_then(|t| t.to_f64()),
                "heart_rate": r.heart_rate,
                "respiratory_rate": r.respiratory_rate,
                "notes": r.notes,
                "health_score": r.health_score,
                "recorded_at": r.recorded_at,
            })
        })
        .collect();

    Ok(Json(response))
}
