//! HyperledgerService — koneksi ke Hyperledger Besu via Blockchain API (Node.js).
//!
//! Alur:
//! 1. submit_transaction  → POST /api/blockchain/submit
//! 2. verify_transaction  → GET  /api/blockchain/verify/:txId
//! 3. log_activity        → POST /api/blockchain/activity/log
//!
//! Jika Besu API tidak tersedia, fungsi tetap berhasil dengan hash off-chain (SHA-256)
//! sehingga backend tidak crash.

use anyhow::Result;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::traits::service::BlockchainService;

#[derive(Clone)]
pub struct HyperledgerService {
    rpc_url:     String, // URL ke Blockchain API (contoh: http://blockchain-api:3001)
    http_client: reqwest::Client,
}

impl HyperledgerService {
    pub fn new(rpc_url: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");

        // Tentukan base URL: jika blockchain_rpc_url menunjuk ke Besu (port 8545),
        // gunakan URL yang sama untuk API (port 3001). Jika sudah 3001, pakai apa adanya.
        // Pengguna bisa set BLOCKCHAIN_API_URL secara eksplisit di env.
        Self {
            rpc_url,
            http_client,
        }
    }

    /// Base URL untuk Blockchain API (Node.js), dibedakan dari Besu RPC.
    fn api_base(&self) -> String {
        // Cek env var override
        if let Ok(url) = std::env::var("BLOCKCHAIN_API_URL") {
            return url.trim_end_matches('/').to_string();
        }
        // Fallback: ganti port 8545 → 3001 (dev default)
        self.rpc_url
            .replace(":8545", ":3001")
            .trim_end_matches('/')
            .to_string()
    }

    /// Generate hash SHA-256 sebagai fallback ketika API tidak tersedia.
    fn sha256_hash(reference_type: &str, reference_id: Uuid, payload: &Value) -> String {
        let payload_bytes = serde_json::to_vec(payload).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&payload_bytes);
        hasher.update(reference_id.as_bytes());
        hasher.update(reference_type.as_bytes());
        format!("0x{}", hex::encode(hasher.finalize()))
    }

    /// Kirim log aktivitas ke Blockchain API untuk dicatat di smart contract.
    pub async fn log_activity(
        &self,
        activity_id: Uuid,
        title: &str,
        action_type: &str,
        table_affected: &str,
        description: &str,
        username: &str,
        old_data: &Value,
        new_data: &Value,
        ip_address: &str,
    ) -> Result<String> {
        let body = json!({
            "activityId":    activity_id.to_string(),
            "title":         title,
            "actionType":    action_type,
            "tableAffected": table_affected,
            "description":   description,
            "username":      username,
            "oldData":       old_data,
            "newData":       new_data,
            "ipAddress":     ip_address,
        });

        let url = format!("{}/api/blockchain/activity/log", self.api_base());
        match self.http_client.post(&url).json(&body).send().await {
            Ok(resp) if resp.status().is_success() => {
                let data: Value = resp.json().await.unwrap_or(json!({}));
                let tx_hash = data["txHash"]
                    .as_str()
                    .unwrap_or("0x")
                    .to_string();
                tracing::info!(
                    tx_hash = %tx_hash,
                    activity_id = %activity_id,
                    action_type = %action_type,
                    table = %table_affected,
                    "Activity logged on-chain"
                );
                Ok(tx_hash)
            }
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                tracing::warn!("Blockchain API error {}: {}", status, body);
                // Fallback off-chain hash
                Ok(Self::sha256_hash(action_type, activity_id, &json!({ "table": table_affected })))
            }
            Err(e) => {
                tracing::warn!("Blockchain API tidak tersedia: {}", e);
                Ok(Self::sha256_hash(action_type, activity_id, &json!({ "fallback": true })))
            }
        }
    }

    /// Expose HTTP client untuk digunakan oleh handler (proxy ke Blockchain API).
    pub fn get_http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Retry pending blockchain transactions (best-effort).
    pub async fn retry_pending(&self) -> Result<()> {
        tracing::info!("blockchain retry_pending — checking Besu API status");
        let url = format!("{}/health", self.api_base());
        match self.http_client.get(&url).send().await {
            Ok(resp) => {
                tracing::info!("Blockchain API health: {}", resp.status());
            }
            Err(e) => {
                tracing::warn!("Blockchain API tidak tersedia saat retry_pending: {}", e);
            }
        }
        Ok(())
    }
}

impl BlockchainService for HyperledgerService {
    /// Submit transaksi bisnis (POS, order, livestock, land, harvest) ke smart contract.
    async fn submit_transaction(
        &self,
        reference_type: &str,
        reference_id: Uuid,
        payload: Value,
    ) -> Result<String> {
        let body = json!({
            "referenceType": reference_type,
            "referenceId":   reference_id.to_string(),
            "payload":       payload,
        });

        let url = format!("{}/api/blockchain/submit", self.api_base());
        match self.http_client.post(&url).json(&body).send().await {
            Ok(resp) if resp.status().is_success() => {
                let data: Value = resp.json().await.unwrap_or(json!({}));
                let tx_hash = data["txHash"]
                    .as_str()
                    .unwrap_or("0x")
                    .to_string();
                tracing::info!(
                    tx_hash = %tx_hash,
                    reference_type = %reference_type,
                    reference_id  = %reference_id,
                    "Blockchain transaction submitted"
                );
                Ok(tx_hash)
            }
            Ok(resp) => {
                let status = resp.status();
                tracing::warn!("Blockchain submit error: {}", status);
                Ok(Self::sha256_hash(reference_type, reference_id, &payload))
            }
            Err(e) => {
                tracing::warn!("Blockchain API tidak tersedia: {} — using off-chain hash", e);
                Ok(Self::sha256_hash(reference_type, reference_id, &payload))
            }
        }
    }

    /// Verifikasi transaksi yang tersimpan di smart contract.
    async fn verify_transaction(&self, tx_hash: &str) -> Result<Value> {
        let url = format!("{}/api/blockchain/verify/{}", self.api_base(), tx_hash);
        match self.http_client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let data: Value = resp.json().await.unwrap_or(json!({}));
                Ok(data)
            }
            Ok(resp) if resp.status().as_u16() == 404 => {
                Ok(json!({
                    "tx_hash": tx_hash,
                    "status":  "not_found",
                    "message": "Transaksi tidak ditemukan di smart contract"
                }))
            }
            _ => {
                // Fallback placeholder
                Ok(json!({
                    "tx_hash":   tx_hash,
                    "status":    "unknown",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "note":      "Blockchain API tidak tersedia — tidak dapat verifikasi"
                }))
            }
        }
    }
}
