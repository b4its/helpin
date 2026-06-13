use anyhow::{anyhow, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use uuid::Uuid;

use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::infrastructure::services::jwt_service::JwtService;
use crate::domain::traits::repository::UserRepository;

pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub struct LoginOutput {
    pub user_id: Uuid,
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(
    input: LoginInput,
    user_repo: &PostgresUserRepository,
    jwt_service: &JwtService,
    expiry_minutes: u64,
    refresh_expiry_days: u64,
) -> Result<LoginOutput> {
    let user = user_repo.find_by_email(&input.email).await?
        .ok_or_else(|| anyhow!("Kredensial tidak valid"))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| anyhow!("Invalid password hash: {}", e))?;
    Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow!("Kredensial tidak valid"))?;

    // Generate tokens
    let access_token = jwt_service.generate_access_token(user.id, &user.role, expiry_minutes)?;
    let refresh_token = jwt_service.generate_refresh_token(user.id, refresh_expiry_days)?;

    Ok(LoginOutput { user_id: user.id, role: user.role, access_token, refresh_token })
}
