use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::infrastructure::services::ml_client::MlClientService;
use crate::domain::traits::service::MlService;
use sqlx::PgPool;

pub struct PredictFeedInput {
    pub livestock_id: Option<Uuid>,
    pub tag_id: String,
    pub breed: String,
    pub weight: f64,
}

pub struct PredictFeedOutput {
    pub prediction: Value,
    pub recommendation_id: Uuid,
}

pub async fn predict_feed(
    input: PredictFeedInput,
    ml_service: &MlClientService,
    pool: &PgPool,
) -> Result<PredictFeedOutput> {
    // Forward request to ML service
    let ml_input = json!({
        "tag_id": input.tag_id,
        "breed": input.breed,
        "weight": input.weight,
    });

    let prediction = ml_service.predict_feed(ml_input).await
        .map_err(|e| anyhow!("ML service error: {}", e))?;

    // Extract prediction data for storage
    let feed_name = prediction.get("feed_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let efficiency = prediction.get("efficiency")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let digestibility = prediction.get("digestibility")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let gain_prediction = prediction.get("gain_prediction")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let nutrition = prediction.get("nutrition").cloned().unwrap_or(json!({}));
    let ingredients = prediction.get("ingredients").cloned().unwrap_or(json!([]));

    let weight_bd = bigdecimal::BigDecimal::try_from(input.weight)
        .map_err(|e| anyhow!("Invalid weight: {}", e))?;
    let efficiency_bd = bigdecimal::BigDecimal::try_from(efficiency)
        .map_err(|e| anyhow!("Invalid efficiency: {}", e))?;
    let digestibility_bd = bigdecimal::BigDecimal::try_from(digestibility)
        .map_err(|e| anyhow!("Invalid digestibility: {}", e))?;
    let gain_bd = bigdecimal::BigDecimal::try_from(gain_prediction)
        .map_err(|e| anyhow!("Invalid gain: {}", e))?;

    // Save result to feed_recommendations table
    let rec_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO feed_recommendations (id, livestock_id, tag_id, breed, weight, feed_name, efficiency, digestibility, gain_prediction, nutrition, ingredients)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(rec_id)
    .bind(input.livestock_id)
    .bind(&input.tag_id)
    .bind(&input.breed)
    .bind(&weight_bd)
    .bind(feed_name)
    .bind(&efficiency_bd)
    .bind(&digestibility_bd)
    .bind(&gain_bd)
    .bind(&nutrition)
    .bind(&ingredients)
    .execute(pool)
    .await?;

    Ok(PredictFeedOutput {
        prediction,
        recommendation_id: rec_id,
    })
}
