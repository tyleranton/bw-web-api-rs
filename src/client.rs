use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use crate::config::ApiConfig;
use crate::endpoints::Endpoint;
use crate::error::ApiError;

pub struct ApiClient {
    client: reqwest::Client,
    config: ApiConfig,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ApiError::ClientCreationError(e.to_string()))?;

        Ok(Self { client, config })
    }

    pub async fn request<E>(
        &self,
        endpoint: &E,
        params: &E::Request,
    ) -> Result<E::Response, ApiError>
    where
        E: Endpoint,
        E::Request: Serialize,
        E::Response: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, endpoint.endpoint());

        let mut request_builder = self.client.get(&url);
        request_builder = request_builder.header("Accept", "application/json");

        if let Some(ref api_key) = self.config.api_key {
            request_builder = request_builder.header("x-api-key", format!("Bearer {}", api_key));
        }

        let response = request_builder.query(params).send().await?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let text = response.text().await?;

        serde_json::from_str(&text).map_err(|e| {
            ApiError::DeserializationError(format!(
                "Failed to deserialize: {}. Response starts with: {}",
                e,
                &text[..text.len().min(200)]
            ))
        })
    }
}
