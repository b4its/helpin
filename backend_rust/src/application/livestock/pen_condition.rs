//! Auto-generate kondisi kandang dari okupansi (jumlah ternak vs kapasitas)
//! dan rata-rata skor kesehatan ternak di dalamnya.

use sqlx::PgPool;
use uuid::Uuid;

/// Hitung jumlah ternak di sebuah kandang.
pub async fn count_occupancy(pool: &PgPool, pen_id: Uuid) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM livestock WHERE pen_id = $1")
        .bind(pen_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
}

/// Rata-rata skor kesehatan ternak di kandang (0..100). None jika kosong.
pub async fn avg_health(pool: &PgPool, pen_id: Uuid) -> Option<f64> {
    sqlx::query_scalar::<_, Option<f64>>(
        "SELECT AVG(health_score)::float8 FROM livestock WHERE pen_id = $1",
    )
    .bind(pen_id)
    .fetch_one(pool)
    .await
    .ok()
    .flatten()
}

/// Tentukan label kondisi kandang.
pub fn generate_condition(occupancy: i64, capacity: i32, avg_health: Option<f64>) -> String {
    if occupancy <= 0 {
        return "Kosong".to_string();
    }
    let cap = if capacity > 0 { capacity as f64 } else { 1.0 };
    let ratio = occupancy as f64 / cap;
    let health = avg_health.unwrap_or(100.0);

    if ratio > 1.0 {
        "Overkapasitas".to_string()
    } else if health < 50.0 {
        "Buruk".to_string()
    } else if health < 75.0 {
        "Perlu Perhatian".to_string()
    } else if ratio >= 0.85 {
        "Padat".to_string()
    } else {
        "Optimal".to_string()
    }
}

/// Hitung & simpan ulang kondisi kandang ke kolom `pens.condition`.
pub async fn refresh_pen_condition(pool: &PgPool, pen_id: Uuid) {
    let occupancy = count_occupancy(pool, pen_id).await;
    let capacity = sqlx::query_scalar::<_, i32>("SELECT capacity FROM pens WHERE id = $1")
        .bind(pen_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(0);
    let health = avg_health(pool, pen_id).await;
    let condition = generate_condition(occupancy, capacity, health);

    let _ = sqlx::query("UPDATE pens SET condition = $2 WHERE id = $1")
        .bind(pen_id)
        .bind(&condition)
        .execute(pool)
        .await;
}
