use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LivestockResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub tag_id: String,
    pub category: String,
    pub breed: String,
    pub weight: f64,
    pub gender: String,
    pub health_status: Option<String>,
    pub health_score: Option<i32>,
    pub pen_id: Option<Uuid>,
    pub age_months: Option<i32>,
    pub entry_date: NaiveDate,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PenResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub pen_type: String,
    pub location: Option<String>,
    pub condition: Option<String>,
    pub occupancy: i64,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LandResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub code: String,
    pub name: String,
    pub area_hectare: f64,
    pub soil_type: String,
    pub status: String,
    pub crop_type: Option<String>,
    pub location: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
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
    pub expired_at: Option<NaiveDate>,
    pub entry_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub status: Option<String>,
    pub total_amount: i64,
    pub shipping_address: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PosTransactionResponse {
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

#[derive(Debug, Serialize)]
pub struct FinancialRecordResponse {
    pub id: Uuid,
    pub record_type: String,
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub balance_after: i64,
    pub recorded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub balance: i64,
}

#[derive(Debug, Serialize)]
pub struct SyncQueueResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub payload: serde_json::Value,
    pub status: Option<String>,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ProcessQueueResponse {
    pub processed: u32,
    pub succeeded: u32,
    pub failed: u32,
}

#[derive(Debug, Serialize)]
pub struct SyncStatusResponse {
    pub pending: usize,
    pub items: Vec<SyncQueueResponse>,
}

#[derive(Debug, Serialize)]
pub struct MlPredictionResponse {
    pub recommendation_id: Uuid,
    pub prediction: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct MlHealthResponse {
    pub health_score: i32,
    pub evaluation: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct HealthRecordResponse {
    pub id: Uuid,
    pub livestock_id: Uuid,
    pub symptoms: Option<String>,
    pub body_temp: Option<f64>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub notes: Option<String>,
    pub health_score: Option<i32>,
    pub recorded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PlantResponse {
    pub id: Uuid,
    pub land_id: Uuid,
    pub plant_type: String,
    pub plant_date: NaiveDate,
    pub estimated_harvest: Option<NaiveDate>,
    pub actual_harvest: Option<NaiveDate>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct HarvestResponse {
    pub id: Uuid,
    pub plant_id: Uuid,
    pub land_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub quality_grade: Option<String>,
    pub progress_percent: Option<i32>,
    pub harvested_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub land_id: Option<Uuid>,
    pub category: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub item_count: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}
