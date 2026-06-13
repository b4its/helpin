use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plant {
    pub id: Uuid,
    pub land_id: Uuid,
    pub plant_type: String,
    pub plant_date: NaiveDate,
    pub estimated_harvest: Option<NaiveDate>,
    pub actual_harvest: Option<NaiveDate>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
