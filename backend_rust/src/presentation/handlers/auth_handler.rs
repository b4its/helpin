use axum::{extract::State, Json};
use validator::Validate;

use crate::application::auth::{login, refresh_token, register};
use crate::domain::traits::repository::UserRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::presentation::dto::response::{AuthResponse, TokenResponse, UserResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = register::RegisterInput {
        name: body.name,
        email: body.email,
        password: body.password,
        role: body.role,
    };

    let output = register::register(input, &state.user_repo, &state.jwt_service, &state.crypto_service).await?;

    Ok(Json(AuthResponse {
        user_id: output.user_id,
        access_token: output.access_token,
        refresh_token: output.refresh_token,
    }))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = login::LoginInput {
        email: body.email,
        password: body.password,
    };

    let output = login::login(
        input,
        &state.user_repo,
        &state.jwt_service,
        state.config.jwt_expiry_minutes,
        state.config.refresh_token_expiry_days,
    )
    .await?;

    Ok(Json(AuthResponse {
        user_id: output.user_id,
        access_token: output.access_token,
        refresh_token: output.refresh_token,
    }))
}

pub async fn refresh_handler(
    State(state): State<AppState>,
    Json(body): Json<RefreshTokenRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let output = refresh_token::refresh_token(
        &body.refresh_token,
        &state.jwt_service,
        &state.user_repo,
        state.config.jwt_expiry_minutes,
    )
    .await?;

    Ok(Json(TokenResponse {
        access_token: output.access_token,
    }))
}

pub async fn get_me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state
        .user_repo
        .find_by_id(auth.user_id)
        .await?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "User not found".to_string(),
            details: None,
        })?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
        created_at: user.created_at,
    }))
}
