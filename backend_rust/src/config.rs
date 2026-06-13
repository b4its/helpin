use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub mongodb_uri: String,
    pub jwt_secret: String,
    pub jwt_expiry_minutes: u64,
    pub refresh_token_expiry_days: u64,
    pub ml_service_url: String,
    pub blockchain_rpc_url: String,
    pub openrouter_api_key: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// Attempts to load a `.env` file first (silently ignores if not found).
    /// Panics with a helpful message if a required variable is missing.
    pub fn from_env() -> Self {
        // Load .env file if it exists; ignore errors (file may not exist in production)
        dotenvy::dotenv().ok();

        Self {
            database_url: required_var("DATABASE_URL"),
            mongodb_uri: optional_var("MONGODB_URI", "mongodb://127.0.0.1:27017"),
            jwt_secret: required_var("JWT_SECRET"),
    jwt_expiry_minutes: optional_var_parsed("JWT_EXPIRY_MINUTES", 480), // default 8 jam
            refresh_token_expiry_days: optional_var_parsed("REFRESH_TOKEN_EXPIRY_DAYS", 7),
            ml_service_url: required_var("ML_SERVICE_URL"),
            blockchain_rpc_url: required_var("BLOCKCHAIN_RPC_URL"),
            openrouter_api_key: optional_var("OPENROUTER_API_KEY", ""),
            server_host: optional_var("SERVER_HOST", "0.0.0.0"),
            server_port: optional_var_parsed("SERVER_PORT", 8080),
        }
    }
}

/// Read a required environment variable or panic with a helpful message.
fn required_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        panic!(
            "Environment variable `{}` is required but not set. \
             Please set it in your .env file or system environment.",
            name
        )
    })
}

/// Read an optional environment variable with a default string value.
fn optional_var(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

/// Read an optional environment variable, parsing it to the desired type.
/// Falls back to `default` if the variable is not set.
/// Panics if the variable is set but cannot be parsed.
fn optional_var_parsed<T: std::str::FromStr>(name: &str, default: T) -> T
where
    T::Err: std::fmt::Display,
{
    match env::var(name) {
        Ok(val) => val.parse::<T>().unwrap_or_else(|e| {
            panic!(
                "Environment variable `{}` has invalid value `{}`: {}",
                name, val, e
            )
        }),
        Err(_) => default,
    }
}
