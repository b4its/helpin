use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::order::{Order, OrderItem};
use crate::domain::traits::repository::OrderRepository;

#[derive(Clone)]
pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl OrderRepository for PostgresOrderRepository {
    async fn create(&self, order: &Order, items: &[OrderItem]) -> Result<Order> {
        let mut tx = self.pool.begin().await?;

        let created_order = sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO orders (id, buyer_id, status, total_amount, shipping_address)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, buyer_id, status, total_amount, shipping_address, created_at, updated_at
            "#,
        )
        .bind(order.id)
        .bind(order.buyer_id)
        .bind(&order.status)
        .bind(order.total_amount)
        .bind(&order.shipping_address)
        .fetch_one(&mut *tx)
        .await?;

        for item in items {
            sqlx::query(
                r#"
                INSERT INTO order_items (id, order_id, product_id, quantity, price_at_purchase)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(item.id)
            .bind(item.order_id)
            .bind(item.product_id)
            .bind(item.quantity)
            .bind(item.price_at_purchase)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(created_order)
    }

    async fn find_by_buyer(&self, buyer_id: Uuid) -> Result<Vec<Order>> {
        let rows = sqlx::query_as::<_, Order>(
            r#"
            SELECT id, buyer_id, status, total_amount, shipping_address, created_at, updated_at
            FROM orders WHERE buyer_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(buyer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Order>> {
        let row = sqlx::query_as::<_, Order>(
            r#"
            SELECT id, buyer_id, status, total_amount, shipping_address, created_at, updated_at
            FROM orders WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
}
