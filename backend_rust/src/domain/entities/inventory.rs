use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Inventori milik petani (owner) — independen dari lahan.
/// Bisa dipakai di banyak lahan lewat tabel land_inventories.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub land_id: Option<Uuid>,   // opsional: referensi lahan jika sudah dipakai
    pub category: String,        // "Pupuk" | "Bibit" | "Alat Pertanian" | "Lainnya"
    pub name: String,
    pub quantity: sqlx::types::BigDecimal,
    pub unit: String,
    pub created_at: Option<DateTime<Utc>>,
}
