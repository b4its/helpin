use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::livestock::Livestock;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;

pub struct ListLivestockInput {
    pub owner_id: Uuid,
    pub category: Option<String>,
    pub search: Option<String>,
}

pub async fn list_livestock(
    input: ListLivestockInput,
    livestock_repo: &PostgresLivestockRepository,
) -> Result<Vec<Livestock>> {
    let all = livestock_repo.find_all_by_owner(input.owner_id).await?;

    let filtered: Vec<Livestock> = all
        .into_iter()
        .filter(|l| {
            // Filter by category if provided
            if let Some(ref cat) = input.category {
                if l.category.to_lowercase() != cat.to_lowercase() {
                    return false;
                }
            }
            // Filter by search query (tag_id or breed)
            if let Some(ref query) = input.search {
                let q = query.to_lowercase();
                let tag_match = l.tag_id.to_lowercase().contains(&q);
                let breed_match = l.breed.to_lowercase().contains(&q);
                if !tag_match && !breed_match {
                    return false;
                }
            }
            true
        })
        .collect();

    Ok(filtered)
}
