use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::inventory::Inventory;
use crate::domain::traits::repository::InventoryRepository;

#[derive(Clone)]
pub struct PostgresInventoryRepository {
    pool: PgPool,
}

impl PostgresInventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl InventoryRepository for PostgresInventoryRepository {
    async fn create(&self, inventory: &Inventory) -> Result<Inventory> {
        let row = sqlx::query_as::<_, Inventory>(
            r#"
            INSERT INTO inventories (id, owner_id, land_id, category, name, quantity, unit)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, owner_id, land_id, category, name, quantity, unit, created_at
            "#,
        )
        .bind(inventory.id)
        .bind(inventory.owner_id)
        .bind(inventory.land_id)
        .bind(&inventory.category)
        .bind(&inventory.name)
        .bind(&inventory.quantity)
        .bind(&inventory.unit)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn find_by_land(&self, land_id: Uuid) -> Result<Vec<Inventory>> {
        // Return inventory that is linked to this land via land_inventories junction table
        let rows = sqlx::query_as::<_, Inventory>(
            r#"
            SELECT i.id, i.owner_id, i.land_id, i.category, i.name, i.quantity, i.unit, i.created_at
            FROM inventories i
            INNER JOIN land_inventories li ON li.inventory_id = i.id
            WHERE li.land_id = $1
            ORDER BY i.category, i.name
            "#,
        )
        .bind(land_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_all_by_owner(&self, owner_id: Uuid) -> Result<Vec<Inventory>> {
        let rows = sqlx::query_as::<_, Inventory>(
            r#"
            SELECT id, owner_id, land_id, category, name, quantity, unit, created_at
            FROM inventories WHERE owner_id = $1
            ORDER BY category, name
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_by_owner_and_categories(&self, owner_id: Uuid, categories: &[&str]) -> Result<Vec<Inventory>> {
        // sqlx doesn't support binding arrays directly in all versions, use unnest
        let rows = sqlx::query_as::<_, Inventory>(
            r#"
            SELECT id, owner_id, land_id, category, name, quantity, unit, created_at
            FROM inventories
            WHERE owner_id = $1 AND category = ANY($2)
            ORDER BY category, name
            "#,
        )
        .bind(owner_id)
        .bind(categories)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Inventory>> {
        let row = sqlx::query_as::<_, Inventory>(
            r#"
            SELECT id, owner_id, land_id, category, name, quantity, unit, created_at
            FROM inventories WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update(&self, inventory: &Inventory) -> Result<Inventory> {
        let row = sqlx::query_as::<_, Inventory>(
            r#"
            UPDATE inventories
            SET category = $2, name = $3, quantity = $4, unit = $5, land_id = $6
            WHERE id = $1
            RETURNING id, owner_id, land_id, category, name, quantity, unit, created_at
            "#,
        )
        .bind(inventory.id)
        .bind(&inventory.category)
        .bind(&inventory.name)
        .bind(&inventory.quantity)
        .bind(&inventory.unit)
        .bind(inventory.land_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM inventories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
