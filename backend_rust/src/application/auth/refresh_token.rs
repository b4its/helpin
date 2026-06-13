use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::infrastructure::services::jwt_service::JwtService;
use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::domain::traits::repository::UserRepository;

pub struct RefreshOutput {
    pub access_token: String,
}

pub async fn refresh_token(
    token: &str,
    jwt_service: &JwtService,
    user_repo: &PostgresUserRepository,
    expiry_minutes: u64,
) -> Result<RefreshOutput> {
    let claims = jwt_service.validate_token(token)?;

    if claims.role != "refresh" {
        return Err(anyhow!("Invalid refresh token"));
    }

    let user_id = Uuid::parse_str(&claims.sub)?;
    let user = user_repo.find_by_id(user_id).await?
        .ok_or_else(|| anyhow!("User not found"))?;

    let access_token = jwt_service.generate_access_token(user.id, &user.role, expiry_minutes)?;

    Ok(RefreshOutput { access_token })
}
