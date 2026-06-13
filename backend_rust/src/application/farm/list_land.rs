use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::land::Land;
use crate::infrastructure::repositories::land_repo::PostgresLandRepository;
use crate::domain::traits::repository::LandRepository;

pub async fn list_land(
    owner_id: Uuid,
    land_repo: &PostgresLandRepository,
) -> Result<Vec<Land>> {
    let lands = land_repo.find_all_by_owner(owner_id).await?;
    Ok(lands)
}
