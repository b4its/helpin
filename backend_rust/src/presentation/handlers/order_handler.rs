use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::application::ecommerce::create_order;
use crate::domain::entities::order::Order;
use crate::domain::traits::repository::OrderRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::CheckoutRequest;
use crate::presentation::dto::response::OrderResponse;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_order_response(o: Order) -> OrderResponse {
    OrderResponse {
        id: o.id,
        buyer_id: o.buyer_id,
        status: o.status,
        total_amount: o.total_amount,
        shipping_address: o.shipping_address,
        created_at: o.created_at,
        updated_at: o.updated_at,
    }
}

pub async fn checkout(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CheckoutRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = create_order::CreateOrderInput {
        buyer_id: auth.user_id,
        shipping_address: body.shipping_address,
    };

    let output =
        create_order::create_order(input, &state.order_repo, &state.product_repo, &state.pool)
            .await?;

    Ok(Json(to_order_response(output.order)))
}

pub async fn list_orders(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    let orders = state.order_repo.find_by_buyer(auth.user_id).await?;
    let response: Vec<OrderResponse> = orders.into_iter().map(to_order_response).collect();
    Ok(Json(response))
}

pub async fn get_order(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = state
        .order_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "Order tidak ditemukan".to_string(),
            details: None,
        })?;

    if order.buyer_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke order ini".to_string(),
            details: None,
        });
    }

    Ok(Json(to_order_response(order)))
}
