use serde::{Deserialize, Serialize};

use crate::{error::ErrorResponse, videos::Video};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequest {
    pub client_key: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
    pub code_verifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub client_key: String,
    pub client_secret: String,
    pub grant_type: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub open_id: String,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResponse {
    #[serde(default)]
    pub data: UserInfoData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserInfoData {
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub bio_description: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub is_verified: bool,
    #[serde(default)]
    pub follower_count: i32,
    #[serde(default)]
    pub following_count: i32,
    #[serde(default)]
    pub likes_count: i32,
    #[serde(default)]
    pub video_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LikedVideosRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LikedVideosResponse {
    pub data: UserLikedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLikedVideosData {
    pub user_liked_videos: Vec<Video>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinnedVideosRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinnedVideosResponse {
    pub data: PinnedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinnedVideosData {
    pub user_pinned_videos: Vec<Video>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowersRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowersResponse {
    pub data: UserFollowerData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowerData {
    pub user_followers: Vec<UserInfo>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub display_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowingRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowingResponse {
    pub data: UserFollowingData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowingData {
    pub user_following: Vec<UserInfo>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepostedVideosRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepostedVideosResponse {
    pub data: RepostedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepostedVideosData {
    pub user_reposted_videos: Vec<Video>,
    pub cursor: i64,
    pub has_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_with_empty_data() {
        let json_data = r#"{"data":{},"error":{"code":"access_token_invalid","message":"The access token is invalid or not found in the request.","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: UserInfoResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.data.display_name, "");
        assert_eq!(response.data.bio_description, "");
        assert_eq!(response.data.avatar_url, "");
        assert_eq!(response.data.is_verified, false);
        assert_eq!(response.data.follower_count, 0);
        assert_eq!(response.data.following_count, 0);
        assert_eq!(response.data.likes_count, 0);
        assert_eq!(response.data.video_count, 0);
        assert_eq!(response.error.code, "access_token_invalid");
    }

    #[test]
    fn test_deserialize_with_data() {
        let json_data = r#"{"data":{"display_name":"John Doe","bio_description":"This is a bio","avatar_url":"http://example.com/avatar.jpg","is_verified":true,"follower_count":1000,"following_count":500,"likes_count":3000,"video_count":50},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: UserInfoResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.data.display_name, "John Doe");
        assert_eq!(response.data.bio_description, "This is a bio");
        assert_eq!(response.data.avatar_url, "http://example.com/avatar.jpg");
        assert_eq!(response.data.is_verified, true);
        assert_eq!(response.data.follower_count, 1000);
        assert_eq!(response.data.following_count, 500);
        assert_eq!(response.data.likes_count, 3000);
        assert_eq!(response.data.video_count, 50);
        assert_eq!(response.error.code, "none");
    }
}
