use super::{CreatorData, CreatorInfoResponse};
use crate::error::TikTokApiError;
use reqwest::Client;

pub struct Service {
    base_url: String,
}

impl Service {
    /// Creates a new instance of the Service.
    pub fn new() -> Self {
        Self {
            base_url: String::from("https://open.tiktokapis.com"),
        }
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

    /// Retrieves creator information from the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `CreatorData` on success, or a `TikTokApiError` on failure.
    pub async fn get_creator_info(&self, token: &str) -> Result<CreatorData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/creator_info/query/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
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
