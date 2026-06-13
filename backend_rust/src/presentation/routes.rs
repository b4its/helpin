use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::config::Config;
use crate::infrastructure::repositories::financial_repo::PostgresFinancialRepository;
use crate::infrastructure::repositories::inventory_repo::PostgresInventoryRepository;
use crate::infrastructure::repositories::land_repo::PostgresLandRepository;
use crate::infrastructure::repositories::livestock_repo::PostgresLivestockRepository;
use crate::infrastructure::repositories::order_repo::PostgresOrderRepository;
use crate::infrastructure::repositories::product_repo::PostgresProductRepository;
use crate::infrastructure::repositories::sync_queue_repo::PostgresSyncQueueRepository;
use crate::infrastructure::repositories::transaction_repo::PostgresTransactionRepository;
use crate::infrastructure::repositories::user_repo::PostgresUserRepository;
use crate::infrastructure::services::blockchain_service::HyperledgerService;
use crate::infrastructure::services::crypto_service::Ed25519CryptoService;
use crate::infrastructure::services::jwt_service::JwtService;
use crate::infrastructure::services::ml_client::MlClientService;
use crate::infrastructure::services::mongo_service::MongoService;

use crate::presentation::handlers::{
    admin_handler, auth_handler, cart_handler, farm_handler, feed_handler, finance_handler,
    health_handler, inventory_handler, livestock_handler, livestock_health_handler, ml_handler,
    order_handler, pen_handler, pos_handler, prediction_handler, prefetch_handler, product_handler,
    sync_handler, treatment_handler, blockchain_handler,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub user_repo: PostgresUserRepository,
    pub livestock_repo: PostgresLivestockRepository,
    pub land_repo: PostgresLandRepository,
    pub inventory_repo: PostgresInventoryRepository,
    pub product_repo: PostgresProductRepository,
    pub order_repo: PostgresOrderRepository,
    pub transaction_repo: PostgresTransactionRepository,
    pub financial_repo: PostgresFinancialRepository,
    pub sync_queue_repo: PostgresSyncQueueRepository,
    pub jwt_service: JwtService,
    pub crypto_service: Ed25519CryptoService,
    pub ml_client: MlClientService,
    pub blockchain_service: HyperledgerService,
    pub mongo: MongoService,
    pub config: Config,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Auth (public)
        .route("/api/auth/register", post(auth_handler::register_handler))
        .route("/api/auth/login", post(auth_handler::login_handler))
        .route("/api/auth/refresh", post(auth_handler::refresh_handler))
        .route("/api/auth/me", get(auth_handler::get_me))
        // Livestock
        .route(
            "/api/livestock",
            get(livestock_handler::list).post(livestock_handler::create),
        )
        .route(
            "/api/livestock/{id}",
            get(livestock_handler::get_by_id)
                .put(livestock_handler::update)
                .delete(livestock_handler::delete),
        )
        // Kondisi kesehatan ternak (generate detail + riwayat)
        .route(
            "/api/livestock/{id}/health",
            post(livestock_health_handler::record_health),
        )
        .route(
            "/api/livestock/{id}/health-history",
            get(livestock_health_handler::health_history),
        )
        // Pens
        .route("/api/pens", get(pen_handler::list).post(pen_handler::create))
        .route(
            "/api/pens/{id}",
            put(pen_handler::update).delete(pen_handler::delete),
        )
        .route("/api/pens/{id}/livestock", get(pen_handler::list_livestock))
        // Analisa pakan per kandang
        .route(
            "/api/pens/{id}/feed-analysis",
            get(feed_handler::list_analysis).post(feed_handler::analyze),
        )
        // Lands
        .route(
            "/api/lands",
            get(farm_handler::list_lands).post(farm_handler::create_land_handler),
        )
        .route(
            "/api/lands/{id}",
            put(farm_handler::update_land).delete(farm_handler::delete_land),
        )
        .route("/api/lands/{id}/harvests", get(farm_handler::list_harvests))
        // Treatment & estimasi panen lahan
        .route(
            "/api/lands/{id}/treatment",
            post(treatment_handler::add_treatment),
        )
        .route(
            "/api/lands/{id}/treatments",
            get(treatment_handler::list_treatments),
        )
        .route(
            "/api/lands/{id}/prediction",
            get(treatment_handler::get_prediction),
        )
        // Inventory (standalone — milik owner)
        .route(
            "/api/inventory",
            get(inventory_handler::list_inventory).post(inventory_handler::create_inventory),
        )
        .route(
            "/api/inventory/available",
            get(inventory_handler::list_inventory_for_land),
        )
        .route(
            "/api/inventory/{id}",
            put(inventory_handler::update_inventory).delete(inventory_handler::delete_inventory),
        )
        // Inventory by land (via junction table)
        .route(
            "/api/lands/{id}/inventory",
            get(inventory_handler::list_inventory_by_land),
        )
        // Plants
        .route("/api/plants", post(farm_handler::create_plant))
        .route(
            "/api/plants/{id}/status",
            put(farm_handler::update_plant_status),
        )
        // Products
        .route(
            "/api/products",
            get(product_handler::list).post(product_handler::create),
        )
        .route(
            "/api/products/{id}",
            get(product_handler::get_by_id)
                .put(product_handler::update)
                .delete(product_handler::delete),
        )
        .route("/api/categories", get(product_handler::list_categories).post(product_handler::create_category))
        .route(
            "/api/categories/{id}",
            put(product_handler::update_category).delete(product_handler::delete_category),
        )
        // Cart
        .route(
            "/api/cart",
            get(cart_handler::get_cart).post(cart_handler::add_to_cart),
        )
        .route(
            "/api/cart/{id}",
            put(cart_handler::update_qty).delete(cart_handler::remove_item),
        )
        // Orders
        .route(
            "/api/orders",
            get(order_handler::list_orders).post(order_handler::checkout),
        )
        .route("/api/orders/{id}", get(order_handler::get_order))
        // POS
        .route(
            "/api/pos/transactions",
            get(pos_handler::list_transactions).post(pos_handler::create_transaction),
        )
        // Finance
        .route("/api/finance/balance", get(finance_handler::get_balance))
        .route(
            "/api/finance/expenses",
            post(finance_handler::record_expense),
        )
        .route(
            "/api/finance/income",
            post(finance_handler::record_income),
        )
        .route("/api/finance/report", get(finance_handler::get_report))
        // Admin panel
        .route("/api/admin/users", get(admin_handler::list_users))
        .route(
            "/api/admin/employees",
            get(admin_handler::list_employees).post(admin_handler::create_employee),
        )
        .route(
            "/api/admin/employees/{id}",
            put(admin_handler::update_employee).delete(admin_handler::delete_employee),
        )
        .route(
            "/api/admin/suppliers",
            get(admin_handler::list_suppliers).post(admin_handler::create_supplier),
        )
        .route(
            "/api/admin/suppliers/{id}",
            put(admin_handler::update_supplier).delete(admin_handler::delete_supplier),
        )
        .route("/api/admin/activities", get(admin_handler::list_activities))
        .route("/api/admin/stats", get(admin_handler::stats))
        // ML
        .route(
            "/api/ml/feed-recommendation",
            post(ml_handler::predict_feed_handler),
        )
        .route("/api/ml/feed-history", get(ml_handler::get_feed_history))
        .route(
            "/api/ml/health-evaluation",
            post(ml_handler::evaluate_health_handler),
        )
        // Harvest Prediction (OpenRouter + XGBoost fallback)
        .route(
            "/api/predict/harvest/{land_id}",
            post(prediction_handler::predict_harvest),
        )
        // Sync
        .route("/api/sync/enqueue", post(sync_handler::enqueue))
        .route(
            "/api/sync/process",
            post(sync_handler::process_queue_handler),
        )
        .route("/api/sync/status", get(sync_handler::get_status))
        // Prefetch
        .route("/api/prefetch", get(prefetch_handler::prefetch))
        .route("/api/prefetch/delta", get(prefetch_handler::delta_sync))
        // Health
        .route("/api/health", get(health_handler::health_check))
        // ── Blockchain Explorer ──────────────────────────────────
        .route("/api/blockchain/status", get(blockchain_handler::status))
        .route("/api/blockchain/stats", get(blockchain_handler::stats))
        .route("/api/blockchain/tx/{hash}", get(blockchain_handler::get_tx))
        .route("/api/blockchain/block/{number}", get(blockchain_handler::get_block))
        .route("/api/blockchain/explorer/search/{query}", get(blockchain_handler::explorer_search))
        .route("/api/blockchain/explorer/hash/{tx_hash}/trace", get(blockchain_handler::explorer_trace))
        .route("/api/blockchain/explorer/recent", get(blockchain_handler::explorer_recent))
        .route("/api/blockchain/explorer/activities/recent", get(blockchain_handler::explorer_activities_recent))
        .route("/api/blockchain/activity/{id}", get(blockchain_handler::get_activity))
        .route("/api/blockchain/activity/verify", post(blockchain_handler::verify_activity))
        .with_state(state)
}
