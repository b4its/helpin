use axum::{extract::State, Json};
use validator::Validate;

use crate::application::sync::{enqueue_offline, process_queue};
use crate::domain::traits::repository::SyncQueueRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::EnqueueSyncRequest;
use crate::presentation::dto::response::{ProcessQueueResponse, SyncQueueResponse, SyncStatusResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

pub async fn enqueue(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<EnqueueSyncRequest>,
) -> Result<Json<SyncQueueResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = enqueue_offline::EnqueueInput {
        user_id: auth.user_id,
        payload: body.payload,
        signature: body.signature,
    };

    let item = enqueue_offline::enqueue_offline(
        input,
        &state.sync_queue_repo,
        &state.user_repo,
        &state.crypto_service,
    )
    .await?;

    Ok(Json(SyncQueueResponse {
        id: item.id,
        user_id: item.user_id,
        payload: item.payload,
        status: item.status,
        error_message: item.error_message,
        created_at: item.created_at,
        synced_at: item.synced_at,
    }))
}

pub async fn process_queue_handler(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ProcessQueueResponse>, AppError> {
    let output = process_queue::process_queue(
        &state.sync_queue_repo,
        &state.user_repo,
        &state.crypto_service,
        &state.blockchain_service,
    )
    .await?;

    Ok(Json(ProcessQueueResponse {
        processed: output.processed,
        succeeded: output.succeeded,
        failed: output.failed,
    }))
}

pub async fn get_status(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<SyncStatusResponse>, AppError> {
    let pending = state.sync_queue_repo.get_pending().await?;
    let items: Vec<SyncQueueResponse> = pending
        .into_iter()
        .filter(|item| item.user_id == auth.user_id)
        .map(|item| SyncQueueResponse {
            id: item.id,
            user_id: item.user_id,
            payload: item.payload,
            status: item.status,
            error_message: item.error_message,
            created_at: item.created_at,
            synced_at: item.synced_at,
        })
        .collect();

    let count = items.len();
    Ok(Json(SyncStatusResponse {
        pending: count,
        items,
    }))
}
