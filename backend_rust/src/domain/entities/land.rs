use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Lahan pertanian milik petani.
/// crop_type ditentukan dari bibit yang dipilih saat buat lahan.
/// Semua tanaman di lahan ini = 1 komoditas (crop_type).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Land {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub code: String,
    pub name: String,
    pub area_hectare: sqlx::types::BigDecimal,
    pub soil_type: String,
    pub status: String,
    pub crop_type: Option<String>,  // komoditas tunggal, dari bibit yang dipilih
    pub location: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
