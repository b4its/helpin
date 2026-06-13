use anyhow::{anyhow, Result};
use serde_json::Value;
use std::time::Duration;

use crate::domain::traits::service::MlService;

#[derive(Clone)]
pub struct MlClientService {
    client: reqwest::Client,
    base_url: String,
}

impl MlClientService {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }
}

impl MlService for MlClientService {
    async fn predict_feed(&self, input: Value) -> Result<Value> {
        let url = format!("{}/predict/feed", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&input)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow!("ML service feed prediction request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "ML service returned error status: {}",
                response.status()
            ));
        }

        let result = response
            .json::<Value>()
            .await
            .map_err(|e| anyhow!("Failed to parse ML feed response: {}", e))?;

        Ok(result)
    }

    async fn evaluate_health(&self, input: Value) -> Result<Value> {
        let url = format!("{}/predict/health", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&input)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow!("ML service health evaluation request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "ML service returned error status: {}",
                response.status()
            ));
        }

        let result = response
            .json::<Value>()
            .await
            .map_err(|e| anyhow!("Failed to parse ML health response: {}", e))?;

        Ok(result)
    }
}
