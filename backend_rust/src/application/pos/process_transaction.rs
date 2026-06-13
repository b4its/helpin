use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::financial_record::FinancialRecord;
use crate::domain::entities::transaction::PosTransaction;
use crate::infrastructure::repositories::financial_repo::PostgresFinancialRepository;
use crate::infrastructure::repositories::transaction_repo::PostgresTransactionRepository;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosItem {
    pub product_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub price: i64,
}

pub struct ProcessTransactionInput {
    pub cashier_id: Uuid,
    pub items: Vec<PosItem>,
    pub amount_tendered: i64,
}

pub struct ProcessTransactionOutput {
    pub transaction: PosTransaction,
    pub financial_record: FinancialRecord,
}

pub async fn process_transaction(
    input: ProcessTransactionInput,
    transaction_repo: &PostgresTransactionRepository,
    financial_repo: &PostgresFinancialRepository,
    pool: &PgPool,
) -> Result<ProcessTransactionOutput> {
    if input.items.is_empty() {
        return Err(anyhow!("Transaksi harus memiliki minimal 1 item"));
    }

    // Calculate subtotal
    let subtotal: i64 = input.items.iter().map(|item| item.price * item.quantity as i64).sum();

    // Calculate tax = subtotal * 0.02
    let tax = (subtotal as f64 * 0.02) as i64;
    let total = subtotal + tax;

    // Validate amount tendered
    if input.amount_tendered < total {
        return Err(anyhow!(
            "Jumlah pembayaran tidak mencukupi. Total: {}, Dibayar: {}",
            total, input.amount_tendered
        ));
    }

    let change_amount = input.amount_tendered - total;

    // Create POS transaction
    let transaction = PosTransaction {
        id: Uuid::new_v4(),
        cashier_id: input.cashier_id,
        items: serde_json::to_value(&input.items)?,
        subtotal,
        tax,
        total,
        amount_tendered: input.amount_tendered,
        change_amount,
        created_at: None,
    };

    let saved_transaction = transaction_repo.create(&transaction).await?;

    // Reduce stock for each item
    for item in &input.items {
        sqlx::query("UPDATE products SET stock = stock - $2, updated_at = NOW() WHERE id = $1")
            .bind(item.product_id)
            .bind(item.quantity)
            .execute(pool)
            .await?;
    }

    // Get current balance
    let current_balance = financial_repo.get_balance().await?;
    let new_balance = current_balance + total;

    // Create financial record with type "pemasukan"
    let financial_record = FinancialRecord {
        id: Uuid::new_v4(),
        record_type: "pemasukan".to_string(),
        amount: total,
        category: Some("Penjualan POS".to_string()),
        description: Some(format!("Transaksi kasir - {} item", input.items.len())),
        reference_id: Some(saved_transaction.id),
        balance_after: new_balance,
        recorded_at: None,
    };

    let saved_financial = financial_repo.create_record(&financial_record).await?;

    Ok(ProcessTransactionOutput {
        transaction: saved_transaction,
        financial_record: saved_financial,
    })
}
