use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::financial_record::FinancialRecord;
use crate::infrastructure::repositories::financial_repo::PostgresFinancialRepository;

pub struct RecordExpenseInput {
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
}

pub async fn record_expense(
    input: RecordExpenseInput,
    financial_repo: &PostgresFinancialRepository,
) -> Result<FinancialRecord> {
    if input.amount <= 0 {
        return Err(anyhow!("Jumlah pengeluaran harus lebih dari 0"));
    }

    // Get current balance
    let current_balance = financial_repo.get_balance().await?;
    let new_balance = current_balance - input.amount;

    let record = FinancialRecord {
        id: Uuid::new_v4(),
        record_type: "pengeluaran".to_string(),
        amount: input.amount,
        category: input.category,
        description: input.description,
        reference_id: None,
        balance_after: new_balance,
        recorded_at: None,
    };

    let saved = financial_repo.create_record(&record).await?;
    Ok(saved)
}

/// Catat pemasukan kas manual (mis. modal awal, pemasukan lain).
pub async fn record_income(
    input: RecordExpenseInput,
    financial_repo: &PostgresFinancialRepository,
) -> Result<FinancialRecord> {
    if input.amount <= 0 {
        return Err(anyhow!("Jumlah pemasukan harus lebih dari 0"));
    }
    let current_balance = financial_repo.get_balance().await?;
    let new_balance = current_balance + input.amount;
    let record = FinancialRecord {
        id: Uuid::new_v4(),
        record_type: "pemasukan".to_string(),
        amount: input.amount,
        category: input.category,
        description: input.description,
        reference_id: None,
        balance_after: new_balance,
        recorded_at: None,
    };
    let saved = financial_repo.create_record(&record).await?;
    Ok(saved)
}

pub async fn get_balance(
    financial_repo: &PostgresFinancialRepository,
) -> Result<i64> {
    let balance = financial_repo.get_balance().await?;
    Ok(balance)
}

pub async fn get_report_by_period(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    financial_repo: &PostgresFinancialRepository,
) -> Result<Vec<FinancialRecord>> {
    let records = financial_repo.get_report_by_period(start, end).await?;
    Ok(records)
}
