use crate::error::TikTokApiError;

use reqwest::Client;

use super::{
    ListVideoRequest, ListVideoResponse, QueryVideoRequest, QueryVideoResponse,
    UserVideoListPostResponseData, Video, VideoField, VideoFilters,
};

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

    /// Query videos for the authenticated user.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The OAuth access token for the authenticated user.
    /// * `video_ids` - A vector of video IDs to query.
    /// * `fields` - A vector of VideoField enum values to request.
    ///
    /// # Returns
    ///
    /// Returns a Result containing a Vec<Video> if successful, or a TikTokApiError if an error occurs.
    pub async fn query_videos(
        &self,
        access_token: &str,
        video_ids: Vec<String>,
        fields: Vec<VideoField>,
    ) -> Result<Vec<Video>, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/video/query/", self.base_url);

        let fields_str = fields
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let request_body = QueryVideoRequest {
            filters: VideoFilters { video_ids },
        };

        let response = client
            .post(&url)
            .query(&[("fields", fields_str)])
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
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
            Ok(query_video_response.data.videos)
        } else {
            Err(TikTokApiError::from(query_video_response.error))
        }
    }

    /// List videos for the authenticated user.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The OAuth access token for the authenticated user.
    /// * `cursor` - Optional cursor for pagination.
    /// * `max_count` - Optional maximum number of videos to return (default is 10, maximum is 20).
    /// * `fields` - A vector of VideoField enum values to request.
    ///
    /// # Returns
    ///
    /// Returns a Result containing UserVideoListPostResponseData if successful, or a TikTokApiError if an error occurs.
    pub async fn list_videos(
        &self,
        access_token: &str,
        cursor: Option<i64>,
        max_count: Option<i32>,
        fields: Vec<VideoField>,
    ) -> Result<UserVideoListPostResponseData, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/video/list/", self.base_url);

        let fields_str = fields
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join(",");

        let request_body = ListVideoRequest { cursor, max_count };

        let response = client
            .post(&url)
            .query(&[("fields", fields_str)])
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let list_video_response: ListVideoResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && list_video_response.error.code == "ok" {
            Ok(list_video_response.data)
        } else {
            Err(TikTokApiError::from(list_video_response.error))
        }
    }
}
