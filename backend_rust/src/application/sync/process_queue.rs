use anyhow::{anyhow, Result};
use serde_json::Value;
use uuid::Uuid;

use crate::infrastructure::repositories::sync_queue_repo::PostgresSyncQueueRepository;
use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::infrastructure::services::crypto_service::Ed25519CryptoService;
use crate::infrastructure::services::blockchain_service::HyperledgerService;
use crate::domain::traits::repository::{SyncQueueRepository, UserRepository};
use crate::domain::traits::service::{BlockchainService, CryptoService};

pub struct ProcessQueueOutput {
    pub processed: u32,
    pub succeeded: u32,
    pub failed: u32,
}

pub async fn process_queue(
    sync_repo: &PostgresSyncQueueRepository,
    user_repo: &PostgresUserRepository,
    crypto_service: &Ed25519CryptoService,
    blockchain_service: &HyperledgerService,
) -> Result<ProcessQueueOutput> {
    // Get all pending items (FIFO by created_at ASC)
    let pending_items = sync_repo.get_pending().await?;

    let mut processed: u32 = 0;
    let mut succeeded: u32 = 0;
    let mut failed: u32 = 0;

    for item in pending_items {
        processed += 1;

        match process_single_item(&item.id, &item.user_id, &item.payload, &item.signature, user_repo, crypto_service, blockchain_service).await {
            Ok(_) => {
                sync_repo.update_status(item.id, "synced", None).await?;
                succeeded += 1;
            }
            Err(e) => {
                sync_repo.update_status(item.id, "failed", Some(&e.to_string())).await?;
                failed += 1;
            }
        }
    }

    Ok(ProcessQueueOutput { processed, succeeded, failed })
}

async fn process_single_item(
    item_id: &Uuid,
    user_id: &Uuid,
    payload: &Value,
    signature: &str,
    user_repo: &PostgresUserRepository,
    crypto_service: &Ed25519CryptoService,
    blockchain_service: &HyperledgerService,
) -> Result<()> {
    // Get user to verify signature
    let user = user_repo.find_by_id(*user_id).await?
        .ok_or_else(|| anyhow!("User tidak ditemukan"))?;

    let private_key = user.private_key
        .ok_or_else(|| anyhow!("User tidak memiliki kunci kriptografis"))?;

    // Derive public key from private key
    let key_bytes = hex::decode(&private_key)
        .map_err(|e| anyhow!("Invalid private key: {}", e))?;
    let key_array: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| anyhow!("Private key must be 32 bytes"))?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&key_array);
    let public_key_hex = hex::encode(signing_key.verifying_key().to_bytes());

    // Verify signature
    let payload_bytes = serde_json::to_vec(payload)?;
    let is_valid = crypto_service.verify_signature(&public_key_hex, &payload_bytes, signature)?;

    if !is_valid {
        return Err(anyhow!("Tanda tangan tidak valid"));
    }

    // Determine transaction value from payload for blockchain threshold
    let transaction_value = payload.get("amount")
        .or_else(|| payload.get("total"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // If transaction value > 100000, trigger blockchain submission
    if transaction_value > 100_000 {
        let reference_type = payload.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("sync_transaction");

        blockchain_service.submit_transaction(
            reference_type,
            *item_id,
            payload.clone(),
        ).await?;
    }

    Ok(())
}
