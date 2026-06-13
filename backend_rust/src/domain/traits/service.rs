use serde_json::Value;

pub trait MlService: Send + Sync {
    fn predict_feed(
        &self,
        input: Value,
    ) -> impl std::future::Future<Output = Result<Value, anyhow::Error>> + Send;

    fn evaluate_health(
        &self,
        input: Value,
    ) -> impl std::future::Future<Output = Result<Value, anyhow::Error>> + Send;
}

pub trait BlockchainService: Send + Sync {
    fn submit_transaction(
        &self,
        reference_type: &str,
        reference_id: uuid::Uuid,
        payload: Value,
    ) -> impl std::future::Future<Output = Result<String, anyhow::Error>> + Send;

    fn verify_transaction(
        &self,
        tx_hash: &str,
    ) -> impl std::future::Future<Output = Result<Value, anyhow::Error>> + Send;
}

pub trait CryptoService: Send + Sync {
    fn generate_keypair(&self) -> Result<(String, String), anyhow::Error>;

    fn sign_payload(
        &self,
        private_key: &str,
        payload: &[u8],
    ) -> Result<String, anyhow::Error>;

    fn verify_signature(
        &self,
        public_key: &str,
        payload: &[u8],
        signature: &str,
    ) -> Result<bool, anyhow::Error>;
}
