use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::health_record::HealthRecord;
use crate::domain::entities::livestock::Livestock;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::domain::traits::repository::LivestockRepository;
use sqlx::PgPool;

pub struct HealthEvaluationInput {
    pub livestock_id: Uuid,
    pub owner_id: Uuid,
    pub symptoms: Option<String>,
    pub body_temp: Option<f64>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub notes: Option<String>,
}

pub struct HealthEvaluationOutput {
    pub health_record: HealthRecord,
    pub livestock: Livestock,
}

pub async fn evaluate_health(
    input: HealthEvaluationInput,
    livestock_repo: &PostgresLivestockRepository,
    pool: &PgPool,
) -> Result<HealthEvaluationOutput> {
    let mut livestock = livestock_repo.find_by_id(input.livestock_id).await?
        .ok_or_else(|| anyhow!("Ternak tidak ditemukan"))?;

    // Verify ownership
    if livestock.owner_id != input.owner_id {
        return Err(anyhow!("Tidak memiliki akses ke ternak ini"));
    }

    // Calculate health score based on biometrics
    let health_score = calculate_health_score(
        input.body_temp,
        input.heart_rate,
        input.respiratory_rate,
        input.symptoms.as_deref(),
    );

    // Determine health status based on score
    let health_status = if health_score < 50 {
        "Sakit"
    } else if health_score < 75 {
        "Perlu Perhatian"
    } else {
        "Sehat"
    };

    // Save health record
    let record_id = Uuid::new_v4();
    let body_temp_bd = input.body_temp.map(|t| {
        bigdecimal::BigDecimal::try_from(t).unwrap_or_default()
    });

    let health_record = sqlx::query_as::<_, HealthRecord>(
        r#"
        INSERT INTO health_records (id, livestock_id, symptoms, body_temp, heart_rate, respiratory_rate, notes, health_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, livestock_id, symptoms, body_temp, heart_rate, respiratory_rate, notes, health_score, recorded_at
        "#,
    )
    .bind(record_id)
    .bind(input.livestock_id)
    .bind(&input.symptoms)
    .bind(&body_temp_bd)
    .bind(input.heart_rate)
    .bind(input.respiratory_rate)
    .bind(&input.notes)
    .bind(health_score)
    .fetch_one(pool)
    .await?;

    // Update livestock health_score and health_status
    livestock.health_score = Some(health_score);
    livestock.health_status = Some(health_status.to_string());
    let updated_livestock = livestock_repo.update(&livestock).await?;

    Ok(HealthEvaluationOutput {
        health_record,
        livestock: updated_livestock,
    })
}

fn calculate_health_score(
    body_temp: Option<f64>,
    heart_rate: Option<i32>,
    respiratory_rate: Option<i32>,
    symptoms: Option<&str>,
) -> i32 {
    let mut score: i32 = 100;

    // Body temperature check (normal range for cattle: 38.0-39.5°C)
    if let Some(temp) = body_temp {
        if temp < 37.0 || temp > 40.5 {
            score -= 30;
        } else if temp < 38.0 || temp > 39.5 {
            score -= 15;
        }
    }

    // Heart rate check (normal for cattle: 40-80 bpm)
    if let Some(hr) = heart_rate {
        if hr < 30 || hr > 100 {
            score -= 25;
        } else if hr < 40 || hr > 80 {
            score -= 10;
        }
    }

    // Respiratory rate check (normal for cattle: 12-36 breaths/min)
    if let Some(rr) = respiratory_rate {
        if rr < 8 || rr > 50 {
            score -= 25;
        } else if rr < 12 || rr > 36 {
            score -= 10;
        }
    }

    // Symptoms penalty
    if let Some(s) = symptoms {
        if !s.trim().is_empty() {
            score -= 20;
        }
    }

    score.max(0).min(100)
}
