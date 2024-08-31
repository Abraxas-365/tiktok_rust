use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserInfo {
    #[serde(default)]
    pub open_id: Option<String>,
    #[serde(default)]
    pub union_id: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub avatar_url_100: Option<String>,
    #[serde(default)]
    pub avatar_large_url: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub bio_description: Option<String>,
    #[serde(default)]
    pub profile_deep_link: Option<String>,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub follower_count: Option<i64>,
    #[serde(default)]
    pub following_count: Option<i64>,
    #[serde(default)]
    pub likes_count: Option<i64>,
    #[serde(default)]
    pub video_count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserInfoData {
    #[serde(default)]
    pub user: UserInfo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserInfoResponse {
    #[serde(default)]
    pub data: UserInfoData,
    pub error: ErrorResponse,
}
