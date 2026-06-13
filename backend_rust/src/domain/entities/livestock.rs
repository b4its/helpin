use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Livestock {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub tag_id: String,
    pub category: String,
    pub breed: String,
    pub weight: sqlx::types::BigDecimal,
    pub gender: String,
    pub health_status: Option<String>,
    pub health_score: Option<i32>,
    pub pen_id: Option<Uuid>,
    pub age_months: Option<i32>,
    pub entry_date: NaiveDate,
    pub biometrics: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
