//! Service MongoDB untuk menyimpan data treatment lahan yang fleksibel
//! (pupuk + jumlah, alat pertanian, catatan) yang dipakai untuk prediksi panen.

use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::{Client, Database};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone)]
pub struct MongoService {
    db: Option<Database>,
}

impl MongoService {
    /// Coba konek ke MongoDB. Jika gagal, service tetap berjalan (treatment dinonaktifkan).
    pub async fn connect(uri: &str) -> Self {
        match Client::with_uri_str(uri).await {
            Ok(client) => {
                tracing::info!("MongoDB client created ({})", uri);
                Self {
                    db: Some(client.database("helpin")),
                }
            }
            Err(e) => {
                tracing::warn!("MongoDB connection failed: {} (treatment dinonaktifkan)", e);
                Self { db: None }
            }
        }
    }

    pub fn is_available(&self) -> bool {
        self.db.is_some()
    }

    /// Simpan satu dokumen treatment untuk sebuah lahan.
    /// `data` adalah JSON fleksibel: { fertilizers: [...], tools: [...], notes, source }.
    pub async fn save_treatment(
        &self,
        land_id: Uuid,
        owner_id: Uuid,
        data: &Value,
    ) -> anyhow::Result<()> {
        let Some(db) = &self.db else {
            return Ok(()); // Mongo tidak tersedia — lewati diam-diam
        };
        let coll = db.collection::<Document>("land_treatments");
        let mut document = doc! {
            "land_id": land_id.to_string(),
            "owner_id": owner_id.to_string(),
            "created_at": BsonDateTime::now(),
        };
        if let Ok(Bson::Document(d)) = mongodb::bson::to_bson(data) {
            document.insert("data", d);
        }
        coll.insert_one(document).await?;
        Ok(())
    }

    /// Ambil semua treatment untuk sebuah lahan (terbaru dulu) sebagai JSON.
    pub async fn list_treatments(&self, land_id: Uuid) -> anyhow::Result<Vec<Value>> {
        let mut out = Vec::new();
        let Some(db) = &self.db else { return Ok(out); };
        let coll = db.collection::<Document>("land_treatments");
        let mut cursor = coll
            .find(doc! { "land_id": land_id.to_string() })
            .sort(doc! { "created_at": -1 })
            .await?;
        while let Some(d) = cursor.try_next().await? {
            out.push(Bson::Document(d).into_relaxed_extjson());
        }
        Ok(out)
    }

    /// Simpan dokumen fleksibel ke koleksi mana pun, di-scope dengan ref_id (land/pen) + owner.
    pub async fn save_doc(
        &self,
        collection: &str,
        ref_id: Uuid,
        owner_id: Uuid,
        data: &Value,
    ) -> anyhow::Result<()> {
        let Some(db) = &self.db else { return Ok(()); };
        let coll = db.collection::<Document>(collection);
        let mut document = doc! {
            "ref_id": ref_id.to_string(),
            "land_id": ref_id.to_string(),
            "pen_id": ref_id.to_string(),
            "owner_id": owner_id.to_string(),
            "created_at": BsonDateTime::now(),
        };
        if let Ok(Bson::Document(d)) = mongodb::bson::to_bson(data) {
            document.insert("data", d);
        }
        coll.insert_one(document).await?;
        Ok(())
    }

    /// Ambil dokumen dari koleksi tertentu untuk ref_id (terbaru dulu).
    pub async fn list_docs(&self, collection: &str, ref_id: Uuid) -> anyhow::Result<Vec<Value>> {
        let mut out = Vec::new();
        let Some(db) = &self.db else { return Ok(out); };
        let coll = db.collection::<Document>(collection);
        let mut cursor = coll
            .find(doc! { "ref_id": ref_id.to_string() })
            .sort(doc! { "created_at": -1 })
            .await?;
        while let Some(d) = cursor.try_next().await? {
            out.push(Bson::Document(d).into_relaxed_extjson());
        }
        Ok(out)
    }

    /// Simpan satu log aktivitas (audit trail) ke koleksi `activity_logs`.
    pub async fn log_activity(&self, activity: &Value) -> anyhow::Result<()> {
        let Some(db) = &self.db else { return Ok(()); };
        let coll = db.collection::<Document>("activity_logs");
        let mut document = doc! { "created_at": BsonDateTime::now() };
        if let Ok(Bson::Document(d)) = mongodb::bson::to_bson(activity) {
            for (k, v) in d {
                document.insert(k, v);
            }
        }
        coll.insert_one(document).await?;
        Ok(())
    }

    /// Ambil daftar aktivitas terbaru (audit trail).
    pub async fn list_activities(&self, limit: i64) -> anyhow::Result<Vec<Value>> {
        let mut out = Vec::new();
        let Some(db) = &self.db else { return Ok(out); };
        let coll = db.collection::<Document>("activity_logs");
        let mut cursor = coll
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .await?;
        while let Some(d) = cursor.try_next().await? {
            out.push(Bson::Document(d).into_relaxed_extjson());
        }
        Ok(out)
    }

    /// Ringkasan agregat treatment untuk prediksi:
    /// (jumlah_event_pupuk, total_qty_pupuk, jumlah_event_alat).
    pub async fn treatment_aggregate(&self, land_id: Uuid) -> (usize, f64, usize) {
        let mut fert_count = 0usize;
        let mut fert_qty = 0.0f64;
        let mut tool_count = 0usize;

        let Some(db) = &self.db else {
            return (0, 0.0, 0);
        };
        let coll = db.collection::<Document>("land_treatments");
        let cursor = coll.find(doc! { "land_id": land_id.to_string() }).await;
        if let Ok(mut cursor) = cursor {
            while let Ok(Some(d)) = cursor.try_next().await {
                if let Ok(data) = d.get_document("data") {
                    if let Ok(ferts) = data.get_array("fertilizers") {
                        for f in ferts {
                            fert_count += 1;
                            if let Bson::Document(fd) = f {
                                if let Some(q) = fd.get("quantity").and_then(bson_to_f64) {
                                    fert_qty += q;
                                }
                            }
                        }
                    }
                    if let Ok(tools) = data.get_array("tools") {
                        tool_count += tools.len();
                    }
                }
            }
        }
        (fert_count, fert_qty, tool_count)
    }
}

fn bson_to_f64(b: &Bson) -> Option<f64> {
    match b {
        Bson::Double(d) => Some(*d),
        Bson::Int32(i) => Some(*i as f64),
        Bson::Int64(i) => Some(*i as f64),
        Bson::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}
