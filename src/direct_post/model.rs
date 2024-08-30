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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostMode {
    DirectPost,
    MediaUpload,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Photo,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub chunk_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub total_chunk_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub photo_cover_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub photo_images: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into))]
pub struct VideoInitRequest {
    pub post_info: PostInfo,
    pub source_info: SourceInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VideoInitResponseData {
    #[serde(default)]
    pub publish_id: String,
    #[serde(default)]
    pub upload_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInitResponse {
    #[serde(default)]
    pub data: VideoInitResponseData,
    pub error: ErrorResponse,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into))]
pub struct PhotoInitRequest {
    pub post_info: PostInfo,
    pub source_info: SourceInfo,
    pub post_mode: PostMode,
    pub media_type: MediaType,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_with_empty_data() {
        let json_data = r#"{"data":{},"error":{"code":"access_token_invalid","message":"The access token is invalid or not found in the request.","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: VideoInitResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.data.publish_id, "");
        assert_eq!(response.data.upload_url, "");
        assert_eq!(response.error.code, "access_token_invalid");
    }

    #[test]
    fn test_deserialize_with_data() {
        let json_data = r#"{"data":{"publish_id":"12345","upload_url":"http://example.com/upload"},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: VideoInitResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.data.publish_id, "12345");
        assert_eq!(response.data.upload_url, "http://example.com/upload");
        assert_eq!(response.error.code, "none");
    }
}
