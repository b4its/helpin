use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

#[derive(Debug, Serialize)]
pub struct HarvestPredictionResponse {
    pub land_id: Uuid,
    pub land_name: String,
    pub predicted_harvest_date: String,
    pub predicted_weight_kg: f64,
    pub quality_grade: String,
    pub feasibility_status: String,
    pub crop_name: String,
    pub details: Value,
    pub prediction_source: String, // "openrouter" or "xgboost"
}

/// POST /api/predict/harvest/:land_id - Predict harvest for a given land
/// Uses OpenRouter (Gemini Flash) if API key available + internet connected,
/// otherwise falls back to XGBoost ensemble via Python ML service
pub async fn predict_harvest(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
) -> Result<Json<HarvestPredictionResponse>, AppError> {
    use crate::domain::traits::repository::LandRepository;
    use bigdecimal::ToPrimitive;

    // Get land data
    let land = state.land_repo.find_by_id(land_id).await?.ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Lahan tidak ditemukan".to_string(),
        details: None,
    })?;

    // Get plants for this land
    let plants = sqlx::query_as::<_, crate::domain::entities::plant::Plant>(
        "SELECT id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at FROM plants WHERE land_id = $1 AND status IN ('Ditanam', 'Tumbuh') ORDER BY plant_date DESC LIMIT 1"
    )
    .bind(land_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let current_plant = plants.first().ok_or_else(|| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Tidak ada tanaman aktif di lahan ini untuk diprediksi".to_string(),
        details: None,
    })?;

    // Get inventory data for context
    let inventories = sqlx::query_as::<_, crate::domain::entities::inventory::Inventory>(
        "SELECT id, land_id, owner_id, category, name, quantity, unit, created_at FROM inventories WHERE land_id = $1"
    )
    .bind(land_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    // Get past harvests for this land (for average data)
    let past_harvests = sqlx::query_as::<_, crate::domain::entities::harvest::Harvest>(
        "SELECT id, plant_id, land_id, quantity, unit, quality_grade, progress_percent, harvested_at FROM harvests WHERE land_id = $1 ORDER BY harvested_at DESC LIMIT 10"
    )
    .bind(land_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let area = land.area_hectare.to_f64().unwrap_or(1.0);
    let crop_name = current_plant.plant_type.clone();

    // Build context for prediction
    let context = json!({
        "land_name": land.name,
        "area_hectare": area,
        "soil_type": land.soil_type,
        "crop_type": crop_name,
        "plant_date": current_plant.plant_date.to_string(),
        "estimated_harvest": current_plant.estimated_harvest.map(|d| d.to_string()),
        "inventories": inventories.iter().map(|i| json!({
            "name": i.name,
            "category": i.category,
            "quantity": i.quantity.to_f64().unwrap_or(0.0),
            "unit": i.unit
        })).collect::<Vec<_>>(),
        "historical_harvests": past_harvests.iter().map(|h| json!({
            "quantity": h.quantity.to_f64().unwrap_or(0.0),
            "unit": h.unit,
            "quality_grade": h.quality_grade,
            "harvested_at": h.harvested_at.map(|d| d.to_string())
        })).collect::<Vec<_>>()
    });

    // Try OpenRouter first (online mode)
    let openrouter_key = &state.config.openrouter_api_key;
    if !openrouter_key.is_empty() {
        match call_openrouter(openrouter_key, &context).await {
            Ok(prediction) => {
                return Ok(Json(HarvestPredictionResponse {
                    land_id,
                    land_name: land.name,
                    predicted_harvest_date: prediction.predicted_harvest_date,
                    predicted_weight_kg: prediction.predicted_weight_kg,
                    quality_grade: prediction.quality_grade,
                    feasibility_status: prediction.feasibility_status,
                    crop_name,
                    details: prediction.details,
                    prediction_source: "openrouter".to_string(),
                }));
            }
            Err(e) => {
                tracing::warn!("OpenRouter prediction failed (falling back to XGBoost): {}", e);
            }
        }
    }

    // Fallback: call Python ML service (XGBoost ensemble)
    let ml_input = json!({
        "land_name": land.name,
        "area_hectare": area,
        "soil_type": land.soil_type,
        "crop_type": crop_name,
        "plant_date": current_plant.plant_date.to_string(),
        "historical_avg_yield": if past_harvests.is_empty() {
            area * 5.0 // default estimation: 5 ton/ha
        } else {
            past_harvests.iter().map(|h| h.quantity.to_f64().unwrap_or(0.0)).sum::<f64>() / past_harvests.len() as f64
        },
        "fertilizer_count": inventories.iter().filter(|i| i.category == "Pupuk").count(),
        "seed_type": inventories.iter().find(|i| i.category == "Bibit").map(|i| i.name.clone()).unwrap_or_default()
    });

    let ml_url = format!("{}/predict/harvest", state.config.ml_service_url);
    let client = reqwest::Client::new();
    
    match client.post(&ml_url).json(&ml_input).send().await {
        Ok(response) if response.status().is_success() => {
            let result: Value = response.json().await.unwrap_or(json!({}));
            
            let predicted_weight = result["predicted_weight_kg"].as_f64().unwrap_or(area * 4.5);
            let quality = result["quality_grade"].as_str().unwrap_or("B").to_string();
            let feasibility = if predicted_weight > 0.0 { "Layak" } else { "Tidak Layak" };

            Ok(Json(HarvestPredictionResponse {
                land_id,
                land_name: land.name,
                predicted_harvest_date: result["predicted_harvest_date"].as_str()
                    .unwrap_or(&current_plant.estimated_harvest.map(|d| d.to_string()).unwrap_or_default())
                    .to_string(),
                predicted_weight_kg: predicted_weight,
                quality_grade: quality,
                feasibility_status: feasibility.to_string(),
                crop_name,
                details: result.get("details").cloned().unwrap_or(json!({
                    "method": "xgboost_ensemble",
                    "confidence": 0.85
                })),
                prediction_source: "xgboost".to_string(),
            }))
        }
        _ => {
            // Ultimate fallback: simple calculation based on area and historical data
            let avg_yield = if past_harvests.is_empty() {
                area * 4.5
            } else {
                past_harvests.iter().map(|h| h.quantity.to_f64().unwrap_or(0.0)).sum::<f64>() / past_harvests.len() as f64
            };

            Ok(Json(HarvestPredictionResponse {
                land_id,
                land_name: land.name,
                predicted_harvest_date: current_plant.estimated_harvest
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Belum ditentukan".to_string()),
                predicted_weight_kg: avg_yield,
                quality_grade: "B".to_string(),
                feasibility_status: "Layak".to_string(),
                crop_name,
                details: json!({
                    "method": "simple_average",
                    "confidence": 0.6,
                    "note": "ML service offline, menggunakan rata-rata historis"
                }),
                prediction_source: "fallback".to_string(),
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenRouterPrediction {
    predicted_harvest_date: String,
    predicted_weight_kg: f64,
    quality_grade: String,
    feasibility_status: String,
    details: Value,
}

async fn call_openrouter(api_key: &str, context: &Value) -> anyhow::Result<OpenRouterPrediction> {
    let client = reqwest::Client::new();

    let prompt = format!(
        r#"Kamu adalah ahli agronomi AI. Berdasarkan data lahan pertanian berikut, prediksi hasil panen:

Data Lahan:
{}

Berikan prediksi dalam format JSON SAJA (tanpa markdown, tanpa penjelasan lain):
{{
  "predicted_harvest_date": "YYYY-MM-DD",
  "predicted_weight_kg": <angka dalam kg>,
  "quality_grade": "A" atau "B" atau "C",
  "feasibility_status": "Layak" atau "Tidak Layak",
  "details": {{
    "confidence_score": <0.0-1.0>,
    "estimated_revenue_idr": <angka>,
    "recommended_actions": ["saran1", "saran2"],
    "risk_factors": ["risiko1"],
    "nutrient_status": "Optimal/Defisit/Berlebih",
    "pest_risk": "Rendah/Sedang/Tinggi"
  }}
}}"#,
        serde_json::to_string_pretty(context).unwrap_or_default()
    );

    let body = json!({
        "model": "google/gemini-2.0-flash-001",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": 0.3,
        "max_tokens": 1000
    });

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("OpenRouter API returned status: {}", response.status()));
    }

    let result: Value = response.json().await?;
    
    // Parse the AI response
    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No content in OpenRouter response"))?;

    // Try to parse JSON from the response (handle potential markdown wrapping)
    let json_str = if content.contains("```json") {
        content
            .split("```json")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(content)
    } else if content.contains("```") {
        content
            .split("```")
            .nth(1)
            .unwrap_or(content)
    } else {
        content
    };

    let parsed: Value = serde_json::from_str(json_str.trim())
        .map_err(|e| anyhow::anyhow!("Failed to parse AI response as JSON: {}", e))?;

    Ok(OpenRouterPrediction {
        predicted_harvest_date: parsed["predicted_harvest_date"].as_str().unwrap_or("").to_string(),
        predicted_weight_kg: parsed["predicted_weight_kg"].as_f64().unwrap_or(0.0),
        quality_grade: parsed["quality_grade"].as_str().unwrap_or("B").to_string(),
        feasibility_status: parsed["feasibility_status"].as_str().unwrap_or("Layak").to_string(),
        details: parsed.get("details").cloned().unwrap_or(json!({})),
    })
}
