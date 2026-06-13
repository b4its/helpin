use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::transaction::PosTransaction;

#[derive(Clone)]
pub struct PostgresTransactionRepository {
    pool: PgPool,
}

impl PostgresTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, transaction: &PosTransaction) -> Result<PosTransaction> {
        let row = sqlx::query_as::<_, PosTransaction>(
            r#"
            INSERT INTO pos_transactions (id, cashier_id, items, subtotal, tax, total, amount_tendered, change_amount)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, cashier_id, items, subtotal, tax, total, amount_tendered, change_amount, created_at
            "#,
        )
        .bind(transaction.id)
        .bind(transaction.cashier_id)
        .bind(&transaction.items)
        .bind(transaction.subtotal)
        .bind(transaction.tax)
        .bind(transaction.total)
        .bind(transaction.amount_tendered)
        .bind(transaction.change_amount)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_cashier(&self, cashier_id: Uuid) -> Result<Vec<PosTransaction>> {
        let rows = sqlx::query_as::<_, PosTransaction>(
            r#"
            SELECT id, cashier_id, items, subtotal, tax, total, amount_tendered, change_amount, created_at
            FROM pos_transactions WHERE cashier_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(cashier_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PosTransaction>> {
        let row = sqlx::query_as::<_, PosTransaction>(
            r#"
            SELECT id, cashier_id, items, subtotal, tax, total, amount_tendered, change_amount, created_at
            FROM pos_transactions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
}
