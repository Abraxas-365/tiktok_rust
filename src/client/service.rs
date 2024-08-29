use std::env;

use reqwest::Client;

use crate::error::{ErrorResponse, TikTokApiError};

use super::TokenResponse;

pub struct Service {
    client_key: String,
    client_secret: String,
    base_url: String,
}

impl Service {
    /// Creates a new instance of the Service with the client key and secret from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if the `TIKTOK_CLIENT_KEY` or `TIKTOK_CLIENT_SECRET` environment variables are not set.
    pub fn new() -> Self {
        let client_key = env::var("TIKTOK_CLIENT_KEY").expect("TIKTOK_CLIENT_KEY must be set");
        let client_secret =
            env::var("TIKTOK_CLIENT_SECRET").expect("TIKTOK_CLIENT_SECRET must be set");
        Self {
            client_key,
            client_secret,
            base_url: String::from("https://open.tiktokapis.com"),
        }
    }

    /// Creates a new instance of the Service with the provided client key and secret.
    ///
    /// # Arguments
    ///
    /// * `client_key` - A string slice that holds the client key.
    /// * `client_secret` - A string slice that holds the client secret.
    pub fn with_credentials(client_key: &str, client_secret: &str) -> Self {
        Self {
            client_key: client_key.into(),
            client_secret: client_secret.into(),
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

    /// Retrieves an access token from the TikTok API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TokenResponse` on success, or a `TikTokApiError` on failure.
    pub async fn get_access_token(&self) -> Result<TokenResponse, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/oauth/token/", self.base_url);

        let params = [
            ("client_key", &self.client_key),
            ("client_secret", &self.client_secret),
            ("grant_type", &String::from("client_credentials")),
        ];

        let response = client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        if status.is_success() {
            let token_response: TokenResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Ok(token_response)
        } else {
            let error_response: ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Err(TikTokApiError::from(error_response))
        }
    }
}
