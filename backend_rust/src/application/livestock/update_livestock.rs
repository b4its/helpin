use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::livestock::Livestock;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;

pub struct UpdateLivestockInput {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub tag_id: Option<String>,
    pub category: Option<String>,
    pub breed: Option<String>,
    pub weight: Option<f64>,
    pub gender: Option<String>,
    pub health_status: Option<String>,
    pub pen_id: Option<Uuid>,
    pub age_months: Option<i32>,
}

pub async fn update_livestock(
    input: UpdateLivestockInput,
    livestock_repo: &PostgresLivestockRepository,
) -> Result<Livestock> {
    let mut livestock = livestock_repo.find_by_id(input.id).await?
        .ok_or_else(|| anyhow!("Ternak tidak ditemukan"))?;

    // Verify ownership
    if livestock.owner_id != input.owner_id {
        return Err(anyhow!("Tidak memiliki akses ke ternak ini"));
    }

    // Update fields if provided
    if let Some(tag_id) = input.tag_id {
        livestock.tag_id = tag_id;
    }
    if let Some(category) = input.category {
        livestock.category = category;
    }
    if let Some(breed) = input.breed {
        livestock.breed = breed;
    }
    if let Some(weight) = input.weight {
        livestock.weight = bigdecimal::BigDecimal::try_from(weight)
            .map_err(|e| anyhow!("Invalid weight: {}", e))?;
    }
    if let Some(gender) = input.gender {
        livestock.gender = gender;
    }
    if let Some(status) = input.health_status {
        livestock.health_status = Some(status);
    }
    if let Some(pen_id) = input.pen_id {
        livestock.pen_id = Some(pen_id);
    }
    if let Some(age) = input.age_months {
        livestock.age_months = Some(age);
    }

    let updated = livestock_repo.update(&livestock).await?;
    Ok(updated)
}
