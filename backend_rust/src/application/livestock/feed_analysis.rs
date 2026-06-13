//! Analisa pakan untuk ternak di sebuah kandang.
//!
//! Konsep:
//! - ONLINE  : pakai OpenRouter (AI agent) jika OPENROUTER_API_KEY tersedia.
//! - OFFLINE : ensemble/rule-based dari kandungan nutrisi produk + inventori "Pakan"
//!             + data ternak (kategori, ras, berat, usia, kesehatan).
//!
//! Hasil: daftar top-tier pakan terbaik + proyeksi kualitas ternak.

use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::infrastructure::services::mongo_service::MongoService;

struct CandidateFeed {
    name: String,
    source: String, // "produk" | "inventori"
    price: Option<i64>,
    protein: f64,
    energy_tdn: f64,
    fiber: f64,
    fat: f64,
    calcium: f64,
    phosphorus: f64,
}

/// Estimasi kandungan nutrisi dari nama pakan (untuk pakan tanpa data nutrisi eksplisit).
fn estimate_nutrition(name: &str) -> (f64, f64, f64, f64, f64, f64) {
    // (protein, energy_tdn, fiber, fat, calcium, phosphorus)
    let n = name.to_lowercase();
    if n.contains("konsentrat") || n.contains("concentrate") {
        (18.0, 72.0, 9.0, 5.0, 1.2, 0.7)
    } else if n.contains("bungkil") || n.contains("kedelai") || n.contains("soy") {
        (40.0, 78.0, 7.0, 6.0, 0.3, 0.6)
    } else if n.contains("dedak") || n.contains("bekatul") {
        (13.0, 65.0, 12.0, 8.0, 0.1, 1.2)
    } else if n.contains("jagung") || n.contains("corn") {
        (9.0, 80.0, 3.0, 4.0, 0.1, 0.3)
    } else if n.contains("silase") || n.contains("silage") {
        (11.0, 65.0, 25.0, 3.0, 0.4, 0.3)
    } else if n.contains("rumput") || n.contains("hijauan") || n.contains("gajah") || n.contains("odot") {
        (10.0, 56.0, 30.0, 2.5, 0.5, 0.3)
    } else if n.contains("ampas") || n.contains("tahu") {
        (23.0, 70.0, 14.0, 5.0, 0.4, 0.3)
    } else {
        (13.0, 62.0, 18.0, 4.0, 0.6, 0.4)
    }
}

fn num(v: &Value, key: &str, default: f64) -> f64 {
    v.get(key).and_then(|x| x.as_f64()).unwrap_or(default)
}

/// Klasifikasi kasar jenis ternak → (target_protein%, target_tdn%, base_adg kg/hari)
fn livestock_profile(category: &str) -> (f64, f64, f64) {
    let c = category.to_lowercase();
    if c.contains("unggas") || c.contains("aves") || c.contains("ayam") || c.contains("bebek") || c.contains("itik") || c.contains("puyuh") {
        (20.0, 70.0, 0.05) // unggas: protein tinggi, ADG kecil
    } else if c.contains("serangga") || c.contains("insect") || c.contains("maggot") || c.contains("jangkrik") {
        (40.0, 60.0, 0.005) // serangga: protein sangat tinggi
    } else if c.contains("rumin") || c.contains("sapi") || c.contains("kerbau") {
        (15.0, 68.0, 1.0)
    } else if c.contains("kambing") || c.contains("domba") {
        (14.0, 62.0, 0.12)
    } else if c.contains("mamalia") || c.contains("babi") || c.contains("kelinci") {
        (16.0, 66.0, 0.5)
    } else {
        (14.0, 64.0, 0.4)
    }
}

/// Target kebutuhan nutrisi berdasarkan kategori ternak, berat, dan usia.
fn target_nutrition(category: &str, avg_weight: f64, avg_age_months: f64) -> (f64, f64) {
    let (mut protein, mut tdn, _) = livestock_profile(category);
    // Ternak muda (pertumbuhan) butuh protein lebih tinggi
    if avg_age_months > 0.0 && avg_age_months < 12.0 {
        protein += 3.0;
        tdn += 2.0;
    }
    // Ruminansia ringan (fase pertumbuhan) butuh protein lebih
    if avg_weight > 0.0 && avg_weight < 250.0 && (category.to_lowercase().contains("sapi") || category.to_lowercase().contains("rumin")) {
        protein += 1.5;
    }
    (protein, tdn)
}

/// Skor kecocokan pakan terhadap target (0..100) + proyeksi ADG (kg/hari).
fn score_feed(
    category: &str,
    target_protein: f64,
    target_tdn: f64,
    f: &CandidateFeed,
) -> (f64, f64) {
    let protein_gap = (f.protein - target_protein).abs();
    let tdn_gap = (f.energy_tdn - target_tdn).abs();
    let mut compat = 100.0 - protein_gap * 2.5 - tdn_gap * 1.2;
    // Serat terlalu tinggi menurunkan kecocokan (kurang energi)
    if f.fiber > 28.0 {
        compat -= (f.fiber - 28.0) * 0.8;
    }
    let compat = compat.clamp(0.0, 100.0);

    // Proyeksi ADG dasar per kategori, diskala kualitas pakan
    let base_adg = livestock_profile(category).2;
    let quality_factor = (f.energy_tdn / 70.0) * (f.protein / target_protein).min(1.4);
    let adg = (base_adg * quality_factor * (compat / 100.0)).max(0.0);
    (compat, (adg * 1000.0).round() / 1000.0)
}

fn grade_from_score(score: f64) -> &'static str {
    if score >= 80.0 {
        "A"
    } else if score >= 65.0 {
        "B"
    } else {
        "C"
    }
}

/// Jalankan analisa pakan untuk kandang. Menyimpan hasil ke MongoDB & mengembalikan JSON.
pub async fn analyze_pen_feed(
    pool: &PgPool,
    mongo: &MongoService,
    config: &Config,
    pen_id: Uuid,
    owner_id: Uuid,
) -> Result<Value> {
    // Data kandang
    let pen = sqlx::query_as::<_, (String, i32)>("SELECT name, capacity FROM pens WHERE id = $1")
        .bind(pen_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Kandang tidak ditemukan"))?;
    let (pen_name, _capacity) = pen;

    // Ternak di kandang
    let animals = sqlx::query_as::<_, (String, String, sqlx::types::BigDecimal, Option<i32>, Option<i32>)>(
        "SELECT category, breed, weight, age_months, health_score FROM livestock WHERE pen_id = $1 AND owner_id = $2",
    )
    .bind(pen_id)
    .bind(owner_id)
    .fetch_all(pool)
    .await?;

    if animals.is_empty() {
        return Err(anyhow!("Kandang kosong — tambahkan ternak ke kandang sebelum analisa pakan"));
    }

    let count = animals.len() as f64;
    let avg_weight = animals.iter().map(|a| a.2.to_f64().unwrap_or(0.0)).sum::<f64>() / count;
    let ages: Vec<f64> = animals.iter().filter_map(|a| a.3.map(|x| x as f64)).collect();
    let avg_age = if ages.is_empty() { 0.0 } else { ages.iter().sum::<f64>() / ages.len() as f64 };
    let avg_health = animals.iter().filter_map(|a| a.4.map(|x| x as f64)).sum::<f64>()
        / animals.iter().filter(|a| a.4.is_some()).count().max(1) as f64;
    // Kategori dominan
    let dominant_category = {
        use std::collections::HashMap;
        let mut m: HashMap<&str, usize> = HashMap::new();
        for a in &animals {
            *m.entry(a.0.as_str()).or_insert(0) += 1;
        }
        m.into_iter().max_by_key(|(_, c)| *c).map(|(k, _)| k.to_string()).unwrap_or_else(|| "Sapi".to_string())
    };
    let breeds: Vec<String> = {
        let mut b: Vec<String> = animals.iter().map(|a| a.1.clone()).collect();
        b.sort();
        b.dedup();
        b
    };

    let (target_protein, target_tdn) = target_nutrition(&dominant_category, avg_weight, avg_age);

    // Kandidat pakan dari PRODUK (kandungan nutrisi) + INVENTORI kategori "Pakan"
    let mut candidates: Vec<CandidateFeed> = Vec::new();

    // Produk: yang punya nutrition ATAU product_type/kategori mengindikasikan pakan
    let products = sqlx::query_as::<_, (String, i64, Option<Value>, Option<String>)>(
        "SELECT name, price, nutrition, product_type FROM products",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    for (name, price, nutrition, ptype) in products {
        let is_feed_type = ptype.as_deref().map(|t| {
            let t = t.to_lowercase();
            t.contains("pakan") || t.contains("feed")
        }).unwrap_or(false);
        if let Some(n) = nutrition {
            candidates.push(CandidateFeed {
                name,
                source: "produk".to_string(),
                price: Some(price),
                protein: num(&n, "protein", 0.0),
                energy_tdn: num(&n, "energy_tdn", num(&n, "tdn", 0.0)),
                fiber: num(&n, "fiber", 15.0),
                fat: num(&n, "fat", 4.0),
                calcium: num(&n, "calcium", 0.5),
                phosphorus: num(&n, "phosphorus", 0.4),
            });
        } else if is_feed_type {
            let (p, t, fi, fa, c, ph) = estimate_nutrition(&name);
            candidates.push(CandidateFeed { name, source: "produk".to_string(), price: Some(price), protein: p, energy_tdn: t, fiber: fi, fat: fa, calcium: c, phosphorus: ph });
        }
    }

    // Inventori kategori "Pakan" milik peternak
    let inv = sqlx::query_as::<_, (String,)>(
        "SELECT name FROM inventories WHERE owner_id = $1 AND category = 'Pakan'",
    )
    .bind(owner_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    for (name,) in inv {
        let (p, t, fi, fa, c, ph) = estimate_nutrition(&name);
        candidates.push(CandidateFeed { name, source: "inventori".to_string(), price: None, protein: p, energy_tdn: t, fiber: fi, fat: fa, calcium: c, phosphorus: ph });
    }

    // Skor & ranking (ensemble offline)
    let mut scored: Vec<(f64, f64, &CandidateFeed)> = candidates
        .iter()
        .map(|f| {
            let (compat, adg) = score_feed(&dominant_category, target_protein, target_tdn, f);
            (compat, adg, f)
        })
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let top_feeds: Vec<Value> = scored
        .iter()
        .take(5)
        .map(|(compat, adg, f)| {
            json!({
                "name": f.name,
                "source": f.source,
                "compatibility_percent": (compat * 10.0).round() / 10.0,
                "projected_adg_kg": adg,
                "price": f.price,
                "nutrition": {
                    "protein": f.protein,
                    "energy_tdn": f.energy_tdn,
                    "fiber": f.fiber,
                    "fat": f.fat,
                    "calcium": f.calcium,
                    "phosphorus": f.phosphorus
                },
                "grade": grade_from_score(*compat),
            })
        })
        .collect();

    let best = scored.first();
    let best_compat = best.map(|b| b.0).unwrap_or(0.0);
    let best_adg = best.map(|b| b.1).unwrap_or(0.0);

    // Proyeksi kualitas ternak jika diberi pakan terbaik
    let fcr = if best_adg > 0.0 {
        // FCR kasar: kebutuhan bahan kering harian / ADG
        let dmi = avg_weight * 0.03; // ~3% bobot badan
        ((dmi / best_adg) * 10.0).round() / 10.0
    } else {
        0.0
    };
    let target_weight = ((avg_weight + best_adg * 90.0) * 10.0).round() / 10.0; // proyeksi 90 hari
    let quality_grade = grade_from_score(best_compat);
    let body_condition = if avg_health >= 80.0 { "Prima" } else if avg_health >= 60.0 { "Baik" } else { "Perlu Perhatian" };

    let context = json!({
        "pen_name": pen_name,
        "livestock_count": animals.len(),
        "dominant_category": dominant_category,
        "breeds": breeds,
        "avg_weight_kg": (avg_weight * 10.0).round() / 10.0,
        "avg_age_months": avg_age,
        "avg_health_score": avg_health,
        "target_nutrition": { "protein": target_protein, "energy_tdn": target_tdn },
        "candidate_feeds": scored.iter().take(8).map(|(c, a, f)| json!({
            "name": f.name, "source": f.source, "protein": f.protein, "energy_tdn": f.energy_tdn,
            "compatibility_percent": (c*10.0).round()/10.0, "projected_adg_kg": a
        })).collect::<Vec<_>>(),
    });

    // Coba OpenRouter (online) untuk hasil lebih kaya
    let mut analysis_source = "ensemble".to_string();
    let mut ai_block: Option<Value> = None;
    if !config.openrouter_api_key.is_empty() && !candidates.is_empty() {
        match call_openrouter_feed(&config.openrouter_api_key, &context).await {
            Ok(v) => {
                analysis_source = "openrouter".to_string();
                ai_block = Some(v);
            }
            Err(e) => tracing::warn!("OpenRouter feed analysis gagal, pakai ensemble: {}", e),
        }
    }

    let recommendations: Vec<String> = if let Some(ai) = &ai_block {
        ai.get("recommendations")
            .and_then(|r| r.as_array())
            .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default()
    } else {
        let mut recs = vec![];
        if candidates.is_empty() {
            recs.push("Belum ada pakan. Tambahkan produk pakan (dengan kandungan nutrisi) atau inventori kategori 'Pakan'.".to_string());
        } else {
            recs.push(format!("Prioritaskan '{}' (kecocokan {:.0}%) untuk {}.", scored.first().map(|s| s.2.name.clone()).unwrap_or_default(), best_compat, dominant_category));
            if avg_health < 75.0 {
                recs.push("Skor kesehatan rata-rata di bawah optimal — tingkatkan kualitas pakan & monitor kondisi ternak.".to_string());
            }
            if target_protein > best.map(|b| b.2.protein).unwrap_or(0.0) {
                recs.push("Tambahkan sumber protein (mis. bungkil kedelai/konsentrat) untuk memenuhi kebutuhan.".to_string());
            }
        }
        recs
    };

    let result = json!({
        "pen_id": pen_id.to_string(),
        "pen_name": pen_name,
        "livestock_count": animals.len(),
        "dominant_category": dominant_category,
        "breeds": breeds,
        "avg_weight_kg": (avg_weight * 10.0).round() / 10.0,
        "avg_age_months": avg_age,
        "avg_health_score": (avg_health * 10.0).round() / 10.0,
        "target_nutrition": { "protein": target_protein, "energy_tdn": target_tdn },
        "top_feeds": top_feeds,
        "quality_projection": {
            "grade": quality_grade,
            "projected_adg_kg": best_adg,
            "fcr": fcr,
            "target_weight_kg": target_weight,
            "projection_days": 90,
            "body_condition": body_condition,
            "best_feed": best.map(|b| b.2.name.clone()),
        },
        "recommendations": recommendations,
        "ai_detail": ai_block,
        "analysis_source": analysis_source,
    });

    // Simpan ke MongoDB (riwayat analisa pakan kandang)
    let _ = mongo.save_doc("pen_feed_analysis", pen_id, owner_id, &result).await;

    Ok(result)
}

#[derive(serde::Deserialize)]
struct AiFeed {
    #[serde(default)]
    recommendations: Vec<String>,
}

async fn call_openrouter_feed(api_key: &str, context: &Value) -> Result<Value> {
    let client = reqwest::Client::new();
    let prompt = format!(
        r#"Kamu adalah ahli nutrisi ternak (animal nutritionist) AI. Berdasarkan data kandang & kandidat pakan berikut, analisa kecocokan pakan untuk ternak.

Data:
{}

Berikan output JSON SAJA (tanpa markdown):
{{
  "ranked_feeds": [{{"name": "...", "compatibility_percent": <0-100>, "reason": "..."}}],
  "quality_projection": {{"grade": "A|B|C", "projected_adg_kg": <angka>, "notes": "..."}},
  "recommendations": ["saran1", "saran2", "saran3"]
}}"#,
        serde_json::to_string_pretty(context).unwrap_or_default()
    );

    let body = json!({
        "model": "google/gemini-2.0-flash-001",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3,
        "max_tokens": 1200
    });

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("OpenRouter status: {}", response.status()));
    }
    let result: Value = response.json().await?;
    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow!("No content"))?;
    let json_str = if content.contains("```json") {
        content.split("```json").nth(1).and_then(|s| s.split("```").next()).unwrap_or(content)
    } else if content.contains("```") {
        content.split("```").nth(1).unwrap_or(content)
    } else {
        content
    };
    let parsed: Value = serde_json::from_str(json_str.trim())?;
    // validasi minimal
    let _: AiFeed = serde_json::from_value(parsed.clone()).unwrap_or(AiFeed { recommendations: vec![] });
    Ok(parsed)
}
