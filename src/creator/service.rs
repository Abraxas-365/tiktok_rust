use std::env;

use reqwest::Client;

use crate::error::TikTokApiError;

use super::{CreatorData, CreatorInfoResponse};

pub struct Service {
    token: String,
    base_url: String,
}

impl Service {
    /// Creates a new instance of the Service with the token from the environment variable `TIKTOK_API_TOKEN`.
    ///
    /// # Panics
    ///
    /// Panics if the `TIKTOK_API_TOKEN` environment variable is not set.
    pub fn new() -> Self {
        let token = env::var("TIKTOK_API_TOKEN").expect("TIKTOK_API_TOKEN must be set");
        Self {
            token,
            base_url: String::from("https://open.tiktokapis.com"),
        }
    }

    ///Sets a token for the Service
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the API token.
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = token.into();
        self
    }

    /// Sets a custom base URL for the Service.
    ///
    /// # Arguments
    ///
    /// * `base_url` - A string slice that holds the custom base URL.
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }
}

impl Service {
    /// Retrieves creator information from the TikTok API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `CreatorData` on success, or a `TikTokApiError` on failure.
    pub async fn get_creator_info(&self) -> Result<CreatorData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/creator_info/query/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let creator_info_response: CreatorInfoResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && creator_info_response.error.code == "ok" {
            Ok(creator_info_response.data)
        } else {
            Err(TikTokApiError::from(creator_info_response.error))
        }
    }
}
