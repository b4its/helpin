//! Audit trail / log aktivitas sistem.
//!
//! Semua aktivitas penting (CRUD pada panel mana pun) dicatat ke MongoDB
//! lengkap dengan: judul, tipe aksi, tabel terdampak, nama pengguna, deskripsi,
//! data lama (old), data baru (new), IP, user-agent, koordinat (lat/long),
//! dan blockchain hash untuk integritas.

use serde_json::{json, Value};
use uuid::Uuid;

use crate::domain::traits::service::BlockchainService;
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
    // Payload untuk hashing integritas
    let payload = json!({
        "title": ctx.title,
        "action_type": ctx.action_type,
        "table_affected": ctx.table_affected,
        "username": ctx.username,
        "old_data": ctx.old_data,
        "new_data": ctx.new_data,
    });

    // Blockchain hash (placeholder SHA256)
    let blockchain_hash = blockchain
        .submit_transaction("activity", Uuid::new_v4(), payload.clone())
        .await
        .unwrap_or_else(|_| "0x".to_string());

    let doc = json!({
        "title": ctx.title,
        "action_type": ctx.action_type,
        "table_affected": ctx.table_affected,
        "description": ctx.description,
        "username": ctx.username,
        "old_data": ctx.old_data,
        "new_data": ctx.new_data,
        "ip_address": meta.ip_address,
        "user_agent": meta.user_agent,
        "lat": meta.lat,
        "long": meta.long,
        "blockchain_hash": blockchain_hash,
    });

    if let Err(e) = mongo.log_activity(&doc).await {
        tracing::warn!("Gagal mencatat aktivitas: {}", e);
    }
}
