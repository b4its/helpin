use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::cart::CartItem;
use crate::domain::entities::order::{Order, OrderItem};
use crate::infrastructure::repositories::order_repo::PostgresOrderRepository;
use crate::infrastructure::repositories::product_repo::PostgresProductRepository;
use crate::domain::traits::repository::{OrderRepository, ProductRepository};
use sqlx::PgPool;

pub struct CreateOrderInput {
    pub buyer_id: Uuid,
    pub shipping_address: Option<String>,
}

pub struct CreateOrderOutput {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

pub async fn create_order(
    input: CreateOrderInput,
    order_repo: &PostgresOrderRepository,
    product_repo: &PostgresProductRepository,
    pool: &PgPool,
) -> Result<CreateOrderOutput> {
    // Get user's cart items
    let cart_items = sqlx::query_as::<_, CartItem>(
        "SELECT id, user_id, product_id, quantity, created_at FROM cart_items WHERE user_id = $1"
    )
    .bind(input.buyer_id)
    .fetch_all(pool)
    .await?;

    if cart_items.is_empty() {
        return Err(anyhow!("Keranjang kosong"));
    }

    // Validate stock and calculate total
    let mut total_amount: i64 = 0;
    let mut order_items: Vec<OrderItem> = Vec::new();
    let mut products_to_update: Vec<(Uuid, i32)> = Vec::new();

    let order_id = Uuid::new_v4();

    for cart_item in &cart_items {
        let product = product_repo.find_by_id(cart_item.product_id).await?
            .ok_or_else(|| anyhow!("Produk tidak ditemukan: {}", cart_item.product_id))?;

        if product.stock < cart_item.quantity {
            return Err(anyhow!(
                "Stok tidak mencukupi untuk produk '{}'. Tersedia: {}, Diminta: {}",
                product.name, product.stock, cart_item.quantity
            ));
        }

        let item_total = product.price * cart_item.quantity as i64;
        total_amount += item_total;

        order_items.push(OrderItem {
            id: Uuid::new_v4(),
            order_id,
            product_id: product.id,
            quantity: cart_item.quantity,
            price_at_purchase: product.price,
        });

        products_to_update.push((product.id, cart_item.quantity));
    }

    // Create order with items (uses transaction internally)
    let order = Order {
        id: order_id,
        buyer_id: input.buyer_id,
        status: Some("Menunggu Pembayaran".to_string()),
        total_amount,
        shipping_address: input.shipping_address,
        created_at: None,
        updated_at: None,
    };

    let created_order = order_repo.create(&order, &order_items).await?;

    // Reduce stock for each product
    for (product_id, qty) in &products_to_update {
        sqlx::query("UPDATE products SET stock = stock - $2, updated_at = NOW() WHERE id = $1")
            .bind(product_id)
            .bind(qty)
            .execute(pool)
            .await?;
    }

    // Clear cart
    sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
        .bind(input.buyer_id)
        .execute(pool)
        .await?;

    Ok(CreateOrderOutput {
        order: created_order,
        items: order_items,
    })
}
