use anyhow::{anyhow, Result};
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::application::farm::treatment::refresh_land_prediction;
use crate::domain::entities::land::Land;
use crate::infrastructure::repositories::land_repo::PostgresLandRepository;
use crate::infrastructure::services::mongo_service::MongoService;
use crate::domain::traits::repository::LandRepository;

pub struct CreateLandInput {
    pub owner_id: Uuid,
    pub name: String,
    pub area_hectare: f64,
    pub soil_type: String,
    pub status: Option<String>,
    pub crop_type: Option<String>,  // komoditas tunggal lahan, dari bibit yang dipilih
    pub location: Option<String>,
    pub inventory_ids: Option<Vec<Uuid>>, // inventori yang dipakai di lahan ini
}

pub async fn create_land(
    input: CreateLandInput,
    land_repo: &PostgresLandRepository,
    pool: &PgPool,
    mongo: &MongoService,
) -> Result<Land> {
    if input.area_hectare <= 0.0 {
        return Err(anyhow!("Luas lahan harus lebih dari 0"));
    }

    // Default status "Persiapan" — tidak perlu diinput user
    let status = input.status.unwrap_or_else(|| "Persiapan".to_string());

    let valid_statuses = ["Aktif Ditanami", "Persiapan", "Masa Bera"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(anyhow!(
            "Status tidak valid. Harus salah satu dari: Aktif Ditanami, Persiapan, Masa Bera"
        ));
    }

    let code = {
        let mut rng = rand::thread_rng();
        format!("LHN-{:03}", rng.gen_range(0..1000))
    };

    let land = Land {
        id: Uuid::new_v4(),
        owner_id: input.owner_id,
        code,
        name: input.name,
        area_hectare: bigdecimal::BigDecimal::try_from(input.area_hectare)
            .map_err(|e| anyhow!("Invalid area: {}", e))?,
        soil_type: input.soil_type,
        status,
        crop_type: input.crop_type,
        location: input.location,
        created_at: None,
        updated_at: None,
    };

    let created = land_repo.create(&land).await?;

    // Link inventori ke lahan via land_inventories junction table
    if let Some(inv_ids) = input.inventory_ids {
        for inv_id in inv_ids {
            // Ignore errors silently (e.g., duplicate or invalid inventory_id)
            let _ = sqlx::query(
                "INSERT INTO land_inventories (id, land_id, inventory_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
            )
            .bind(Uuid::new_v4())
            .bind(created.id)
            .bind(inv_id)
            .execute(pool)
            .await;
        }
    }

    // Auto-prediksi panen: jika lahan punya komoditas (crop_type), buat tanaman aktif,
    // hitung estimasi panen, dan simpan ke riwayat panen (harvests). Best-effort —
    // kegagalan prediksi tidak membatalkan pembuatan lahan.
    if created.crop_type.as_deref().map(|c| !c.trim().is_empty()).unwrap_or(false) {
        match refresh_land_prediction(pool, mongo, created.id).await {
            Ok(Some(pred)) => {
                tracing::info!(
                    "Auto-prediksi lahan {}: {} kg, grade {}, panen {}",
                    created.id, pred.predicted_weight_kg, pred.quality_grade, pred.predicted_harvest_date
                );
            }
            Ok(None) => {}
            Err(e) => tracing::warn!("Auto-prediksi lahan {} gagal: {}", created.id, e),
        }
    }

    Ok(created)
}
