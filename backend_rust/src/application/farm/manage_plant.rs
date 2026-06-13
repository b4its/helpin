use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::entities::plant::Plant;
use sqlx::PgPool;

pub struct CreatePlantInput {
    pub land_id: Uuid,
    pub plant_type: String,
    pub plant_date: NaiveDate,
    pub estimated_harvest: Option<NaiveDate>,
}

pub async fn create_plant(
    input: CreatePlantInput,
    pool: &PgPool,
) -> Result<Plant> {
    let id = Uuid::new_v4();

    let plant = sqlx::query_as::<_, Plant>(
        r#"
        INSERT INTO plants (id, land_id, plant_type, plant_date, estimated_harvest, status)
        VALUES ($1, $2, $3, $4, $5, 'Ditanam')
        RETURNING id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at
        "#,
    )
    .bind(id)
    .bind(input.land_id)
    .bind(&input.plant_type)
    .bind(input.plant_date)
    .bind(input.estimated_harvest)
    .fetch_one(pool)
    .await?;

    Ok(plant)
}

pub async fn update_plant_status(
    plant_id: Uuid,
    new_status: &str,
    pool: &PgPool,
) -> Result<Plant> {
    // Validate status transition
    let valid_statuses = ["Ditanam", "Tumbuh", "Panen", "Selesai"];
    if !valid_statuses.contains(&new_status) {
        return Err(anyhow!(
            "Status tidak valid. Harus salah satu dari: Ditanam, Tumbuh, Panen, Selesai"
        ));
    }

    // Get current plant
    let current = sqlx::query_as::<_, Plant>(
        "SELECT id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at FROM plants WHERE id = $1"
    )
    .bind(plant_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Tanaman tidak ditemukan"))?;

    // Validate valid status transitions: Ditanam -> Tumbuh -> Panen -> Selesai
    let current_status = current.status.as_deref().unwrap_or("Ditanam");
    let valid_transition = match current_status {
        "Ditanam" => new_status == "Tumbuh",
        "Tumbuh" => new_status == "Panen",
        "Panen" => new_status == "Selesai",
        _ => false,
    };

    if !valid_transition {
        return Err(anyhow!(
            "Transisi status tidak valid: {} -> {}",
            current_status,
            new_status
        ));
    }

    // Update actual_harvest date when status is Panen
    let plant = if new_status == "Panen" {
        sqlx::query_as::<_, Plant>(
            r#"
            UPDATE plants SET status = $2, actual_harvest = CURRENT_DATE
            WHERE id = $1
            RETURNING id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at
            "#,
        )
        .bind(plant_id)
        .bind(new_status)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_as::<_, Plant>(
            r#"
            UPDATE plants SET status = $2
            WHERE id = $1
            RETURNING id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at
            "#,
        )
        .bind(plant_id)
        .bind(new_status)
        .fetch_one(pool)
        .await?
    };

    Ok(plant)
}
