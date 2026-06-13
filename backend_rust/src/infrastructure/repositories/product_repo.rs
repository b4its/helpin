use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::product::Product;
use crate::domain::traits::repository::ProductRepository;

#[derive(Clone)]
pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

const COLS: &str = "id, seller_id, category_id, name, price, stock, unit, product_type, location, image_url, description, nutrition, purchase_price, expired_at, entry_date, created_at, updated_at";

impl ProductRepository for PostgresProductRepository {
    async fn create(&self, product: &Product) -> Result<Product> {
        let row = sqlx::query_as::<_, Product>(&format!(
            r#"
            INSERT INTO products (id, seller_id, category_id, name, price, stock, unit, product_type, location, image_url, description, nutrition, purchase_price, expired_at, entry_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, COALESCE($15, CURRENT_DATE))
            RETURNING {COLS}
            "#
        ))
        .bind(product.id)
        .bind(product.seller_id)
        .bind(product.category_id)
        .bind(&product.name)
        .bind(product.price)
        .bind(product.stock)
        .bind(&product.unit)
        .bind(&product.product_type)
        .bind(&product.location)
        .bind(&product.image_url)
        .bind(&product.description)
        .bind(&product.nutrition)
        .bind(product.purchase_price)
        .bind(product.expired_at)
        .bind(product.entry_date)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<Product>> {
        let rows = sqlx::query_as::<_, Product>(&format!(
            "SELECT {COLS} FROM products ORDER BY created_at DESC"
        ))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>> {
        let row = sqlx::query_as::<_, Product>(&format!(
            "SELECT {COLS} FROM products WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, product: &Product) -> Result<Product> {
        let row = sqlx::query_as::<_, Product>(&format!(
            r#"
            UPDATE products
            SET category_id = $2, name = $3, price = $4, stock = $5, unit = $6,
                product_type = $7, location = $8, image_url = $9, description = $10, nutrition = $11,
                purchase_price = $12, expired_at = $13, entry_date = $14, updated_at = NOW()
            WHERE id = $1
            RETURNING {COLS}
            "#
        ))
        .bind(product.id)
        .bind(product.category_id)
        .bind(&product.name)
        .bind(product.price)
        .bind(product.stock)
        .bind(&product.unit)
        .bind(&product.product_type)
        .bind(&product.location)
        .bind(&product.image_url)
        .bind(&product.description)
        .bind(&product.nutrition)
        .bind(product.purchase_price)
        .bind(product.expired_at)
        .bind(product.entry_date)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
