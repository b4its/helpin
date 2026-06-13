use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::livestock::Livestock;
use crate::domain::traits::repository::LivestockRepository;

#[derive(Clone)]
pub struct PostgresLivestockRepository {
    pool: PgPool,
}

impl PostgresLivestockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl LivestockRepository for PostgresLivestockRepository {
    async fn create(&self, livestock: &Livestock) -> Result<Livestock> {
        let row = sqlx::query_as::<_, Livestock>(
            r#"
            INSERT INTO livestock (id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics, created_at, updated_at
            "#,
        )
        .bind(livestock.id)
        .bind(livestock.owner_id)
        .bind(&livestock.tag_id)
        .bind(&livestock.category)
        .bind(&livestock.breed)
        .bind(&livestock.weight)
        .bind(&livestock.gender)
        .bind(&livestock.health_status)
        .bind(livestock.health_score)
        .bind(livestock.pen_id)
        .bind(livestock.age_months)
        .bind(livestock.entry_date)
        .bind(&livestock.biometrics)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_all_by_owner(&self, owner_id: Uuid) -> Result<Vec<Livestock>> {
        let rows = sqlx::query_as::<_, Livestock>(
            r#"
            SELECT id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics, created_at, updated_at
            FROM livestock WHERE owner_id = $1
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Livestock>> {
        let row = sqlx::query_as::<_, Livestock>(
            r#"
            SELECT id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics, created_at, updated_at
            FROM livestock WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update(&self, livestock: &Livestock) -> Result<Livestock> {
        let row = sqlx::query_as::<_, Livestock>(
            r#"
            UPDATE livestock
            SET tag_id = $2, category = $3, breed = $4, weight = $5, gender = $6,
                health_status = $7, health_score = $8, pen_id = $9, entry_date = $10,
                biometrics = $11, age_months = $12, updated_at = NOW()
            WHERE id = $1
            RETURNING id, owner_id, tag_id, category, breed, weight, gender, health_status, health_score, pen_id, age_months, entry_date, biometrics, created_at, updated_at
            "#,
        )
        .bind(livestock.id)
        .bind(&livestock.tag_id)
        .bind(&livestock.category)
        .bind(&livestock.breed)
        .bind(&livestock.weight)
        .bind(&livestock.gender)
        .bind(&livestock.health_status)
        .bind(livestock.health_score)
        .bind(livestock.pen_id)
        .bind(livestock.entry_date)
        .bind(&livestock.biometrics)
        .bind(livestock.age_months)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM livestock WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
