use reqwest::Client;

use crate::error::TikTokApiError;

use super::{UserInfo, UserInfoResponse};

pub struct Service {
    base_url: String,
}

impl Service {
    /// Creates a new instance of the Service with the client key and secret from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if the `TIKTOK_CLIENT_KEY` or `TIKTOK_CLIENT_SECRET` environment variables are not set.
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
}

impl Service {
    pub async fn get_user_info(
        &self,
        access_token: &str,
        fields: Vec<&str>,
    ) -> Result<UserInfo, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/user/info/", self.base_url);

        let fields_str = fields.join(",");

        let response = client
            .get(&url)
            .query(&[("fields", fields_str)])
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let user_info_response: UserInfoResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && user_info_response.error.code == "ok" {
            Ok(user_info_response.data.user)
        } else {
            Err(TikTokApiError::from(user_info_response.error))
        }
    }
}
