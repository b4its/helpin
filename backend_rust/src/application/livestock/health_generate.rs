//! Generate kondisi kesehatan ternak yang detail.
//!
//! - Jika biometrik tidak diberikan, di-generate otomatis (online OpenRouter /
//!   offline rule-based berdasar kategori, ras, usia, berat, & skor kesehatan terakhir).
//! - Menghitung skor kesehatan, status, kemungkinan kondisi, dan rekomendasi.
//! - Menyimpan ke tabel `health_records` dan memperbarui status ternak.

use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;

#[derive(Default)]
pub struct ProvidedVitals {
    pub heart_rate: Option<i32>,
    pub body_temp: Option<f64>,
    pub respiratory_rate: Option<i32>,
    pub symptoms: Option<String>,
    pub notes: Option<String>,
}

/// Rentang normal biometrik per kategori ternak: (hr_min,hr_max, temp_min,temp_max, rr_min,rr_max)
fn normal_ranges(category: &str) -> (i32, i32, f64, f64, i32, i32) {
    match category {
        "Sapi" => (48, 84, 38.0, 39.3, 12, 36),
        "Kambing" | "Domba" => (70, 90, 38.5, 40.0, 15, 35),
        _ => (50, 90, 38.0, 39.5, 12, 36),
    }
}

/// Generate biometrik realistis offline dari kondisi terakhir ternak.
fn generate_vitals(category: &str, last_health: i32) -> (i32, f64, i32) {
    let (hr_min, hr_max, t_min, t_max, rr_min, rr_max) = normal_ranges(category);
    // Sehat → mendekati tengah; kurang sehat → mendekati/melewati batas atas
    let factor = ((100 - last_health.clamp(0, 100)) as f64) / 100.0; // 0 sehat .. 1 sakit
    let hr = hr_min as f64 + (hr_max - hr_min) as f64 * (0.45 + 0.6 * factor);
    let temp = t_min + (t_max - t_min) * (0.4 + 0.8 * factor);
    let rr = rr_min as f64 + (rr_max - rr_min) as f64 * (0.45 + 0.7 * factor);
    (hr.round() as i32, (temp * 10.0).round() / 10.0, rr.round() as i32)
}

fn score_vitals(category: &str, hr: i32, temp: f64, rr: i32, symptoms: Option<&str>) -> (i32, Vec<String>, Vec<String>) {
    let (hr_min, hr_max, t_min, t_max, rr_min, rr_max) = normal_ranges(category);
    let mut score: i32 = 100;
    let mut recs: Vec<String> = vec![];
    let mut conds: Vec<String> = vec![];

    if hr < hr_min - 10 || hr > hr_max + 15 {
        score -= 25;
        recs.push("Detak jantung abnormal — periksa kondisi kardiovaskular.".to_string());
        conds.push("Aritmia".to_string());
    } else if hr < hr_min || hr > hr_max {
        score -= 10;
        recs.push("Detak jantung sedikit di luar normal — monitor berkala.".to_string());
    }

    if temp > t_max + 1.0 {
        score -= 30;
        recs.push("Demam tinggi — isolasi ternak & konsultasi medis.".to_string());
        conds.push("Infeksi bakteri".to_string());
    } else if temp > t_max {
        score -= 15;
        recs.push("Suhu tubuh agak tinggi — pastikan air minum cukup.".to_string());
    } else if temp < t_min - 0.5 {
        score -= 20;
        recs.push("Suhu rendah (hipotermia) — periksa lingkungan kandang.".to_string());
        conds.push("Hipotermia".to_string());
    }

    if rr > rr_max + 8 {
        score -= 20;
        recs.push("Laju napas tinggi — periksa saluran pernapasan.".to_string());
        conds.push("Pneumonia".to_string());
    } else if rr > rr_max {
        score -= 10;
        recs.push("Respirasi agak cepat — pastikan ventilasi kandang baik.".to_string());
    }

    if let Some(s) = symptoms {
        if !s.trim().is_empty() {
            score -= 20;
            recs.push(format!("Gejala dilaporkan: {} — perlu pemeriksaan lebih lanjut.", s.trim()));
        }
    }

    if recs.is_empty() {
        recs.push("Kondisi ternak dalam batas normal. Lanjutkan pemantauan rutin.".to_string());
    }
    (score.clamp(0, 100), recs, conds)
}

pub async fn generate_health(
    pool: &PgPool,
    config: &Config,
    livestock_repo: &PostgresLivestockRepository,
    livestock_id: Uuid,
    owner_id: Uuid,
    provided: ProvidedVitals,
) -> Result<Value> {
    let mut livestock = livestock_repo
        .find_by_id(livestock_id)
        .await?
        .ok_or_else(|| anyhow!("Ternak tidak ditemukan"))?;
    if livestock.owner_id != owner_id {
        return Err(anyhow!("Tidak memiliki akses ke ternak ini"));
    }

    let category = livestock.category.clone();
    let last_health = livestock.health_score.unwrap_or(100);

    // Lengkapi biometrik (generate jika kosong)
    let auto = provided.heart_rate.is_none()
        && provided.body_temp.is_none()
        && provided.respiratory_rate.is_none();
    let (gen_hr, gen_temp, gen_rr) = generate_vitals(&category, last_health);
    let hr = provided.heart_rate.unwrap_or(gen_hr);
    let temp = provided.body_temp.unwrap_or(gen_temp);
    let rr = provided.respiratory_rate.unwrap_or(gen_rr);

    let (score, mut recommendations, mut conditions) =
        score_vitals(&category, hr, temp, rr, provided.symptoms.as_deref());

    let status = if score < 50 {
        "Sakit"
    } else if score < 75 {
        "Perlu Perhatian"
    } else {
        "Sehat"
    };

    // Online: perkaya detail via OpenRouter (opsional)
    let mut analysis_source = if auto { "ensemble_auto" } else { "ensemble" }.to_string();
    let mut ai_detail: Option<Value> = None;
    if !config.openrouter_api_key.is_empty() {
        let ctx = json!({
            "category": category, "breed": livestock.breed,
            "weight_kg": livestock.weight.to_f64().unwrap_or(0.0),
            "age_months": livestock.age_months,
            "heart_rate": hr, "body_temp": temp, "respiratory_rate": rr,
            "symptoms": provided.symptoms, "computed_score": score
        });
        if let Ok(v) = call_openrouter_health(&config.openrouter_api_key, &ctx).await {
            analysis_source = "openrouter".to_string();
            if let Some(arr) = v.get("recommendations").and_then(|x| x.as_array()) {
                let ai_recs: Vec<String> = arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
                if !ai_recs.is_empty() { recommendations = ai_recs; }
            }
            if let Some(arr) = v.get("possible_conditions").and_then(|x| x.as_array()) {
                let ai_conds: Vec<String> = arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
                if !ai_conds.is_empty() { conditions = ai_conds; }
            }
            ai_detail = Some(v);
        }
    }

    // Simpan ke health_records
    let temp_bd = bigdecimal::BigDecimal::try_from(temp).ok();
    let notes = provided.notes.clone().unwrap_or_else(|| format!("Auto-generate ({})", analysis_source));
    let rec_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO health_records (id, livestock_id, symptoms, body_temp, heart_rate, respiratory_rate, notes, health_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(rec_id)
    .bind(livestock_id)
    .bind(&provided.symptoms)
    .bind(&temp_bd)
    .bind(hr)
    .bind(rr)
    .bind(&notes)
    .bind(score)
    .execute(pool)
    .await?;

    // Update status ternak
    livestock.health_score = Some(score);
    livestock.health_status = Some(status.to_string());
    let _ = livestock_repo.update(&livestock).await;

    // Perbarui kondisi kandang jika ternak ada di kandang
    if let Some(pen_id) = livestock.pen_id {
        crate::application::livestock::pen_condition::refresh_pen_condition(pool, pen_id).await;
    }

    let body_condition = if score >= 85 { "Prima" } else if score >= 70 { "Baik" } else if score >= 50 { "Perlu Perhatian" } else { "Kritis" };

    Ok(json!({
        "id": rec_id.to_string(),
        "livestock_id": livestock_id.to_string(),
        "tag_id": livestock.tag_id,
        "category": category,
        "breed": livestock.breed,
        "age_months": livestock.age_months,
        "weight_kg": livestock.weight.to_f64().unwrap_or(0.0),
        "vitals": { "heart_rate": hr, "body_temp": temp, "respiratory_rate": rr },
        "auto_generated": auto,
        "health_score": score,
        "status": status,
        "body_condition": body_condition,
        "possible_conditions": conditions,
        "recommendations": recommendations,
        "ai_detail": ai_detail,
        "analysis_source": analysis_source,
    }))
}

async fn call_openrouter_health(api_key: &str, ctx: &Value) -> Result<Value> {
    let client = reqwest::Client::new();
    let prompt = format!(
        r#"Kamu adalah dokter hewan (veterinarian) AI. Berdasarkan data biometrik ternak berikut, evaluasi kesehatannya.

Data:
{}

Berikan output JSON SAJA (tanpa markdown):
{{
  "assessment": "ringkasan kondisi",
  "possible_conditions": ["..."],
  "recommendations": ["saran1", "saran2", "saran3"],
  "urgency": "Rendah|Sedang|Tinggi"
}}"#,
        serde_json::to_string_pretty(ctx).unwrap_or_default()
    );
    let body = json!({
        "model": "google/gemini-2.0-flash-001",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3,
        "max_tokens": 1000
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
    let content = result["choices"][0]["message"]["content"].as_str().ok_or_else(|| anyhow!("No content"))?;
    let json_str = if content.contains("```json") {
        content.split("```json").nth(1).and_then(|s| s.split("```").next()).unwrap_or(content)
    } else if content.contains("```") {
        content.split("```").nth(1).unwrap_or(content)
    } else {
        content
    };
    Ok(serde_json::from_str(json_str.trim())?)
}
