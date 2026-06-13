use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLivestockRequest {
    pub tag_id: Option<String>,
    #[validate(length(min = 1, message = "Jenis ternak wajib diisi"))]
    pub category: String,
    #[validate(length(min = 1, message = "Ras wajib diisi"))]
    pub breed: String,
    pub weight: Option<f64>,
    pub gender: Option<String>,
    pub age_months: Option<i32>,
    pub pen_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLivestockRequest {
    pub tag_id: Option<String>,
    pub category: Option<String>,
    pub breed: Option<String>,
    pub weight: Option<f64>,
    pub gender: Option<String>,
    pub health_status: Option<String>,
    pub pen_id: Option<Uuid>,
    pub age_months: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLandRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub area_hectare: f64,
    #[validate(length(min = 1, message = "Soil type is required"))]
    pub soil_type: String,
    pub status: Option<String>,
    /// Komoditas lahan — diambil dari nama bibit yang dipilih (misal "Padi IR64" → crop_type "Padi IR64")
    pub crop_type: Option<String>,
    pub location: Option<String>,
    /// ID inventori (Bibit, Pupuk, Alat Pertanian) yang dipakai di lahan ini
    pub inventory_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLandRequest {
    pub name: Option<String>,
    pub area_hectare: Option<f64>,
    pub soil_type: Option<String>,
    pub status: Option<String>,
    pub crop_type: Option<String>,
    pub location: Option<String>,
    /// Ganti set inventori yang dipakai di lahan (opsional — null berarti tidak berubah)
    pub inventory_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    pub category_id: Option<Uuid>,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub price: i64,
    pub stock: i32,
    #[validate(length(min = 1, message = "Unit is required"))]
    pub unit: String,
    pub product_type: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub nutrition: Option<serde_json::Value>,
    pub purchase_price: Option<i64>,
    pub expired_at: Option<chrono::NaiveDate>,
    pub entry_date: Option<chrono::NaiveDate>,
    /// koordinat opsional untuk activity log
    pub lat: Option<f64>,
    pub long: Option<f64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub price: Option<i64>,
    pub stock: Option<i32>,
    pub unit: Option<String>,
    pub product_type: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub nutrition: Option<serde_json::Value>,
    pub purchase_price: Option<i64>,
    pub expired_at: Option<chrono::NaiveDate>,
    pub entry_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartQuantityRequest {
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PosItemInput {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PosTransactionRequest {
    pub items: Vec<PosItemInput>,
    pub amount_tendered: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordExpenseRequest {
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct FeedPredictionRequest {
    pub livestock_id: Option<Uuid>,
    #[validate(length(min = 1, message = "Tag ID is required"))]
    pub tag_id: String,
    #[validate(length(min = 1, message = "Breed is required"))]
    pub breed: String,
    pub weight: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct HealthEvaluationRequest {
    pub livestock_id: Uuid,
    pub heart_rate: Option<i32>,
    pub body_temp: Option<f64>,
    pub respiratory_rate: Option<i32>,
    pub symptoms: Option<String>,
    pub weight: Option<f64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnqueueSyncRequest {
    pub payload: serde_json::Value,
    #[validate(length(min = 1, message = "Signature is required"))]
    pub signature: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePenRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub capacity: i32,
    #[validate(length(min = 1, message = "Pen type is required"))]
    pub pen_type: String,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePenRequest {
    pub name: Option<String>,
    pub capacity: Option<i32>,
    pub pen_type: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordHealthRequest {
    pub livestock_id: Uuid,
    pub symptoms: Option<String>,
    pub body_temp: Option<f64>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePlantRequest {
    pub land_id: Uuid,
    pub plant_type: String,
    pub plant_date: chrono::NaiveDate,
    pub estimated_harvest: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePlantStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInventoryRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,
    pub quantity: f64,
    #[validate(length(min = 1, message = "Unit is required"))]
    pub unit: String,
    pub land_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInventoryRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub land_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CheckoutRequest {
    pub shipping_address: Option<String>,
}

/// Query parameters for livestock list
#[derive(Debug, Deserialize)]
pub struct LivestockFilterParams {
    pub category: Option<String>,
    pub search: Option<String>,
}

/// Query parameters for financial report
#[derive(Debug, Deserialize)]
pub struct FinanceReportParams {
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}

/// Query parameters for prefetch delta sync
#[derive(Debug, Deserialize)]
pub struct DeltaSyncParams {
    pub since: Option<chrono::DateTime<chrono::Utc>>,
}
