//! Audit trail / log aktivitas sistem.
//!
//! Setiap aktivitas penting (CRUD pada panel mana pun) dicatat ke:
//! 1. MongoDB  — untuk query cepat & dashboard admin
//! 2. Hyperledger Besu — untuk immutability & tamper-proof (via Blockchain API)
//!
//! Data yang tersimpan: judul, tipe aksi, tabel terdampak, username, deskripsi,
//! data lama (old), data baru (new), IP, user-agent, koordinat (lat/long),
//! dan blockchain tx hash.

use serde_json::{json, Value};
use uuid::Uuid;

use crate::infrastructure::middleware::client_meta::ClientMeta;
use crate::infrastructure::services::blockchain_service::HyperledgerService;
use crate::infrastructure::services::mongo_service::MongoService;

pub struct ActivityCtx {
    pub title: String,
    pub action_type: String, // CREATED | UPDATED | DELETED | LOGIN | PAYMENT | dst
    pub table_affected: String,
    pub description: String,
    pub username: String,
    pub old_data: Value,
    pub new_data: Value,
}

/// Catat aktivitas (best-effort; tidak menggagalkan operasi utama).
pub async fn record(
    mongo: &MongoService,
    blockchain: &HyperledgerService,
    meta: &ClientMeta,
    ctx: ActivityCtx,
) {
    let activity_id = Uuid::new_v4();

    // ── 1. Submit ke Hyperledger Besu (on-chain, via Blockchain API) ──
    //    Gunakan log_activity untuk mencatat ke smart contract.
    //    Fallback ke submit_transaction jika log_activity tidak tersedia.
    let blockchain_hash = blockchain
        .log_activity(
            activity_id,
            &ctx.title,
            &ctx.action_type,
            &ctx.table_affected,
            &ctx.description,
            &ctx.username,
            &ctx.old_data,
            &ctx.new_data,
            meta.ip_address.as_deref().unwrap_or(""),
        )
        .await
        .unwrap_or_else(|_| "0x".to_string());

    // ── 2. Simpan ke MongoDB dengan blockchain_hash ───────────────
    let doc = json!({
        "activity_id":    activity_id.to_string(),
        "title":          ctx.title,
        "action_type":    ctx.action_type,
        "table_affected": ctx.table_affected,
        "description":    ctx.description,
        "username":       ctx.username,
        "old_data":       ctx.old_data,
        "new_data":       ctx.new_data,
        "ip_address":     meta.ip_address,
        "user_agent":     meta.user_agent,
        "lat":            meta.lat,
        "long":           meta.long,
        "blockchain_hash": blockchain_hash,
    });

    if let Err(e) = mongo.log_activity(&doc).await {
        tracing::warn!("Gagal mencatat aktivitas ke MongoDB: {}", e);
    }
}
