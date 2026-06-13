use axum::Json;

use crate::presentation::dto::response::HealthCheckResponse;

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "ok".to_string(),
    })
}
