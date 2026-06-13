use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;

pub async fn delete_livestock(
    id: Uuid,
    owner_id: Uuid,
    livestock_repo: &PostgresLivestockRepository,
) -> Result<()> {
    let livestock = livestock_repo.find_by_id(id).await?
        .ok_or_else(|| anyhow!("Ternak tidak ditemukan"))?;

    // Verify ownership
    if livestock.owner_id != owner_id {
        return Err(anyhow!("Tidak memiliki akses ke ternak ini"));
    }

    livestock_repo.delete(id).await?;
    Ok(())
}
