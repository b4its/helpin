use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::cart::CartItem;
use crate::infrastructure::repositories::product_repo::PostgresProductRepository;
use crate::domain::traits::repository::ProductRepository;
use sqlx::PgPool;

pub struct AddToCartInput {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

pub async fn add_to_cart(
    input: AddToCartInput,
    product_repo: &PostgresProductRepository,
    pool: &PgPool,
) -> Result<CartItem> {
    // Validate product exists and has stock
    let product = product_repo.find_by_id(input.product_id).await?
        .ok_or_else(|| anyhow!("Produk tidak ditemukan"))?;

    if product.stock <= 0 {
        return Err(anyhow!("Stok produk habis"));
    }

    if input.quantity <= 0 {
        return Err(anyhow!("Jumlah harus lebih dari 0"));
    }

    if input.quantity > product.stock {
        return Err(anyhow!("Jumlah melebihi stok yang tersedia"));
    }

    // Upsert cart item (INSERT ON CONFLICT UPDATE)
    let id = Uuid::new_v4();
    let cart_item = sqlx::query_as::<_, CartItem>(
        r#"
        INSERT INTO cart_items (id, user_id, product_id, quantity)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, product_id) DO UPDATE SET quantity = cart_items.quantity + $4
        RETURNING id, user_id, product_id, quantity, created_at
        "#,
    )
    .bind(id)
    .bind(input.user_id)
    .bind(input.product_id)
    .bind(input.quantity)
    .fetch_one(pool)
    .await?;

    Ok(cart_item)
}

pub async fn update_cart_quantity(
    item_id: Uuid,
    user_id: Uuid,
    quantity: i32,
    pool: &PgPool,
) -> Result<CartItem> {
    if quantity <= 0 {
        return Err(anyhow!("Jumlah harus lebih dari 0"));
    }

    let cart_item = sqlx::query_as::<_, CartItem>(
        r#"
        UPDATE cart_items SET quantity = $3
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, product_id, quantity, created_at
        "#,
    )
    .bind(item_id)
    .bind(user_id)
    .bind(quantity)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Item keranjang tidak ditemukan"))?;

    Ok(cart_item)
}

pub async fn remove_from_cart(
    item_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<()> {
    let result = sqlx::query("DELETE FROM cart_items WHERE id = $1 AND user_id = $2")
        .bind(item_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("Item keranjang tidak ditemukan"));
    }

    Ok(())
}

pub async fn get_cart(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<CartItem>> {
    let items = sqlx::query_as::<_, CartItem>(
        r#"
        SELECT id, user_id, product_id, quantity, created_at
        FROM cart_items WHERE user_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(items)
}
