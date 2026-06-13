use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::domain::entities::product::Product;
use crate::infrastructure::repositories::product_repo::PostgresProductRepository;
use crate::domain::traits::repository::ProductRepository;

pub struct CreateProductInput {
    pub seller_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub price: i64,
    pub stock: i32,
    pub unit: String,
    pub product_type: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub nutrition: Option<serde_json::Value>,
    pub purchase_price: Option<i64>,
    pub expired_at: Option<chrono::NaiveDate>,
    pub entry_date: Option<chrono::NaiveDate>,
}

pub struct UpdateProductInput {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub price: Option<i64>,
    pub stock: Option<i32>,
    pub unit: Option<String>,
    pub product_type: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub nutrition: Option<serde_json::Value>,
    pub purchase_price: Option<i64>,
    pub expired_at: Option<chrono::NaiveDate>,
    pub entry_date: Option<chrono::NaiveDate>,
}

pub async fn create_product(
    input: CreateProductInput,
    product_repo: &PostgresProductRepository,
) -> Result<Product> {
    if input.price <= 0 {
        return Err(anyhow!("Harga harus lebih dari 0"));
    }

    let product = Product {
        id: Uuid::new_v4(),
        seller_id: input.seller_id,
        category_id: input.category_id,
        name: input.name,
        price: input.price,
        stock: input.stock,
        unit: input.unit,
        product_type: input.product_type,
        location: input.location,
        image_url: input.image_url,
        description: input.description,
        nutrition: input.nutrition,
        purchase_price: input.purchase_price,
        expired_at: input.expired_at,
        entry_date: input.entry_date,
        created_at: None,
        updated_at: None,
    };

    let created = product_repo.create(&product).await?;
    Ok(created)
}

pub async fn update_product(
    input: UpdateProductInput,
    product_repo: &PostgresProductRepository,
    privileged: bool,
) -> Result<Product> {
    let mut product = product_repo.find_by_id(input.id).await?
        .ok_or_else(|| anyhow!("Produk tidak ditemukan"))?;

    // Admin/karyawan boleh mengelola produk milik siapa pun
    if !privileged && product.seller_id != input.seller_id {
        return Err(anyhow!("Tidak memiliki akses ke produk ini"));
    }

    // Update fields if provided
    if let Some(name) = input.name {
        product.name = name;
    }
    if let Some(price) = input.price {
        if price <= 0 {
            return Err(anyhow!("Harga harus lebih dari 0"));
        }
        product.price = price;
    }
    if let Some(stock) = input.stock {
        product.stock = stock;
    }
    if let Some(unit) = input.unit {
        product.unit = unit;
    }
    if let Some(category_id) = input.category_id {
        product.category_id = Some(category_id);
    }
    if let Some(product_type) = input.product_type {
        product.product_type = Some(product_type);
    }
    if let Some(location) = input.location {
        product.location = Some(location);
    }
    if let Some(image_url) = input.image_url {
        product.image_url = Some(image_url);
    }
    if let Some(description) = input.description {
        product.description = Some(description);
    }
    if let Some(nutrition) = input.nutrition {
        product.nutrition = Some(nutrition);
    }
    if let Some(pp) = input.purchase_price {
        product.purchase_price = Some(pp);
    }
    if let Some(exp) = input.expired_at {
        product.expired_at = Some(exp);
    }
    if let Some(ed) = input.entry_date {
        product.entry_date = Some(ed);
    }

    let updated = product_repo.update(&product).await?;
    Ok(updated)
}

pub async fn delete_product(
    id: Uuid,
    seller_id: Uuid,
    product_repo: &PostgresProductRepository,
    privileged: bool,
) -> Result<()> {
    let product = product_repo.find_by_id(id).await?
        .ok_or_else(|| anyhow!("Produk tidak ditemukan"))?;

    // Admin/karyawan boleh menghapus produk milik siapa pun
    if !privileged && product.seller_id != seller_id {
        return Err(anyhow!("Tidak memiliki akses ke produk ini"));
    }

    product_repo.delete(id).await?;
    Ok(())
}

pub async fn list_products(
    product_repo: &PostgresProductRepository,
) -> Result<Vec<Product>> {
    let products = product_repo.find_all().await?;
    Ok(products)
}

pub async fn get_product(
    id: Uuid,
    product_repo: &PostgresProductRepository,
) -> Result<Product> {
    let product = product_repo.find_by_id(id).await?
        .ok_or_else(|| anyhow!("Produk tidak ditemukan"))?;
    Ok(product)
}
