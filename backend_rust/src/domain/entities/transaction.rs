use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PosTransaction {
    pub id: Uuid,
    pub cashier_id: Uuid,
    pub items: serde_json::Value,
    pub subtotal: i64,
    pub tax: i64,
    pub total: i64,
    pub amount_tendered: i64,
    pub change_amount: i64,
    pub created_at: Option<DateTime<Utc>>,
}
