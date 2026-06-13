//! blockchain_handler.rs
//!
//! Endpoint Rust yang menjadi jembatan antara frontend Nuxt dan Blockchain API.
//! Frontend tidak perlu tahu URL Blockchain API — cukup hit Rust backend.
//!
//! Routes:
//!   GET  /api/blockchain/status
//!   GET  /api/blockchain/stats
//!   GET  /api/blockchain/tx/:hash
//!   GET  /api/blockchain/explorer/search/:query
//!   GET  /api/blockchain/explorer/hash/:txHash/trace
//!   GET  /api/blockchain/explorer/recent
//!   GET  /api/blockchain/explorer/activities/recent
//!   GET  /api/blockchain/activity/:id
//!   POST /api/blockchain/activity/verify

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::Value;

use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

// ── Helper: proxy GET ke Blockchain API ──────────────────────────
async fn proxy_get(state: &AppState, path: &str) -> Result<Json<Value>, AppError> {
    let base = blockchain_api_base(&state.config.blockchain_rpc_url);
    let url  = format!("{}{}", base, path);

    let resp = state
        .blockchain_service
        .get_http_client()
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            message: format!("Blockchain API tidak tersedia: {}", e),
            details: None,
        })?;

    let status = resp.status();
    let body: Value = resp.json().await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Parse response gagal: {}", e),
        details: None,
    })?;

    if status.is_client_error() {
        return Err(AppError {
            status: axum::http::StatusCode::from_u16(status.as_u16()).unwrap_or(axum::http::StatusCode::BAD_REQUEST),
            message: body["error"].as_str().unwrap_or("Error").to_string(),
            details: Some(body),
        });
    }

    Ok(Json(body))
}

// Helper: blockchain API base URL
fn blockchain_api_base(rpc_url: &str) -> String {
    if let Ok(url) = std::env::var("BLOCKCHAIN_API_URL") {
        return url.trim_end_matches('/').to_string();
    }
    rpc_url.replace(":8545", ":3001").trim_end_matches('/').to_string()
}

// ── GET /api/blockchain/status ────────────────────────────────────
pub async fn status(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    proxy_get(&state, "/api/blockchain/status").await
}

// ── GET /api/blockchain/stats ─────────────────────────────────────
pub async fn stats(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    proxy_get(&state, "/api/blockchain/stats").await
}

// ── GET /api/blockchain/tx/:hash ──────────────────────────────────
pub async fn get_tx(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    proxy_get(&state, &format!("/api/blockchain/tx/{}", hash)).await
}

// ── GET /api/blockchain/block/:number ─────────────────────────────
pub async fn get_block(
    State(state): State<AppState>,
    Path(number): Path<String>,
) -> Result<Json<Value>, AppError> {
    proxy_get(&state, &format!("/api/blockchain/block/{}", number)).await
}

// ── GET /api/blockchain/explorer/search/:query ────────────────────
pub async fn explorer_search(
    State(state): State<AppState>,
    Path(query): Path<String>,
) -> Result<Json<Value>, AppError> {
    proxy_get(&state, &format!("/api/blockchain/explorer/search/{}", query)).await
}

// ── GET /api/blockchain/explorer/hash/:txHash/trace ───────────────
pub async fn explorer_trace(
    State(state): State<AppState>,
    Path(tx_hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    proxy_get(&state, &format!("/api/blockchain/explorer/hash/{}/trace", tx_hash)).await
}

// ── GET /api/blockchain/explorer/recent ───────────────────────────
#[derive(Deserialize)]
pub struct LimitQuery {
    pub limit: Option<u32>,
}

pub async fn explorer_recent(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Value>, AppError> {
    let path = match params.limit {
        Some(n) => format!("/api/blockchain/explorer/recent?limit={}", n),
        None    => "/api/blockchain/explorer/recent".to_string(),
    };
    proxy_get(&state, &path).await
}

// ── GET /api/blockchain/explorer/activities/recent ────────────────
pub async fn explorer_activities_recent(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Value>, AppError> {
    let path = match params.limit {
        Some(n) => format!("/api/blockchain/explorer/activities/recent?limit={}", n),
        None    => "/api/blockchain/explorer/activities/recent".to_string(),
    };
    proxy_get(&state, &path).await
}

// ── GET /api/blockchain/activity/:id ──────────────────────────────
pub async fn get_activity(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    proxy_get(&state, &format!("/api/blockchain/activity/{}", id)).await
}

// ── POST /api/blockchain/activity/verify ─────────────────────────
pub async fn verify_activity(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let base = blockchain_api_base(&state.config.blockchain_rpc_url);
    let url  = format!("{}/api/blockchain/activity/verify", base);

    let resp = state
        .blockchain_service
        .get_http_client()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            message: format!("Blockchain API tidak tersedia: {}", e),
            details: None,
        })?;

    let result: Value = resp.json().await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Parse response gagal: {}", e),
        details: None,
    })?;

    Ok(Json(result))
}
