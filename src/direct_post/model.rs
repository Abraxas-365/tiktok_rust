use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    FileUpload,
    PullFromUrl,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrivacyLevel {
    PublicToEveryone,
    MutualFollowFriends,
    FollowerOfCreator,
    SelfOnly,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into))]
pub struct PostInfo {
    pub title: String,
    pub privacy_level: PrivacyLevel,
    pub disable_duet: bool,
    pub disable_comment: bool,
    pub disable_stitch: bool,
    pub video_cover_timestamp_ms: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into))]
pub struct SourceInfo {
    pub source: Source,
    pub video_size: Option<u64>,
    pub chunk_size: Option<u64>,
    pub total_chunk_count: Option<u32>,
    pub video_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into))]
pub struct VideoInitRequest {
    pub post_info: PostInfo,
    pub source_info: SourceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInitResponseData {
    pub publish_id: String,
    pub upload_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInitResponse {
    pub data: VideoInitResponseData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostStatusResponse {
    pub data: PostStatusData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostStatusData {
    pub publish_id: String,
    pub status: String,
}
