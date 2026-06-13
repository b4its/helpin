pub mod config;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

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
use crate::presentation::routes::{AppState, create_router};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with env filter (default RUST_LOG=info)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load configuration from environment
    let config = Config::from_env();

    // Create PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    tracing::info!("Database connection pool created");

    // Jalankan migrasi idempoten saat startup agar skema selalu sinkron
    // (aman dijalankan berulang kali, baik volume baru maupun lama).
    run_startup_migrations(&pool).await;

    // Initialize repositories
    let user_repo = PostgresUserRepository::new(pool.clone());
    let livestock_repo = PostgresLivestockRepository::new(pool.clone());
    let land_repo = PostgresLandRepository::new(pool.clone());
    let inventory_repo = PostgresInventoryRepository::new(pool.clone());
    let product_repo = PostgresProductRepository::new(pool.clone());
    let order_repo = PostgresOrderRepository::new(pool.clone());
    let transaction_repo = PostgresTransactionRepository::new(pool.clone());
    let financial_repo = PostgresFinancialRepository::new(pool.clone());
    let sync_queue_repo = PostgresSyncQueueRepository::new(pool.clone());

    // Initialize services
    let jwt_service = JwtService::new(config.jwt_secret.clone());
    let crypto_service = Ed25519CryptoService::new();
    let ml_client = MlClientService::new(config.ml_service_url.clone());
    let blockchain_service = HyperledgerService::new(config.blockchain_rpc_url.clone());

    // Connect MongoDB (untuk data treatment fleksibel; non-fatal jika gagal)
    let mongo = MongoService::connect(&config.mongodb_uri).await;

    // Build AppState
    let state = AppState {
        pool: pool.clone(),
        user_repo,
        livestock_repo,
        land_repo,
        inventory_repo,
        product_repo,
        order_repo,
        transaction_repo,
        financial_repo,
        sync_queue_repo,
        jwt_service: jwt_service.clone(),
        crypto_service,
        ml_client,
        blockchain_service,
        mongo,
        config: config.clone(),
    };

    // Setup CORS middleware (permissive for development)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers(Any)
        .allow_credentials(false);

    // Setup gzip compression
    let compression = CompressionLayer::new();

    // Build the router with all application routes
    let app = create_router(state)
        .layer(axum::Extension(jwt_service))
        .layer(cors)
        .layer(compression);

    // Bind to configured host:port
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    // Start the server
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .expect("Server error");
}

/// Migrasi idempoten yang dijalankan saat startup.
/// Memastikan tabel/kolom untuk fitur estimasi panen & treatment selalu ada,
/// tanpa bergantung pada init-script Docker (yang hanya jalan di volume baru).
async fn run_startup_migrations(pool: &sqlx::PgPool) {
    let statements = [
        // Junction inventori <-> lahan
        r#"CREATE TABLE IF NOT EXISTS land_inventories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            land_id UUID NOT NULL REFERENCES lands(id) ON DELETE CASCADE,
            inventory_id UUID NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE (land_id, inventory_id)
        )"#,
        "CREATE INDEX IF NOT EXISTS idx_land_inventories_land_id ON land_inventories(land_id)",
        // Kolom estimasi panen di harvests
        "ALTER TABLE harvests ADD COLUMN IF NOT EXISTS is_estimate BOOLEAN NOT NULL DEFAULT FALSE",
        "ALTER TABLE harvests ADD COLUMN IF NOT EXISTS predicted_harvest_date DATE",
        "ALTER TABLE harvests ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW()",
        // Pastikan kolom yang dipakai entity lain ada (defensif)
        "ALTER TABLE inventories ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES users(id)",
        "ALTER TABLE lands ADD COLUMN IF NOT EXISTS crop_type VARCHAR(100)",
        // Peternak: usia ternak, kondisi kandang (auto-generate), nutrisi/kandungan produk
        "ALTER TABLE livestock ADD COLUMN IF NOT EXISTS age_months INTEGER",
        "ALTER TABLE pens ADD COLUMN IF NOT EXISTS condition VARCHAR(50)",
        "ALTER TABLE products ADD COLUMN IF NOT EXISTS nutrition JSONB",
        // Ternak fleksibel: jenis bebas (unggas/mamalia/serangga/dll), gender & weight opsional
        "ALTER TABLE livestock DROP CONSTRAINT IF EXISTS livestock_category_check",
        "ALTER TABLE livestock DROP CONSTRAINT IF EXISTS livestock_gender_check",
        "ALTER TABLE livestock ALTER COLUMN category TYPE VARCHAR(50)",
        "ALTER TABLE livestock ALTER COLUMN gender DROP NOT NULL",
        "ALTER TABLE livestock ALTER COLUMN weight DROP NOT NULL",
        // Admin: role karyawan, kolom phone, tabel suppliers, produk harga beli/expired/entry
        "ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_check",
        "ALTER TABLE users ADD COLUMN IF NOT EXISTS phone VARCHAR(50)",
        "CREATE TABLE IF NOT EXISTS suppliers (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), name VARCHAR(200) NOT NULL, contact VARCHAR(100), commodity VARCHAR(150), address TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW())",
        "ALTER TABLE products ADD COLUMN IF NOT EXISTS purchase_price BIGINT DEFAULT 0",
        "ALTER TABLE products ADD COLUMN IF NOT EXISTS expired_at DATE",
        "ALTER TABLE products ADD COLUMN IF NOT EXISTS entry_date DATE DEFAULT CURRENT_DATE",
    ];

    for stmt in statements {
        if let Err(e) = sqlx::query(stmt).execute(pool).await {
            tracing::warn!("Startup migration statement gagal (mungkin sudah ada): {}", e);
        }
    }
    tracing::info!("Startup migrations applied");
}
