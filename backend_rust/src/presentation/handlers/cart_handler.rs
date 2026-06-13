use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::application::ecommerce::manage_cart;
use crate::domain::entities::cart::CartItem;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{AddToCartRequest, UpdateCartQuantityRequest};
use crate::presentation::dto::response::{CartItemResponse, MessageResponse};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_cart_response(c: CartItem) -> CartItemResponse {
    CartItemResponse {
        id: c.id,
        user_id: c.user_id,
        product_id: c.product_id,
        quantity: c.quantity,
        created_at: c.created_at,
    }
}

pub async fn add_to_cart(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<AddToCartRequest>,
) -> Result<Json<CartItemResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = manage_cart::AddToCartInput {
        user_id: auth.user_id,
        product_id: body.product_id,
        quantity: body.quantity,
    };

    let cart_item = manage_cart::add_to_cart(input, &state.product_repo, &state.pool).await?;
    Ok(Json(to_cart_response(cart_item)))
}

pub async fn get_cart(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItemResponse>>, AppError> {
    let items = manage_cart::get_cart(auth.user_id, &state.pool).await?;
    let response: Vec<CartItemResponse> = items.into_iter().map(to_cart_response).collect();
    Ok(Json(response))
}

pub async fn update_qty(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCartQuantityRequest>,
) -> Result<Json<CartItemResponse>, AppError> {
    let cart_item =
        manage_cart::update_cart_quantity(id, auth.user_id, body.quantity, &state.pool).await?;
    Ok(Json(to_cart_response(cart_item)))
}

pub async fn remove_item(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    manage_cart::remove_from_cart(id, auth.user_id, &state.pool).await?;
    Ok(Json(MessageResponse {
        message: "Item berhasil dihapus dari keranjang".to_string(),
    }))
}
