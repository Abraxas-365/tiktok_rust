use reqwest::Client;

use crate::error::TikTokApiError;

use super::{VideoInitRequest, VideoInitResponse, VideoInitResponseData};

pub struct Service {
    token: String,
    base_url: String,
}

impl Service {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.into(),
            base_url: String::from("https://open.tiktokapis.com"),
        }
    }
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }
}

impl Service {
    pub async fn post_video(
        &self,
        video_init_request: VideoInitRequest,
    ) -> Result<VideoInitResponseData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/video/init/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&video_init_request)
            .send()
            .await
            .map_err(|e| {
                TikTokApiError::Unknown("request_failed".into(), e.to_string(), "".into())
            })?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            TikTokApiError::Unknown("response_read_failed".into(), e.to_string(), "".into())
        })?;

        let video_init_response: VideoInitResponse = serde_json::from_str(&body).map_err(|e| {
            TikTokApiError::Unknown("parse_failed".into(), e.to_string(), "".into())
        })?;

        if status.is_success() && video_init_response.error.code == "ok" {
            Ok(video_init_response.data)
        } else {
            Err(TikTokApiError::from(video_init_response.error))
        }
    }
}
