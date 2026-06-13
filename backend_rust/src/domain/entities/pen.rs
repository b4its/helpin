use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pen {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub pen_type: String,
    pub location: Option<String>,
    pub condition: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
