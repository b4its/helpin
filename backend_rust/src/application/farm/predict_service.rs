//! Prediksi panen rule-based (deterministik, selalu jalan tanpa dependensi eksternal).
//!
//! Memperhitungkan: jenis komoditas, luas lahan, jenis tanah, dan kualitas treatment
//! (bibit, pupuk + jumlah, alat pertanian). Treatment lebih baik => panen lebih cepat
//! dan kualitas (grade) lebih bagus.

use chrono::{Duration, NaiveDate};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PredictionResult {
    pub predicted_weight_kg: f64,
    pub quality_grade: String, // "A" | "B" | "C"
    pub predicted_harvest_date: NaiveDate,
    pub growth_days: i64,
    pub feasibility_status: String, // "Layak" | "Tidak Layak"
    pub score: f64,                 // 0..1 (kualitas treatment keseluruhan)
}

/// Ringkasan treatment yang dipakai di lahan.
#[derive(Debug, Clone, Default)]
pub struct TreatmentSummary {
    pub seed_present: bool,
    pub fertilizer_count: usize,
    pub fertilizer_total_qty: f64,
    pub tool_count: usize,
}

/// Estimasi hasil dasar (kg per hektar) berdasarkan komoditas.
fn base_yield_per_ha(crop: &str) -> f64 {
    let c = crop.to_lowercase();
    if c.contains("padi") {
        6000.0
    } else if c.contains("jagung") {
        5500.0
    } else if c.contains("kedelai") {
        2500.0
    } else if c.contains("sawit") {
        18000.0
    } else if c.contains("cabai") || c.contains("cabe") {
        12000.0
    } else if c.contains("tomat") {
        30000.0
    } else if c.contains("bawang") {
        10000.0
    } else if c.contains("kopi") {
        2000.0
    } else if c.contains("tebu") {
        70000.0
    } else {
        5000.0
    }
}

/// Lama pertumbuhan dasar (hari) sampai panen berdasarkan komoditas.
fn base_growth_days(crop: &str) -> i64 {
    let c = crop.to_lowercase();
    if c.contains("padi") {
        110
    } else if c.contains("jagung") {
        100
    } else if c.contains("kedelai") {
        90
    } else if c.contains("sawit") {
        150
    } else if c.contains("cabai") || c.contains("cabe") {
        90
    } else if c.contains("tomat") {
        75
    } else if c.contains("bawang") {
        70
    } else if c.contains("kopi") {
        240
    } else if c.contains("tebu") {
        300
    } else {
        100
    }
}

/// Faktor kesuburan tanah (0.85..1.1).
fn soil_factor(soil: &str) -> f64 {
    let s = soil.to_lowercase();
    if s.contains("loam") || s.contains("lempung") || s.contains("gembur") {
        1.1
    } else if s.contains("silt") || s.contains("debu") {
        1.0
    } else if s.contains("clay") || s.contains("liat") {
        0.95
    } else if s.contains("sand") || s.contains("pasir") {
        0.85
    } else {
        1.0
    }
}

/// Skor kualitas treatment keseluruhan (0..1).
fn treatment_score(t: &TreatmentSummary) -> f64 {
    let mut score = 0.4_f64; // baseline tanpa treatment
    if t.seed_present {
        score += 0.15;
    }
    score += 0.1 * (t.fertilizer_count.min(3) as f64); // maks +0.3
    score += 0.05 * (t.tool_count.min(2) as f64); // maks +0.1
    score += (t.fertilizer_total_qty / 100.0).min(0.15); // bonus jumlah pupuk, maks +0.15
    score.clamp(0.0, 1.0)
}

/// Hitung prediksi panen untuk sebuah lahan.
pub fn compute_prediction(
    crop_type: &str,
    area_hectare: f64,
    soil_type: &str,
    plant_date: NaiveDate,
    treatment: &TreatmentSummary,
) -> PredictionResult {
    let base = base_yield_per_ha(crop_type);
    let soil_f = soil_factor(soil_type);
    let t_score = treatment_score(treatment);

    // Multiplier hasil panen: treatment buruk 0.7x, treatment optimal 1.3x.
    let yield_multiplier = 0.7 + 0.6 * t_score;
    let area = if area_hectare > 0.0 { area_hectare } else { 1.0 };
    let predicted_weight_kg = (base * area * yield_multiplier * soil_f).round();

    // Grade berdasarkan kombinasi skor treatment & tanah.
    let combined = (t_score * 0.75 + (soil_f - 0.85) / 0.25 * 0.25).clamp(0.0, 1.0);
    let quality_grade = if combined >= 0.8 {
        "A"
    } else if combined >= 0.6 {
        "B"
    } else {
        "C"
    }
    .to_string();

    // Masa tumbuh: treatment lebih baik => panen lebih cepat (maks 15% lebih cepat).
    let base_days = base_growth_days(crop_type);
    let growth_days = ((base_days as f64) * (1.0 - 0.15 * t_score)).round() as i64;
    let predicted_harvest_date = plant_date + Duration::days(growth_days);

    PredictionResult {
        predicted_weight_kg,
        quality_grade,
        predicted_harvest_date,
        growth_days,
        feasibility_status: if predicted_weight_kg > 0.0 {
            "Layak".to_string()
        } else {
            "Tidak Layak".to_string()
        },
        score: t_score,
    }
}
