use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::json;
use validator::Validate;

use crate::application::admin::activity::{self, ActivityCtx};
use crate::application::finance::manage_kas;
use crate::domain::entities::financial_record::FinancialRecord;
use crate::domain::traits::repository::UserRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::infrastructure::middleware::client_meta::ClientMeta;
use crate::presentation::dto::request::{FinanceReportParams, RecordExpenseRequest};
use crate::presentation::dto::response::{BalanceResponse, FinancialRecordResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

async fn actor_name(state: &AppState, id: uuid::Uuid) -> String {
    state
        .user_repo
        .find_by_id(id)
        .await
        .ok()
        .flatten()
        .map(|u| u.name)
        .unwrap_or_else(|| "Unknown".to_string())
}

fn to_financial_response(r: FinancialRecord) -> FinancialRecordResponse {
    FinancialRecordResponse {
        id: r.id,
        record_type: r.record_type,
        amount: r.amount,
        category: r.category,
        description: r.description,
        reference_id: r.reference_id,
        balance_after: r.balance_after,
        recorded_at: r.recorded_at,
    }
}

pub async fn get_balance(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, AppError> {
    let balance = manage_kas::get_balance(&state.financial_repo).await?;
    Ok(Json(BalanceResponse { balance }))
}

pub async fn record_expense(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<RecordExpenseRequest>,
) -> Result<Json<FinancialRecordResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = manage_kas::RecordExpenseInput {
        amount: body.amount,
        category: body.category.clone(),
        description: body.description.clone(),
    };

    let record = manage_kas::record_expense(input, &state.financial_repo).await?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Pengeluaran Kas".to_string(),
        action_type: "CREATED".to_string(),
        table_affected: "financial_records".to_string(),
        description: format!(
            "Uang keluar {} ({})",
            record.amount,
            record.category.clone().unwrap_or_else(|| "Manual".to_string())
        ),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "id": record.id, "type": "pengeluaran", "amount": record.amount, "category": record.category, "balance_after": record.balance_after }),
    }).await;

    Ok(Json(to_financial_response(record)))
}

pub async fn record_income(
    auth: AuthUser,
    meta: ClientMeta,
    State(state): State<AppState>,
    Json(body): Json<RecordExpenseRequest>,
) -> Result<Json<FinancialRecordResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = manage_kas::RecordExpenseInput {
        amount: body.amount,
        category: body.category.clone(),
        description: body.description.clone(),
    };

    let record = manage_kas::record_income(input, &state.financial_repo).await?;

    let actor = actor_name(&state, auth.user_id).await;
    activity::record(&state.mongo, &state.blockchain_service, &meta, ActivityCtx {
        title: "Pemasukan Kas".to_string(),
        action_type: "CREATED".to_string(),
        table_affected: "financial_records".to_string(),
        description: format!(
            "Uang masuk {} ({})",
            record.amount,
            record.category.clone().unwrap_or_else(|| "Manual".to_string())
        ),
        username: actor,
        old_data: json!({}),
        new_data: json!({ "id": record.id, "type": "pemasukan", "amount": record.amount, "category": record.category, "balance_after": record.balance_after }),
    }).await;

    Ok(Json(to_financial_response(record)))
}

pub async fn get_report(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<FinanceReportParams>,
) -> Result<Json<Vec<FinancialRecordResponse>>, AppError> {
    let start = params
        .start
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30));
    let end = params.end.unwrap_or_else(chrono::Utc::now);

    let records =
        manage_kas::get_report_by_period(start, end, &state.financial_repo).await?;
    let response: Vec<FinancialRecordResponse> =
        records.into_iter().map(to_financial_response).collect();
    Ok(Json(response))
}
