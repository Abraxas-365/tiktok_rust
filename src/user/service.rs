use crate::{
    error::{ErrorResponse, TikTokApiError},
    videos::VideoField,
};
use reqwest::Client;
use std::env;

use super::{
    AccessTokenResponse, LikedVideosRequest, LikedVideosResponse, PinnedVideosData,
    PinnedVideosRequest, PinnedVideosResponse, RepostedVideosData, RepostedVideosRequest,
    RepostedVideosResponse, UserFollowerData, UserFollowersRequest, UserFollowersResponse,
    UserFollowingData, UserFollowingRequest, UserFollowingResponse, UserInfoData, UserInfoRequest,
    UserInfoResponse, UserLikedVideosData,
};

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

impl Service {
    /// Queries user info using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `fields` - A list of `VideoField` enums for the desired data.
    /// * `request` - A `UserInfoRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `UserInfoData` on success, or a `TikTokApiError` on failure.
    pub async fn query_user_info(
        &self,
        token: &str,
        fields: &[VideoField],
        request: UserInfoRequest,
    ) -> Result<UserInfoData, TikTokApiError> {
        let client = Client::new();
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "{}/v2/research/user/info/?fields={}",
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

        let user_info_response: UserInfoResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && user_info_response.error.code == "ok" {
            Ok(user_info_response.data)
        } else {
            Err(TikTokApiError::from(user_info_response.error))
        }
    }

    /// Queries liked videos using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `fields` - A list of `VideoField` enums for the desired data.
    /// * `request` - A `LikedVideosRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `UserLikedVideosData` on success, or a `TikTokApiError` on failure.
    pub async fn query_liked_videos(
        &self,
        token: &str,
        fields: &[VideoField],
        request: LikedVideosRequest,
    ) -> Result<UserLikedVideosData, TikTokApiError> {
        let client = Client::new();
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "{}/v2/research/user/liked_videos/?fields={}",
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

        let liked_videos_response: LikedVideosResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && liked_videos_response.error.code == "ok" {
            Ok(liked_videos_response.data)
        } else {
            Err(TikTokApiError::from(liked_videos_response.error))
        }
    }

    /// Queries pinned videos using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `fields` - A list of `VideoField` enums for the desired data.
    /// * `request` - A `PinnedVideosRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PinnedVideosData` on success, or a `TikTokApiError` on failure.
    pub async fn query_pinned_videos(
        &self,
        token: &str,
        fields: &[VideoField],
        request: PinnedVideosRequest,
    ) -> Result<PinnedVideosData, TikTokApiError> {
        let client = Client::new();
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "{}/v2/research/user/pinned_videos/?fields={}",
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

        let pinned_videos_response: PinnedVideosResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && pinned_videos_response.error.code == "ok" {
            Ok(pinned_videos_response.data)
        } else {
            Err(TikTokApiError::from(pinned_videos_response.error))
        }
    }

    /// Queries user followers using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `request` - A `UserFollowersRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `UserFollowerData` on success, or a `TikTokApiError` on failure.
    pub async fn query_user_followers(
        &self,
        token: &str,
        request: UserFollowersRequest,
    ) -> Result<UserFollowerData, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/research/user/followers/", self.base_url);

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

        let user_followers_response: UserFollowersResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && user_followers_response.error.code == "ok" {
            Ok(user_followers_response.data)
        } else {
            Err(TikTokApiError::from(user_followers_response.error))
        }
    }

    /// Queries user following using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `request` - A `UserFollowingRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `UserFollowingData` on success, or a `TikTokApiError` on failure.
    pub async fn query_user_following(
        &self,
        token: &str,
        request: UserFollowingRequest,
    ) -> Result<UserFollowingData, TikTokApiError> {
        let client = Client::new();
        let url = format!("{}/v2/research/user/following/", self.base_url);

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

        let user_following_response: UserFollowingResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && user_following_response.error.code == "ok" {
            Ok(user_following_response.data)
        } else {
            Err(TikTokApiError::from(user_following_response.error))
        }
    }

    /// Queries reposted videos using the TikTok API.
    ///
    /// # Arguments
    ///
    /// * `token` - The client access token.
    /// * `fields` - A list of `VideoField` enums for the desired data.
    /// * `request` - A `RepostedVideosRequest` struct that holds the request parameters.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `RepostedVideosData` on success, or a `TikTokApiError` on failure.
    pub async fn query_reposted_videos(
        &self,
        token: &str,
        fields: &[VideoField],
        request: RepostedVideosRequest,
    ) -> Result<RepostedVideosData, TikTokApiError> {
        let client = Client::new();
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = format!(
            "{}/v2/research/user/reposted_videos/?fields={}",
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

        let reposted_videos_response: RepostedVideosResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && reposted_videos_response.error.code == "ok" {
            Ok(reposted_videos_response.data)
        } else {
            Err(TikTokApiError::from(reposted_videos_response.error))
        }
    }
}
