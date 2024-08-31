use crate::{error::TikTokApiError, videos::VideoField};
use reqwest::Client;

use super::{
    ResearchLikedVideosRequest, ResearchLikedVideosResponse, ResearchPinnedVideosData,
    ResearchPinnedVideosRequest, ResearchPinnedVideosResponse, ResearchRepostedVideosData,
    ResearchRepostedVideosRequest, ResearchRepostedVideosResponse, ResearchUserFollowerData,
    ResearchUserFollowersRequest, ResearchUserFollowersResponse, ResearchUserFollowingData,
    ResearchUserFollowingRequest, ResearchUserFollowingResponse, ResearchUserInfoData,
    ResearchUserInfoRequest, ResearchUserInfoResponse, ResearchUserLikedVideosData,
};

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
        request: ResearchUserInfoRequest,
    ) -> Result<ResearchUserInfoData, TikTokApiError> {
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

        let user_info_response: ResearchUserInfoResponse =
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
        request: ResearchLikedVideosRequest,
    ) -> Result<ResearchUserLikedVideosData, TikTokApiError> {
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

        let liked_videos_response: ResearchLikedVideosResponse =
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
        request: ResearchPinnedVideosRequest,
    ) -> Result<ResearchPinnedVideosData, TikTokApiError> {
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

        let pinned_videos_response: ResearchPinnedVideosResponse =
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
        request: ResearchUserFollowersRequest,
    ) -> Result<ResearchUserFollowerData, TikTokApiError> {
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

        let user_followers_response: ResearchUserFollowersResponse =
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
        request: ResearchUserFollowingRequest,
    ) -> Result<ResearchUserFollowingData, TikTokApiError> {
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

        let user_following_response: ResearchUserFollowingResponse =
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
        request: ResearchRepostedVideosRequest,
    ) -> Result<ResearchRepostedVideosData, TikTokApiError> {
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

        let reposted_videos_response: ResearchRepostedVideosResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && reposted_videos_response.error.code == "ok" {
            Ok(reposted_videos_response.data)
        } else {
            Err(TikTokApiError::from(reposted_videos_response.error))
        }
    }
}
