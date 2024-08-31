use crate::error::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum VideoField {
    Id,
    CreateTime,
    CoverImageUrl,
    ShareUrl,
    VideoDescription,
    Duration,
    Height,
    Width,
    Title,
    EmbedHtml,
    EmbedLink,
    LikeCount,
    CommentCount,
    ShareCount,
    ViewCount,
}

impl VideoField {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoField::Id => "id",
            VideoField::CreateTime => "create_time",
            VideoField::CoverImageUrl => "cover_image_url",
            VideoField::ShareUrl => "share_url",
            VideoField::VideoDescription => "video_description",
            VideoField::Duration => "duration",
            VideoField::Height => "height",
            VideoField::Width => "width",
            VideoField::Title => "title",
            VideoField::EmbedHtml => "embed_html",
            VideoField::EmbedLink => "embed_link",
            VideoField::LikeCount => "like_count",
            VideoField::CommentCount => "comment_count",
            VideoField::ShareCount => "share_count",
            VideoField::ViewCount => "view_count",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoFilters {
    pub video_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryVideoRequest {
    pub filters: VideoFilters,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Video {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub cover_image_url: Option<String>,
    #[serde(default)]
    pub share_url: Option<String>,
    #[serde(default)]
    pub video_description: Option<String>,
    #[serde(default)]
    pub duration: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub embed_html: Option<String>,
    #[serde(default)]
    pub embed_link: Option<String>,
    #[serde(default)]
    pub like_count: Option<i32>,
    #[serde(default)]
    pub comment_count: Option<i32>,
    #[serde(default)]
    pub share_count: Option<i32>,
    #[serde(default)]
    pub view_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QueryUserVideoResponseData {
    #[serde(default)]
    pub videos: Vec<Video>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryVideoResponse {
    #[serde(default)]
    pub data: QueryUserVideoResponseData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListVideoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserVideoListPostResponseData {
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
    pub cursor: i64,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListVideoResponse {
    #[serde(default)]
    pub data: UserVideoListPostResponseData,
    pub error: ErrorResponse,
}
