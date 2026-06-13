use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncQueueItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub payload: serde_json::Value,
    pub signature: String,
    pub status: Option<String>,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockchainTransaction {
    pub id: Uuid,
    pub reference_type: String,
    pub reference_id: Uuid,
    pub tx_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
}
