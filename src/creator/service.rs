use reqwest::Client;

use crate::error::TikTokApiError;

use super::{CreatorData, CreatorInfoResponse};

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
    // curl --location --request POST 'https://open.tiktokapis.com/v2/post/publish/creator_info/query/' \
    // --header 'Authorization: Bearer act.example12345Example12345Example' \
    // --header 'Content-Type: application/json; charset=UTF-8'
    pub async fn get_creator_info(&self) -> Result<CreatorData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/creator_info/query/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await
            .map_err(|e| {
                TikTokApiError::Unknown("request_failed".into(), e.to_string(), "".into())
            })?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            TikTokApiError::Unknown("response_read_failed".into(), e.to_string(), "".into())
        })?;

        let creator_info_response: CreatorInfoResponse =
            serde_json::from_str(&body).map_err(|e| {
                TikTokApiError::Unknown("parse_failed".into(), e.to_string(), "".into())
            })?;

        if status.is_success() && creator_info_response.error.code == "ok" {
            Ok(creator_info_response.data)
        } else {
            Err(TikTokApiError::from(creator_info_response.error))
        }
    }
}
