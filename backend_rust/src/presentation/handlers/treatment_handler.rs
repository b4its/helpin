//! Handler untuk fitur "Tambah Informasi" (treatment) lahan + estimasi panen.

use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::application::farm::treatment::refresh_land_prediction;
use crate::domain::traits::repository::LandRepository;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct FertilizerItem {
    pub inventory_id: Option<Uuid>,
    pub name: String,
    #[serde(default)]
    pub quantity: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToolItem {
    pub inventory_id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTreatmentRequest {
    #[serde(default)]
    pub fertilizers: Vec<FertilizerItem>,
    #[serde(default)]
    pub tools: Vec<ToolItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PredictionView {
    pub has_prediction: bool,
    pub predicted_weight_kg: Option<f64>,
    pub quality_grade: Option<String>,
    pub predicted_harvest_date: Option<String>,
    pub plant_type: Option<String>,
    pub plant_date: Option<String>,
}

/// Verifikasi lahan ada & milik user yang login.
async fn ensure_owner(state: &AppState, land_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let land = state.land_repo.find_by_id(land_id).await?.ok_or_else(|| AppError {
        status: axum::http::StatusCode::NOT_FOUND,
        message: "Lahan tidak ditemukan".to_string(),
        details: None,
    })?;
    if land.owner_id != user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke lahan ini".to_string(),
            details: None,
        });
    }
    Ok(())
}

/// POST /api/lands/:id/treatment — tambah informasi treatment (pupuk + jumlah, alat).
/// Data disimpan ke MongoDB lalu prediksi panen dihitung ulang.
pub async fn add_treatment(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
    Json(body): Json<AddTreatmentRequest>,
) -> Result<Json<Value>, AppError> {
    ensure_owner(&state, land_id, auth.user_id).await?;

    // Bangun dokumen treatment fleksibel untuk MongoDB
    let fertilizers: Vec<Value> = body
        .fertilizers
        .iter()
        .map(|f| {
            json!({
                "inventory_id": f.inventory_id.map(|i| i.to_string()),
                "name": f.name,
                "quantity": f.quantity,
                "unit": f.unit,
            })
        })
        .collect();
    let tools: Vec<Value> = body
        .tools
        .iter()
        .map(|t| {
            json!({
                "inventory_id": t.inventory_id.map(|i| i.to_string()),
                "name": t.name,
            })
        })
        .collect();

    let treatment_doc = json!({
        "fertilizers": fertilizers,
        "tools": tools,
        "notes": body.notes,
        "source": "manual",
    });

    if !state.mongo.is_available() {
        return Err(AppError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            message: "Layanan MongoDB tidak tersedia untuk menyimpan treatment".to_string(),
            details: None,
        });
    }

    state
        .mongo
        .save_treatment(land_id, auth.user_id, &treatment_doc)
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Gagal menyimpan treatment: {}", e),
            details: None,
        })?;

    // Kurangi stok inventori untuk pupuk yang dipakai (best-effort)
    for f in &body.fertilizers {
        if let (Some(inv_id), q) = (f.inventory_id, f.quantity) {
            if q > 0.0 {
                let _ = sqlx::query(
                    "UPDATE inventories SET quantity = GREATEST(quantity - $2, 0) WHERE id = $1 AND owner_id = $3",
                )
                .bind(inv_id)
                .bind(bigdecimal::BigDecimal::try_from(q).unwrap_or_default())
                .bind(auth.user_id)
                .execute(&state.pool)
                .await;
            }
        }
    }

    // Hitung ulang prediksi panen
    let prediction = refresh_land_prediction(&state.pool, &state.mongo, land_id)
        .await
        .map_err(|e| AppError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Gagal menghitung prediksi: {}", e),
            details: None,
        })?;

    match prediction {
        Some(p) => Ok(Json(json!({
            "message": "Treatment tersimpan & prediksi diperbarui",
            "prediction": {
                "predicted_weight_kg": p.predicted_weight_kg,
                "quality_grade": p.quality_grade,
                "predicted_harvest_date": p.predicted_harvest_date.to_string(),
                "growth_days": p.growth_days,
                "feasibility_status": p.feasibility_status,
                "score": p.score,
            }
        }))),
        None => Ok(Json(json!({
            "message": "Treatment tersimpan. Lahan belum punya komoditas untuk diprediksi.",
            "prediction": Value::Null,
        }))),
    }
}

/// GET /api/lands/:id/treatments — daftar treatment dari MongoDB.
pub async fn list_treatments(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
) -> Result<Json<Vec<Value>>, AppError> {
    ensure_owner(&state, land_id, auth.user_id).await?;
    let items = state.mongo.list_treatments(land_id).await.map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Gagal membaca treatment: {}", e),
        details: None,
    })?;
    Ok(Json(items))
}

/// GET /api/lands/:id/prediction — estimasi panen lahan saat ini (dari estimasi tersimpan).
pub async fn get_prediction(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
) -> Result<Json<PredictionView>, AppError> {
    ensure_owner(&state, land_id, auth.user_id).await?;

    let row = sqlx::query_as::<
        _,
        (
            sqlx::types::BigDecimal,
            Option<String>,
            Option<chrono::NaiveDate>,
            String,
            chrono::NaiveDate,
        ),
    >(
        r#"
        SELECT h.quantity, h.quality_grade, h.predicted_harvest_date, p.plant_type, p.plant_date
        FROM harvests h
        JOIN plants p ON p.id = h.plant_id
        WHERE h.land_id = $1 AND h.is_estimate = TRUE
        ORDER BY h.updated_at DESC
        LIMIT 1
        "#,
    )
    .bind(land_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    match row {
        Some((qty, grade, pred_date, plant_type, plant_date)) => Ok(Json(PredictionView {
            has_prediction: true,
            predicted_weight_kg: qty.to_f64(),
            quality_grade: grade,
            predicted_harvest_date: pred_date.map(|d| d.to_string()),
            plant_type: Some(plant_type),
            plant_date: Some(plant_date.to_string()),
        })),
        None => Ok(Json(PredictionView {
            has_prediction: false,
            predicted_weight_kg: None,
            quality_grade: None,
            predicted_harvest_date: None,
            plant_type: None,
            plant_date: None,
        })),
    }
}
