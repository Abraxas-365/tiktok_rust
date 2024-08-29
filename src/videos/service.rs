use crate::error::TikTokApiError;

use super::{
    QueryRequest, QueryVideoResponse, QueryVideoResponseData, ResearchVideoCommentsData,
    VideoCommentsRequest, VideoCommentsResponse, VideoField,
};
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
    /// * `fields` - A list of `VideoField` enums for the desired data.
    /// * `request` - A `QueryRequest` struct that holds the query parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `QueryVideoResponseData` on success, or a `TikTokApiError` on failure.
    pub async fn query_videos(
        &self,
        token: &str,
        fields: &[VideoField],
        request: QueryRequest,
    ) -> Result<QueryVideoResponseData, TikTokApiError> {
        let client = Client::new();
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "{}/v2/research/video/query/?fields={}",
            self.base_url, fields_str
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

    /// Queries video comments using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `fields` - A comma-separated list of field names for the desired data.
    /// * `request` - A `VideoCommentsRequest` struct that holds the query parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ResearchVideoCommentsData` on success, or a `TikTokApiError` on failure.
    pub async fn query_video_comments(
        &self,
        token: &str,
        fields: &str,
        request: VideoCommentsRequest,
    ) -> Result<ResearchVideoCommentsData, TikTokApiError> {
        let client = Client::new();
        let url = format!(
            "{}/v2/research/video/comment/list/?fields={}",
            self.base_url, fields
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

        let video_comments_response: VideoCommentsResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && video_comments_response.error.code == "ok" {
            Ok(video_comments_response.data)
        } else {
            Err(TikTokApiError::from(video_comments_response.error))
        }
    }
}
