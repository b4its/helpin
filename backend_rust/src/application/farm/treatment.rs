//! Logika treatment lahan + auto-prediksi panen.
//!
//! Dipakai saat lahan dibuat (prediksi otomatis) dan saat user menambahkan
//! informasi treatment (pupuk/alat) lewat tombol "Tambah Informasi".

use anyhow::Result;
use bigdecimal::ToPrimitive;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::application::farm::predict_service::{compute_prediction, PredictionResult, TreatmentSummary};
use crate::infrastructure::services::mongo_service::MongoService;

/// Bangun ringkasan treatment dari inventori yang ter-link ke lahan (PostgreSQL)
/// digabung dengan agregat treatment fleksibel (MongoDB).
pub async fn build_treatment_summary(
    pool: &PgPool,
    mongo: &MongoService,
    land_id: Uuid,
) -> TreatmentSummary {
    let mut summary = TreatmentSummary::default();

    // Inventori yang dipakai di lahan (via junction land_inventories)
    let rows = sqlx::query_as::<_, (String, sqlx::types::BigDecimal)>(
        r#"
        SELECT i.category, i.quantity
        FROM land_inventories li
        JOIN inventories i ON i.id = li.inventory_id
        WHERE li.land_id = $1
        "#,
    )
    .bind(land_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for (category, qty) in rows {
        let q = qty.to_f64().unwrap_or(0.0);
        match category.as_str() {
            "Bibit" => summary.seed_present = true,
            "Pupuk" => {
                summary.fertilizer_count += 1;
                summary.fertilizer_total_qty += q;
            }
            "Alat Pertanian" => summary.tool_count += 1,
            _ => {}
        }
    }

    // Tambahan dari treatment fleksibel di MongoDB
    let (fert_count, fert_qty, tool_count) = mongo.treatment_aggregate(land_id).await;
    summary.fertilizer_count += fert_count;
    summary.fertilizer_total_qty += fert_qty;
    summary.tool_count += tool_count;

    summary
}

/// Jalankan prediksi untuk lahan & simpan/perbarui estimasi panen di tabel `harvests`.
///
/// - Membuat tanaman aktif jika belum ada (saat crop_type tersedia).
/// - Memperbarui `plants.estimated_harvest`.
/// - Menyimpan estimasi sebagai baris `harvests` (is_estimate = true, progress 0).
/// - Jika grade hasil prediksi sama dengan sebelumnya => cukup update `updated_at`.
///
/// Mengembalikan `None` jika lahan belum punya komoditas (crop_type).
pub async fn refresh_land_prediction(
    pool: &PgPool,
    mongo: &MongoService,
    land_id: Uuid,
) -> Result<Option<PredictionResult>> {
    // Ambil data lahan
    let land = sqlx::query_as::<_, (Option<String>, sqlx::types::BigDecimal, String)>(
        "SELECT crop_type, area_hectare, soil_type FROM lands WHERE id = $1",
    )
    .bind(land_id)
    .fetch_optional(pool)
    .await?;

    let Some((crop_type_opt, area_bd, soil_type)) = land else {
        return Ok(None);
    };
    let Some(crop_type) = crop_type_opt.filter(|c| !c.trim().is_empty()) else {
        return Ok(None); // belum ada komoditas → tidak bisa prediksi
    };
    let area = area_bd.to_f64().unwrap_or(1.0);

    // Cari tanaman aktif; buat jika belum ada
    let active_plant = sqlx::query_as::<_, (Uuid, chrono::NaiveDate)>(
        "SELECT id, plant_date FROM plants WHERE land_id = $1 AND status IN ('Ditanam', 'Tumbuh') ORDER BY plant_date DESC LIMIT 1",
    )
    .bind(land_id)
    .fetch_optional(pool)
    .await?;

    let (plant_id, plant_date) = match active_plant {
        Some(p) => p,
        None => {
            let today = Utc::now().date_naive();
            let new_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO plants (id, land_id, plant_type, plant_date, status) VALUES ($1, $2, $3, $4, 'Ditanam')",
            )
            .bind(new_id)
            .bind(land_id)
            .bind(&crop_type)
            .bind(today)
            .execute(pool)
            .await?;
            (new_id, today)
        }
    };

    // Bangun ringkasan treatment & hitung prediksi
    let summary = build_treatment_summary(pool, mongo, land_id).await;
    let prediction = compute_prediction(&crop_type, area, &soil_type, plant_date, &summary);

    // Perbarui estimasi panen tanaman
    sqlx::query("UPDATE plants SET estimated_harvest = $2 WHERE id = $1")
        .bind(plant_id)
        .bind(prediction.predicted_harvest_date)
        .execute(pool)
        .await?;

    let weight_bd = bigdecimal::BigDecimal::try_from(prediction.predicted_weight_kg)
        .unwrap_or_else(|_| bigdecimal::BigDecimal::from(0));

    // Cari estimasi panen yang sudah ada untuk tanaman ini
    let existing = sqlx::query_as::<_, (Uuid, Option<String>)>(
        "SELECT id, quality_grade FROM harvests WHERE plant_id = $1 AND is_estimate = TRUE ORDER BY updated_at DESC LIMIT 1",
    )
    .bind(plant_id)
    .fetch_optional(pool)
    .await?;

    match existing {
        Some((harvest_id, prev_grade)) => {
            if prev_grade.as_deref() == Some(prediction.quality_grade.as_str()) {
                // Grade sama → cukup perbarui updated_at
                sqlx::query("UPDATE harvests SET updated_at = NOW() WHERE id = $1")
                    .bind(harvest_id)
                    .execute(pool)
                    .await?;
            } else {
                // Grade berubah → perbarui estimasi
                sqlx::query(
                    r#"
                    UPDATE harvests
                    SET quantity = $2, quality_grade = $3, predicted_harvest_date = $4, updated_at = NOW()
                    WHERE id = $1
                    "#,
                )
                .bind(harvest_id)
                .bind(&weight_bd)
                .bind(&prediction.quality_grade)
                .bind(prediction.predicted_harvest_date)
                .execute(pool)
                .await?;
            }
        }
        None => {
            // Belum ada estimasi → buat baris riwayat panen baru (estimasi)
            sqlx::query(
                r#"
                INSERT INTO harvests
                    (id, plant_id, land_id, quantity, unit, quality_grade, progress_percent, is_estimate, predicted_harvest_date, harvested_at, updated_at)
                VALUES ($1, $2, $3, $4, 'kg', $5, 0, TRUE, $6, NOW(), NOW())
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(plant_id)
            .bind(land_id)
            .bind(&weight_bd)
            .bind(&prediction.quality_grade)
            .bind(prediction.predicted_harvest_date)
            .execute(pool)
            .await?;
        }
    }

    Ok(Some(prediction))
}
