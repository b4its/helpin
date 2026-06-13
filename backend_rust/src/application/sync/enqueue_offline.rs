use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::sync_queue::SyncQueueItem;
use crate::infrastructure::repositories::sync_queue_repo::PostgresSyncQueueRepository;
use crate::infrastructure::services::crypto_service::Ed25519CryptoService;
use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::domain::traits::repository::{SyncQueueRepository, UserRepository};
use crate::domain::traits::service::CryptoService;

pub struct EnqueueInput {
    pub user_id: Uuid,
    pub payload: serde_json::Value,
    pub signature: String,
}

pub async fn enqueue_offline(
    input: EnqueueInput,
    sync_repo: &PostgresSyncQueueRepository,
    user_repo: &PostgresUserRepository,
    crypto_service: &Ed25519CryptoService,
) -> Result<SyncQueueItem> {
    // Get user's public key (derived from private key)
    let user = user_repo.find_by_id(input.user_id).await?
        .ok_or_else(|| anyhow!("User tidak ditemukan"))?;

    let private_key = user.private_key
        .ok_or_else(|| anyhow!("User tidak memiliki kunci kriptografis"))?;

    // Derive public key from private key to verify signature
    let key_bytes = hex::decode(&private_key)
        .map_err(|e| anyhow!("Invalid private key: {}", e))?;
    let key_array: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| anyhow!("Private key must be 32 bytes"))?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&key_array);
    let public_key_hex = hex::encode(signing_key.verifying_key().to_bytes());

    // Verify signature
    let payload_bytes = serde_json::to_vec(&input.payload)?;
    let is_valid = crypto_service.verify_signature(&public_key_hex, &payload_bytes, &input.signature)?;

    if !is_valid {
        return Err(anyhow!("Tanda tangan tidak valid"));
    }

    // Save to sync_queue with status "pending"
    let item = SyncQueueItem {
        id: Uuid::new_v4(),
        user_id: input.user_id,
        payload: input.payload,
        signature: input.signature,
        status: Some("pending".to_string()),
        error_message: None,
        created_at: None,
        synced_at: None,
    };

    let saved = sync_repo.enqueue(&item).await?;
    Ok(saved)
}
