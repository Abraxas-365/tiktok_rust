use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoField {
    Id,
    CreateTime,
    Username,
    RegionCode,
    VideoDescription,
    MusicId,
    LikeCount,
    CommentCount,
    ShareCount,
    ViewCount,
    HashtagNames,
    IsStemVerified,
    FavouritesCount,
    VideoDuration,
}

impl VideoField {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoField::Id => "id",
            VideoField::CreateTime => "create_time",
            VideoField::Username => "username",
            VideoField::RegionCode => "region_code",
            VideoField::VideoDescription => "video_description",
            VideoField::MusicId => "music_id",
            VideoField::LikeCount => "like_count",
            VideoField::CommentCount => "comment_count",
            VideoField::ShareCount => "share_count",
            VideoField::ViewCount => "view_count",
            VideoField::HashtagNames => "hashtag_names",
            VideoField::IsStemVerified => "is_stem_verified",
            VideoField::FavouritesCount => "favourites_count",
            VideoField::VideoDuration => "video_duration",
        }
    }
}

impl ToString for VideoField {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

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
    pub data: UserInfoData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoData {
    pub display_name: String,
    pub bio_description: String,
    pub avatar_url: String,
    pub is_verified: bool,
    pub follower_count: i32,
    pub following_count: i32,
    pub likes_count: i32,
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
pub struct Video {
    pub id: i64,
    pub create_time: i64,
    pub username: String,
    pub region_code: String,
    pub video_description: String,
    pub music_id: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub share_count: i64,
    pub view_count: i64,
    pub hashtag_names: Vec<String>,
    pub video_duration: i64,
    pub is_stem_verified: bool,
    pub favorites_count: i64,
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
