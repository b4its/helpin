use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::land::Land;
use crate::domain::traits::repository::LandRepository;

#[derive(Clone)]
pub struct PostgresLandRepository {
    pool: PgPool,
}

impl PostgresLandRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl LandRepository for PostgresLandRepository {
    async fn create(&self, land: &Land) -> Result<Land> {
        let row = sqlx::query_as::<_, Land>(
            r#"
            INSERT INTO lands (id, owner_id, code, name, area_hectare, soil_type, status, crop_type, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, owner_id, code, name, area_hectare, soil_type, status, crop_type, location, created_at, updated_at
            "#,
        )
        .bind(land.id)
        .bind(land.owner_id)
        .bind(&land.code)
        .bind(&land.name)
        .bind(&land.area_hectare)
        .bind(&land.soil_type)
        .bind(&land.status)
        .bind(&land.crop_type)
        .bind(&land.location)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_all_by_owner(&self, owner_id: Uuid) -> Result<Vec<Land>> {
        let rows = sqlx::query_as::<_, Land>(
            r#"
            SELECT id, owner_id, code, name, area_hectare, soil_type, status, crop_type, location, created_at, updated_at
            FROM lands WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Land>> {
        let row = sqlx::query_as::<_, Land>(
            r#"
            SELECT id, owner_id, code, name, area_hectare, soil_type, status, crop_type, location, created_at, updated_at
            FROM lands WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update(&self, land: &Land) -> Result<Land> {
        let row = sqlx::query_as::<_, Land>(
            r#"
            UPDATE lands
            SET code = $2, name = $3, area_hectare = $4, soil_type = $5, status = $6,
                crop_type = $7, location = $8, updated_at = NOW()
            WHERE id = $1
            RETURNING id, owner_id, code, name, area_hectare, soil_type, status, crop_type, location, created_at, updated_at
            "#,
        )
        .bind(land.id)
        .bind(&land.code)
        .bind(&land.name)
        .bind(&land.area_hectare)
        .bind(&land.soil_type)
        .bind(&land.status)
        .bind(&land.crop_type)
        .bind(&land.location)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM lands WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
