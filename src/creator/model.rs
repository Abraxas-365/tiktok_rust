use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatorInfoResponse {
    pub data: CreatorData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatorData {
    pub creator_avatar_url: String,
    pub creator_username: String,
    pub creator_nickname: String,
    pub privacy_level_options: Vec<String>,
    pub comment_disabled: bool,
    pub duet_disabled: bool,
    pub stitch_disabled: bool,
    pub max_video_post_duration_sec: u32,
}
