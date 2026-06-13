use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::harvest::Harvest;
use crate::domain::entities::plant::Plant;
use sqlx::PgPool;

pub struct RecordHarvestInput {
    pub plant_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub quality_grade: Option<String>,
}

pub async fn record_harvest(
    input: RecordHarvestInput,
    pool: &PgPool,
) -> Result<Harvest> {
    // Get plant to verify it exists and is in "Panen" status
    let plant = sqlx::query_as::<_, Plant>(
        "SELECT id, land_id, plant_type, plant_date, estimated_harvest, actual_harvest, status, created_at FROM plants WHERE id = $1"
    )
    .bind(input.plant_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Tanaman tidak ditemukan"))?;

    let status = plant.status.as_deref().unwrap_or("Ditanam");
    if status != "Panen" && status != "Selesai" {
        return Err(anyhow!(
            "Tanaman harus berstatus 'Panen' atau 'Selesai' untuk mencatat hasil panen"
        ));
    }

    let id = Uuid::new_v4();
    let quantity_bd = bigdecimal::BigDecimal::try_from(input.quantity)
        .map_err(|e| anyhow!("Invalid quantity: {}", e))?;

    let harvest = sqlx::query_as::<_, Harvest>(
        r#"
        INSERT INTO harvests (id, plant_id, land_id, quantity, unit, quality_grade, progress_percent)
        VALUES ($1, $2, $3, $4, $5, $6, 100)
        RETURNING id, plant_id, land_id, quantity, unit, quality_grade, progress_percent, harvested_at
        "#,
    )
    .bind(id)
    .bind(input.plant_id)
    .bind(plant.land_id)
    .bind(&quantity_bd)
    .bind(&input.unit)
    .bind(&input.quality_grade)
    .fetch_one(pool)
    .await?;

    Ok(harvest)
}
