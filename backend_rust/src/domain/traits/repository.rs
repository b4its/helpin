use uuid::Uuid;

use crate::domain::entities::inventory::Inventory;
use crate::domain::entities::land::Land;
use crate::domain::entities::livestock::Livestock;
use crate::domain::entities::order::{Order, OrderItem};
use crate::domain::entities::product::Product;
use crate::domain::entities::sync_queue::SyncQueueItem;
use crate::domain::entities::user::User;

pub trait UserRepository: Send + Sync {
    fn create(
        &self,
        user: &User,
    ) -> impl std::future::Future<Output = Result<User, anyhow::Error>> + Send;

    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<User>, anyhow::Error>> + Send;
}

pub trait LivestockRepository: Send + Sync {
    fn create(
        &self,
        livestock: &Livestock,
    ) -> impl std::future::Future<Output = Result<Livestock, anyhow::Error>> + Send;

    fn find_all_by_owner(
        &self,
        owner_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Livestock>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Livestock>, anyhow::Error>> + Send;

    fn update(
        &self,
        livestock: &Livestock,
    ) -> impl std::future::Future<Output = Result<Livestock, anyhow::Error>> + Send;

    fn delete(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait LandRepository: Send + Sync {
    fn create(
        &self,
        land: &Land,
    ) -> impl std::future::Future<Output = Result<Land, anyhow::Error>> + Send;

    fn find_all_by_owner(
        &self,
        owner_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Land>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Land>, anyhow::Error>> + Send;

    fn update(
        &self,
        land: &Land,
    ) -> impl std::future::Future<Output = Result<Land, anyhow::Error>> + Send;

    fn delete(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait ProductRepository: Send + Sync {
    fn create(
        &self,
        product: &Product,
    ) -> impl std::future::Future<Output = Result<Product, anyhow::Error>> + Send;

    fn find_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Product>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Product>, anyhow::Error>> + Send;

    fn update(
        &self,
        product: &Product,
    ) -> impl std::future::Future<Output = Result<Product, anyhow::Error>> + Send;

    fn delete(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait OrderRepository: Send + Sync {
    fn create(
        &self,
        order: &Order,
        items: &[OrderItem],
    ) -> impl std::future::Future<Output = Result<Order, anyhow::Error>> + Send;

    fn find_by_buyer(
        &self,
        buyer_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Order>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Order>, anyhow::Error>> + Send;
}

pub trait SyncQueueRepository: Send + Sync {
    fn enqueue(
        &self,
        item: &SyncQueueItem,
    ) -> impl std::future::Future<Output = Result<SyncQueueItem, anyhow::Error>> + Send;

    fn get_pending(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<SyncQueueItem>, anyhow::Error>> + Send;

    fn update_status(
        &self,
        id: Uuid,
        status: &str,
        error_message: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait InventoryRepository: Send + Sync {
    fn create(
        &self,
        inventory: &Inventory,
    ) -> impl std::future::Future<Output = Result<Inventory, anyhow::Error>> + Send;

    fn find_by_land(
        &self,
        land_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Inventory>, anyhow::Error>> + Send;

    fn find_all_by_owner(
        &self,
        owner_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Inventory>, anyhow::Error>> + Send;

    fn find_by_owner_and_categories(
        &self,
        owner_id: Uuid,
        categories: &[&str],
    ) -> impl std::future::Future<Output = Result<Vec<Inventory>, anyhow::Error>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Inventory>, anyhow::Error>> + Send;

    fn update(
        &self,
        inventory: &Inventory,
    ) -> impl std::future::Future<Output = Result<Inventory, anyhow::Error>> + Send;

    fn delete(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}
