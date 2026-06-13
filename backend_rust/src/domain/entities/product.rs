use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub price: i64,
    pub stock: i32,
    pub unit: String,
    pub product_type: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub nutrition: Option<serde_json::Value>,
    pub purchase_price: Option<i64>,
    pub expired_at: Option<chrono::NaiveDate>,
    pub entry_date: Option<chrono::NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub item_count: Option<i32>,
}
