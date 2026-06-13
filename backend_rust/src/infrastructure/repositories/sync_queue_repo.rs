use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::sync_queue::SyncQueueItem;
use crate::domain::traits::repository::SyncQueueRepository;

#[derive(Clone)]
pub struct PostgresSyncQueueRepository {
    pool: PgPool,
}

impl PostgresSyncQueueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SyncQueueRepository for PostgresSyncQueueRepository {
    async fn enqueue(&self, item: &SyncQueueItem) -> Result<SyncQueueItem> {
        let row = sqlx::query_as::<_, SyncQueueItem>(
            r#"
            INSERT INTO sync_queue (id, user_id, payload, signature, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, payload, signature, status, error_message, created_at, synced_at
            "#,
        )
        .bind(item.id)
        .bind(item.user_id)
        .bind(&item.payload)
        .bind(&item.signature)
        .bind(&item.status)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn get_pending(&self) -> Result<Vec<SyncQueueItem>> {
        let rows = sqlx::query_as::<_, SyncQueueItem>(
            r#"
            SELECT id, user_id, payload, signature, status, error_message, created_at, synced_at
            FROM sync_queue
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE sync_queue
            SET status = $2, error_message = $3, synced_at = CASE WHEN $2 = 'synced' THEN NOW() ELSE synced_at END
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(error_message)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
