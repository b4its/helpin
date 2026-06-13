use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FinancialRecord {
    pub id: Uuid,
    pub record_type: String,
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub balance_after: i64,
    pub recorded_at: Option<DateTime<Utc>>,
}
