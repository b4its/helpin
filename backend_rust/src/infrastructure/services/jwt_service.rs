use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtService {
    jwt_secret: String,
}

impl JwtService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        role: &str,
        expiry_minutes: u64,
    ) -> Result<String> {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(expiry_minutes as i64))
            .ok_or_else(|| anyhow!("Failed to calculate token expiry"))?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Failed to generate access token: {}", e))?;

        Ok(token)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, expiry_days: u64) -> Result<String> {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::days(expiry_days as i64))
            .ok_or_else(|| anyhow!("Failed to calculate refresh token expiry"))?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            role: "refresh".to_string(),
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Failed to generate refresh token: {}", e))?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Invalid token: {}", e))?;

        Ok(token_data.claims)
    }
}
