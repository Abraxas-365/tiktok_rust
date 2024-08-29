use crate::error::{ErrorResponse, TikTokApiError};
use reqwest::Client;
use std::env;

use super::AccessTokenResponse;

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

    /// Sets a custom base URL for the Service.
    ///
    /// # Arguments
    ///
    /// * `base_url` - A string slice that holds the custom base URL.
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Fetches an access token using an authorization code.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code from the callback.
    /// * `redirect_uri` - The redirect URI used for requesting the code.
    /// * `code_verifier` - The code verifier used in PKCE authorization flow (optional).
    ///
    /// # Returns
    ///
    /// * `Result<AccessTokenResponse, TikTokApiError>` - The access token response or an error.
    pub async fn fetch_access_token(
        &self,
        code: &str,
        redirect_uri: &str,
        code_verifier: Option<&str>,
    ) -> Result<AccessTokenResponse, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/oauth/token/", self.base_url);

        let client_key = self.client_key.clone();
        let client_secret = self.client_secret.clone();
        let code = code.to_string();
        let grant_type = "authorization_code".to_string();
        let redirect_uri = redirect_uri.to_string();

        let mut params = vec![
            ("client_key", client_key),
            ("client_secret", client_secret),
            ("code", code),
            ("grant_type", grant_type),
            ("redirect_uri", redirect_uri),
        ];

        if let Some(verifier) = code_verifier {
            params.push(("code_verifier", verifier.to_string()));
        }

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
            let token_response: AccessTokenResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Ok(token_response)
        } else {
            let error_response: ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Err(TikTokApiError::from(error_response))
        }
    }

    /// Refreshes an access token using a refresh token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The user's refresh token.
    ///
    /// # Returns
    ///
    /// * `Result<AccessTokenResponse, TikTokApiError>` - The new access token response or an error.
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<AccessTokenResponse, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/oauth/token/", self.base_url);

        let params = [
            ("client_key", &self.client_key),
            ("client_secret", &self.client_secret),
            ("grant_type", &"refresh_token".to_string()),
            ("refresh_token", &refresh_token.to_string()),
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
            let token_response: AccessTokenResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Ok(token_response)
        } else {
            let error_response: ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Err(TikTokApiError::from(error_response))
        }
    }
}
