use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::domain::entities::financial_record::FinancialRecord;

#[derive(Clone)]
pub struct PostgresFinancialRepository {
    pool: PgPool,
}

impl PostgresFinancialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_record(&self, record: &FinancialRecord) -> Result<FinancialRecord> {
        let row = sqlx::query_as::<_, FinancialRecord>(
            r#"
            INSERT INTO financial_records (id, record_type, amount, category, description, reference_id, balance_after)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, record_type, amount, category, description, reference_id, balance_after, recorded_at
            "#,
        )
        .bind(record.id)
        .bind(&record.record_type)
        .bind(record.amount)
        .bind(&record.category)
        .bind(&record.description)
        .bind(record.reference_id)
        .bind(record.balance_after)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_balance(&self) -> Result<i64> {
        let row: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT balance_after FROM financial_records
            ORDER BY recorded_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.0).unwrap_or(0))
    }

    pub async fn get_report_by_period(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<FinancialRecord>> {
        let rows = sqlx::query_as::<_, FinancialRecord>(
            r#"
            SELECT id, record_type, amount, category, description, reference_id, balance_after, recorded_at
            FROM financial_records
            WHERE recorded_at BETWEEN $1 AND $2
            ORDER BY recorded_at ASC
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
