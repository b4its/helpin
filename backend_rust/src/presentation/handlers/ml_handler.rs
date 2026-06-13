use axum::{extract::State, Json};
use validator::Validate;

use crate::application::ml::{evaluate_health, predict_feed};
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{FeedPredictionRequest, HealthEvaluationRequest};
use crate::presentation::dto::response::MlPredictionResponse;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

pub async fn predict_feed_handler(
    _auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<FeedPredictionRequest>,
) -> Result<Json<MlPredictionResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = predict_feed::PredictFeedInput {
        livestock_id: body.livestock_id,
        tag_id: body.tag_id,
        breed: body.breed,
        weight: body.weight,
    };

    let output = predict_feed::predict_feed(input, &state.ml_client, &state.pool).await?;

    Ok(Json(MlPredictionResponse {
        recommendation_id: output.recommendation_id,
        prediction: output.prediction,
    }))
}

pub async fn evaluate_health_handler(
    _auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<HealthEvaluationRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = evaluate_health::MlHealthInput {
        livestock_id: body.livestock_id,
        body_temp: body.body_temp,
        heart_rate: body.heart_rate,
        respiratory_rate: body.respiratory_rate,
        symptoms: body.symptoms,
        weight: body.weight,
    };

    let output =
        evaluate_health::evaluate_health_ml(input, &state.ml_client, &state.livestock_repo).await?;

    Ok(Json(serde_json::json!({
        "health_score": output.health_score,
        "evaluation": output.evaluation,
    })))
}

pub async fn get_feed_history(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let rows = sqlx::query_as::<_, (uuid::Uuid, Option<uuid::Uuid>, String, String, sqlx::types::BigDecimal, Option<String>, serde_json::Value, serde_json::Value, Option<chrono::DateTime<chrono::Utc>>)>(
        r#"
        SELECT id, livestock_id, tag_id, breed, weight, feed_name, nutrition, ingredients, created_at
        FROM feed_recommendations
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let response: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.0,
                "livestock_id": r.1,
                "tag_id": r.2,
                "breed": r.3,
                "weight": r.4.to_string(),
                "feed_name": r.5,
                "nutrition": r.6,
                "ingredients": r.7,
                "created_at": r.8,
            })
        })
        .collect();

    Ok(Json(response))
}
