use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct AppError {
    pub status: StatusCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = json!({
            "error": {
                "code": self.status.as_u16(),
                "message": self.message,
                "details": self.details
            }
        });
        (self.status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        let msg = err.to_string();
        tracing::error!("Application error: {}", msg);
        
        let (status, message) = if msg.contains("tidak valid") || msg.contains("Invalid") || msg.contains("Kredensial") {
            (StatusCode::UNAUTHORIZED, msg)
        } else if msg.contains("not found") || msg.contains("tidak ditemukan") {
            (StatusCode::NOT_FOUND, msg)
        } else if msg.contains("already") || msg.contains("already registered") || msg.contains("Email already") {
            (StatusCode::CONFLICT, msg)
        } else if msg.contains("Stok") || msg.contains("insufficient") {
            (StatusCode::CONFLICT, msg)
        } else if msg.contains("Forbidden") || msg.contains("akses") {
            (StatusCode::FORBIDDEN, msg)
        } else if msg.contains("Validation") || msg.contains("harus") || msg.contains("required") {
            (StatusCode::UNPROCESSABLE_ENTITY, msg)
        } else {
            // In development, show actual error; in production this should be generic
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Server error: {}", msg))
        };
        AppError {
            status,
            message,
            details: None,
        }
    }
}
