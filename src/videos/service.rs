use std::env;

use reqwest::Client;

use crate::error::TikTokApiError;

use super::{QueryRequest, QueryVideoResponse, QueryVideoResponseData};

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
    /// Queries videos using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `request` - A `QueryRequest` struct that holds the query parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `QueryVideoResponseData` on success, or a `TikTokApiError` on failure.
    pub async fn query_videos(
        &self,
        request: QueryRequest,
    ) -> Result<QueryVideoResponseData, TikTokApiError> {
        let client = Client::new();
        let url = format!(
            "{}/v2/research/video/query/?fields=id,video_description,create_time",
            self.base_url
        );

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let query_video_response: QueryVideoResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && query_video_response.error.code == "ok" {
            Ok(query_video_response.data)
        } else {
            Err(TikTokApiError::from(query_video_response.error))
        }
    }
}
