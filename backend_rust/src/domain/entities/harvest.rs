use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Harvest {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub land_id: Uuid,
    pub quantity: sqlx::types::BigDecimal,
    pub unit: String,
    pub quality_grade: Option<String>,
    pub progress_percent: Option<i32>,
    pub harvested_at: Option<DateTime<Utc>>,
}
