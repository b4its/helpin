use anyhow::{anyhow, Result};
use chrono::Utc;
use rand::Rng;
use uuid::Uuid;

use crate::domain::entities::livestock::Livestock;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;

pub struct CreateLivestockInput {
    pub owner_id: Uuid,
    pub tag_id: Option<String>,
    pub category: String,
    pub breed: String,
    pub weight: Option<f64>,
    pub gender: Option<String>,
    pub age_months: Option<i32>,
    pub pen_id: Option<Uuid>,
}

/// Buat kode tag otomatis berdasarkan jenis ternak, mis. "UNG-482".
fn generate_tag_id(category: &str) -> String {
    let prefix: String = category
        .chars()
        .filter(|c| c.is_alphabetic())
        .take(3)
        .collect::<String>()
        .to_uppercase();
    let prefix = if prefix.is_empty() { "TRN".to_string() } else { prefix };
    let mut rng = rand::thread_rng();
    format!("{}-{:03}", prefix, rng.gen_range(0..1000))
}

pub async fn create_livestock(
    input: CreateLivestockInput,
    livestock_repo: &PostgresLivestockRepository,
) -> Result<Livestock> {
    if input.category.trim().is_empty() {
        return Err(anyhow!("Jenis ternak wajib diisi"));
    }

    // ID otomatis jika tidak diberikan
    let tag_id = input
        .tag_id
        .filter(|t| !t.trim().is_empty())
        .unwrap_or_else(|| generate_tag_id(&input.category));

    let weight = input.weight.unwrap_or(0.0);
    let gender = input.gender.filter(|g| !g.trim().is_empty()).unwrap_or_else(|| "-".to_string());

    let livestock = Livestock {
        id: Uuid::new_v4(),
        owner_id: input.owner_id,
        tag_id,
        category: input.category,
        breed: input.breed,
        weight: bigdecimal::BigDecimal::try_from(weight)
            .map_err(|e| anyhow!("Invalid weight: {}", e))?,
        gender,
        health_status: Some("Sehat".to_string()),
        health_score: Some(100),
        pen_id: input.pen_id,
        age_months: input.age_months,
        entry_date: Utc::now().date_naive(),
        biometrics: Some(serde_json::json!([])),
        created_at: None,
        updated_at: None,
    };

    let created = livestock_repo.create(&livestock).await?;
    Ok(created)
}
