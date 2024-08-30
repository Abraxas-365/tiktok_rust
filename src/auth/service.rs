use reqwest::Client;

use crate::error::{ErrorResponse, TikTokApiError};

use super::{AccessTokenResponse, AuthCallback, OAuthConfig};

pub struct AuthService {
    config: OAuthConfig,
    client: Client,
    base_url: String,
}

impl AuthService {
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            base_url: String::from("https://open.tiktokapis.com"),
        }
    }

    pub fn get_authorization_url(&self) -> String {
        self.config.authorization_url()
    }

    pub fn validate_callback(&self, callback: &AuthCallback) -> Result<String, String> {
        // Verify CSRF state
        if callback.state.as_deref() != Some(&self.config.csrf_state) {
            return Err("Invalid CSRF state".to_string());
        }

        match (&callback.code, &callback.error) {
            (Some(code), None) => Ok(format!(
                "Authorization successful. Code: {}. Use this code to get an access token.",
                code
            )),
            (None, Some(error)) => Err(format!(
                "Authorization failed. Error: {}. Description: {:?}",
                error, callback.error_description
            )),
            _ => Err("Invalid callback parameters".to_string()),
        }
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
        let url = format!("{}/v2/oauth/token/", self.base_url);

        let client_key = self.config.client_key.clone();
        let client_secret = self.config.client_secret.clone();
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

        let response = self
            .client
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
        let url = format!("{}/v2/oauth/token/", self.base_url);

        let params = [
            ("client_key", &self.config.client_key),
            ("client_secret", &self.config.client_secret),
            ("grant_type", &"refresh_token".to_string()),
            ("refresh_token", &refresh_token.to_string()),
        ];

        let response = self
            .client
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
