use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::infrastructure::services::jwt_service::JwtService;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: String,
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the JWT service from extensions
        let jwt_service = parts
            .extensions
            .get::<JwtService>()
            .ok_or(AuthError::InvalidToken(
                "JWT service not configured".to_string(),
            ))?
            .clone();

        // Extract Bearer token from Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::MissingToken)?;

        // Validate the token
        let claims = jwt_service
            .validate_token(token)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AuthError::InvalidToken(format!("Invalid user ID in token: {}", e)))?;

        Ok(AuthUser {
            user_id,
            role: claims.role,
        })
    }
}
