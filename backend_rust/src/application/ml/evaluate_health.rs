use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::infrastructure::services::ml_client::MlClientService;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::service::MlService;
use crate::domain::traits::repository::LivestockRepository;

pub struct MlHealthInput {
    pub livestock_id: Uuid,
    pub body_temp: Option<f64>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub symptoms: Option<String>,
    pub weight: Option<f64>,
}

pub struct MlHealthOutput {
    pub evaluation: Value,
    pub health_score: i32,
}

pub async fn evaluate_health_ml(
    input: MlHealthInput,
    ml_service: &MlClientService,
    livestock_repo: &PostgresLivestockRepository,
) -> Result<MlHealthOutput> {
    // Forward biometric data to ML service
    let ml_input = json!({
        "livestock_id": input.livestock_id.to_string(),
        "body_temp": input.body_temp,
        "heart_rate": input.heart_rate,
        "respiratory_rate": input.respiratory_rate,
        "symptoms": input.symptoms,
        "weight": input.weight,
    });

    let evaluation = ml_service.evaluate_health(ml_input).await
        .map_err(|e| anyhow!("ML service error: {}", e))?;

    // Extract health score from ML response
    let health_score = evaluation.get("health_score")
        .and_then(|v| v.as_i64())
        .unwrap_or(50) as i32;

    // Update livestock health_score
    let mut livestock = livestock_repo.find_by_id(input.livestock_id).await?
        .ok_or_else(|| anyhow!("Ternak tidak ditemukan"))?;

    livestock.health_score = Some(health_score);
    livestock.health_status = Some(if health_score < 50 {
        "Sakit".to_string()
    } else if health_score < 75 {
        "Perlu Perhatian".to_string()
    } else {
        "Sehat".to_string()
    });

    livestock_repo.update(&livestock).await?;

    Ok(MlHealthOutput {
        evaluation,
        health_score,
    })
}
