use anyhow::{anyhow, Result};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::infrastructure::services::jwt_service::JwtService;
use crate::infrastructure::services::crypto_service::Ed25519CryptoService;
use crate::domain::traits::repository::UserRepository;
use crate::domain::traits::service::CryptoService;

pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub struct RegisterOutput {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn register(
    input: RegisterInput,
    user_repo: &PostgresUserRepository,
    jwt_service: &JwtService,
    crypto_service: &Ed25519CryptoService,
) -> Result<RegisterOutput> {
    // Check if email already exists
    if user_repo.find_by_email(&input.email).await?.is_some() {
        return Err(anyhow!("Email already registered"));
    }

    // Hash password with Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?
        .to_string();

    // Generate Ed25519 keypair
    let (private_key, _public_key) = crypto_service.generate_keypair()?;

    // Create user
    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        email: input.email,
        name: input.name,
        password_hash,
        role: input.role.clone(),
        private_key: Some(private_key),
        created_at: None,
        updated_at: None,
    };

    user_repo.create(&user).await?;

    // Generate tokens
    let access_token = jwt_service.generate_access_token(user_id, &input.role, 480)?; // 8 jam
    let refresh_token = jwt_service.generate_refresh_token(user_id, 7)?;

    Ok(RegisterOutput { user_id, access_token, refresh_token })
}
