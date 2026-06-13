use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HealthRecord {
    pub id: Uuid,
    pub livestock_id: Uuid,
    pub symptoms: Option<String>,
    pub body_temp: Option<sqlx::types::BigDecimal>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub notes: Option<String>,
    pub health_score: Option<i32>,
    pub recorded_at: Option<DateTime<Utc>>,
}
