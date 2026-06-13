use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::ToPrimitive;
use uuid::Uuid;
use validator::Validate;

use crate::application::farm::{create_land, list_land, manage_plant};
use crate::domain::entities::land::Land;
use crate::infrastructure::middleware::auth_middleware::AuthUser;
use crate::presentation::dto::request::{
    CreateLandRequest, CreatePlantRequest, UpdateLandRequest, UpdatePlantStatusRequest,
};
use crate::presentation::dto::response::{
    HarvestResponse, LandResponse, MessageResponse, PlantResponse,
};
use crate::presentation::error::AppError;
use crate::presentation::routes::AppState;

fn to_land_response(l: Land) -> LandResponse {
    LandResponse {
        id: l.id,
        owner_id: l.owner_id,
        code: l.code,
        name: l.name,
        area_hectare: l.area_hectare.to_f64().unwrap_or(0.0),
        soil_type: l.soil_type,
        status: l.status,
        crop_type: l.crop_type,
        location: l.location,
        created_at: l.created_at,
        updated_at: l.updated_at,
    }
}

pub async fn create_land_handler(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateLandRequest>,
) -> Result<Json<LandResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = create_land::CreateLandInput {
        owner_id: auth.user_id,
        name: body.name,
        area_hectare: body.area_hectare,
        soil_type: body.soil_type,
        status: body.status,
        crop_type: body.crop_type,
        location: body.location,
        inventory_ids: body.inventory_ids,
    };

    let land = create_land::create_land(input, &state.land_repo, &state.pool, &state.mongo).await?;
    Ok(Json(to_land_response(land)))
}

pub async fn list_lands(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<LandResponse>>, AppError> {
    let lands = list_land::list_land(auth.user_id, &state.land_repo).await?;
    let response: Vec<LandResponse> = lands.into_iter().map(to_land_response).collect();
    Ok(Json(response))
}

pub async fn update_land(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLandRequest>,
) -> Result<Json<LandResponse>, AppError> {
    use crate::domain::traits::repository::LandRepository;

    let mut land = state
        .land_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "Lahan tidak ditemukan".to_string(),
            details: None,
        })?;

    if land.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke lahan ini".to_string(),
            details: None,
        });
    }

    if let Some(name) = body.name {
        land.name = name;
    }
    if let Some(area) = body.area_hectare {
        land.area_hectare = bigdecimal::BigDecimal::try_from(area).map_err(|e| AppError {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: format!("Invalid area: {}", e),
            details: None,
        })?;
    }
    if let Some(soil_type) = body.soil_type {
        land.soil_type = soil_type;
    }
    if let Some(status) = body.status {
        land.status = status;
    }
    if let Some(crop_type) = body.crop_type {
        land.crop_type = Some(crop_type);
    }
    if let Some(location) = body.location {
        land.location = Some(location);
    }

    let updated = state.land_repo.update(&land).await?;

    // Update land_inventories jika inventory_ids dikirim
    if let Some(inv_ids) = body.inventory_ids {
        // Hapus yang lama dulu
        let _ = sqlx::query("DELETE FROM land_inventories WHERE land_id = $1")
            .bind(id)
            .execute(&state.pool)
            .await;
        // Insert yang baru
        for inv_id in inv_ids {
            let _ = sqlx::query(
                "INSERT INTO land_inventories (id, land_id, inventory_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
            )
            .bind(uuid::Uuid::new_v4())
            .bind(id)
            .bind(inv_id)
            .execute(&state.pool)
            .await;
        }
    }

    // Hitung ulang estimasi panen karena treatment/komoditas bisa berubah (best-effort)
    let _ = crate::application::farm::treatment::refresh_land_prediction(
        &state.pool,
        &state.mongo,
        id,
    )
    .await;

    Ok(Json(to_land_response(updated)))
}

pub async fn delete_land(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    use crate::domain::traits::repository::LandRepository;

    let land = state
        .land_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError {
            status: axum::http::StatusCode::NOT_FOUND,
            message: "Lahan tidak ditemukan".to_string(),
            details: None,
        })?;

    if land.owner_id != auth.user_id {
        return Err(AppError {
            status: axum::http::StatusCode::FORBIDDEN,
            message: "Tidak memiliki akses ke lahan ini".to_string(),
            details: None,
        });
    }

    state.land_repo.delete(id).await?;
    Ok(Json(MessageResponse {
        message: "Lahan berhasil dihapus".to_string(),
    }))
}

pub async fn create_plant(
    _auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreatePlantRequest>,
) -> Result<Json<PlantResponse>, AppError> {
    body.validate().map_err(|e| AppError {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: format!("Validation error: {}", e),
        details: None,
    })?;

    let input = manage_plant::CreatePlantInput {
        land_id: body.land_id,
        plant_type: body.plant_type,
        plant_date: body.plant_date,
        estimated_harvest: body.estimated_harvest,
    };

    let plant = manage_plant::create_plant(input, &state.pool).await?;
    Ok(Json(PlantResponse {
        id: plant.id,
        land_id: plant.land_id,
        plant_type: plant.plant_type,
        plant_date: plant.plant_date,
        estimated_harvest: plant.estimated_harvest,
        actual_harvest: plant.actual_harvest,
        status: plant.status,
        created_at: plant.created_at,
    }))
}

pub async fn update_plant_status(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePlantStatusRequest>,
) -> Result<Json<PlantResponse>, AppError> {
    let plant = manage_plant::update_plant_status(id, &body.status, &state.pool).await?;
    Ok(Json(PlantResponse {
        id: plant.id,
        land_id: plant.land_id,
        plant_type: plant.plant_type,
        plant_date: plant.plant_date,
        estimated_harvest: plant.estimated_harvest,
        actual_harvest: plant.actual_harvest,
        status: plant.status,
        created_at: plant.created_at,
    }))
}

pub async fn list_harvests(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(land_id): Path<Uuid>,
) -> Result<Json<Vec<HarvestResponse>>, AppError> {
    use crate::domain::entities::harvest::Harvest;

    let harvests = sqlx::query_as::<_, Harvest>(
        r#"
        SELECT id, plant_id, land_id, quantity, unit, quality_grade, progress_percent, harvested_at
        FROM harvests WHERE land_id = $1
        ORDER BY harvested_at DESC
        "#,
    )
    .bind(land_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        message: format!("Database error: {}", e),
        details: None,
    })?;

    let response: Vec<HarvestResponse> = harvests
        .into_iter()
        .map(|h| HarvestResponse {
            id: h.id,
            plant_id: h.plant_id,
            land_id: h.land_id,
            quantity: h.quantity.to_f64().unwrap_or(0.0),
            unit: h.unit,
            quality_grade: h.quality_grade,
            progress_percent: h.progress_percent,
            harvested_at: h.harvested_at,
        })
        .collect();

    Ok(Json(response))
}
