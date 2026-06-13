use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::domain::traits::repository::UserRepository;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User> {
        let row = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, name, password_hash, role, private_key)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, name, password_hash, role, private_key, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(&user.private_key)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, User>(
            "SELECT id, email, name, password_hash, role, private_key, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, User>(
            "SELECT id, email, name, password_hash, role, private_key, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
}
