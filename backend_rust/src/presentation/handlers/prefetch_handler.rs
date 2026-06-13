use axum::{
    extract::{Query, State},
    Json,
};

use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::DeltaSyncParams;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

/// Returns a bundle of data for offline use
pub async fn prefetch(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    use crate::domain::traits::repository::{LandRepository, LivestockRepository, ProductRepository};

    let livestock = state.livestock_repo.find_all_by_owner(auth.user_id).await?;
    let lands = state.land_repo.find_all_by_owner(auth.user_id).await?;
    let products = state.product_repo.find_all().await?;

    Ok(Json(serde_json::json!({
        "livestock": livestock,
        "lands": lands,
        "products": products,
    })))
}

/// Returns delta data since a given timestamp
pub async fn delta_sync(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<DeltaSyncParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let since = params.since.unwrap_or_else(|| {
        chrono::Utc::now() - chrono::Duration::hours(24)
    });

    // Fetch products updated since the given timestamp
    let products = sqlx::query_as::<_, crate::domain::entities::product::Product>(
        r#"
        SELECT id, seller_id, category_id, name, price, stock, unit, product_type, location, image_url, description, created_at, updated_at
        FROM products WHERE updated_at > $1
        "#,
    )
    .bind(since)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    Ok(Json(serde_json::json!({
        "since": since,
        "products": products,
    })))
}
