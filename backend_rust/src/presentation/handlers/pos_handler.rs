use axum::{extract::State, Json};
use serde_json::json;
use validator::Validate;

use crate::application::admin::activity::{self, ActivityCtx};
use crate::application::pos::process_transaction;
use crate::domain::entities::transaction::PosTransaction;
use crate::domain::traits::repository::UserRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::infrastructure::middleware::client_meta::ClientMeta;
use crate::presentation::dto::request::PosTransactionRequest;
use crate::presentation::dto::response::PosTransactionResponse;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_pos_response(t: PosTransaction) -> PosTransactionResponse {
    PosTransactionResponse {
        id: t.id,
        cashier_id: t.cashier_id,
        items: t.items,
        subtotal: t.subtotal,
        tax: t.tax,
        total: t.total,
        amount_tendered: t.amount_tendered,
        change_amount: t.change_amount,
        created_at: t.created_at,
    }
}

pub async fn create_transaction(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<PosTransactionRequest>,
) -> Result<Json<PosTransactionResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let items: Vec<process_transaction::PosItem> = body
        .items
        .into_iter()
        .map(|i| process_transaction::PosItem {
            product_id: i.product_id,
            name: String::new(), // Name will be resolved from DB if needed
            quantity: i.quantity,
            price: i.price,
        })
        .collect();

    let input = process_transaction::ProcessTransactionInput {
        cashier_id: auth.user_id,
        items,
        amount_tendered: body.amount_tendered,
    };

    let output = process_transaction::process_transaction(
        input,
        &state.transaction_repo,
        &state.financial_repo,
        &state.pool,
    )
    .await?;

    // Activity log: pembayaran kasir (otomatis tercatat sebagai pemasukan kas)
    let actor = state.user_repo.find_by_id(auth.user_id).await.ok().flatten().map(|u| u.name).unwrap_or_else(|| "Kasir".to_string());
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Transaksi Kasir".to_string(),
        action_type: "PAYMENT".to_string(),
        table_affected: "pos_transactions".to_string(),
        description: format!("Pembayaran kasir total {} (tunai {})", output.transaction.total, output.transaction.amount_tendered),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "transaction_id": output.transaction.id, "total": output.transaction.total, "income_recorded": output.financial_record.amount }),
    }).await;

    Ok(Json(to_pos_response(output.transaction)))
}

pub async fn list_transactions(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<PosTransactionResponse>>, AppError> {
    let transactions = state.transaction_repo.find_by_cashier(auth.user_id).await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let response: Vec<PosTransactionResponse> =
        transactions.into_iter().map(to_pos_response).collect();
    Ok(Json(response))
}
