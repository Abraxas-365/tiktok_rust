use super::{QueryRequest, QueryVideoResponse, QueryVideoResponseData};
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

    /// Queries videos using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `request` - A `QueryRequest` struct that holds the query parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `QueryVideoResponseData` on success, or a `TikTokApiError` on failure.
    pub async fn query_videos(
        &self,
        token: &str,
        request: QueryRequest,
    ) -> Result<QueryVideoResponseData, TikTokApiError> {
        let client = Client::new();
        let url = format!(
            "{}/v2/research/video/query/?fields=id,video_description,create_time",
            self.base_url
        );

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
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
