use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatorInfoResponse {
    #[serde(default)]
    pub data: CreatorData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreatorData {
    #[serde(default)]
    pub creator_avatar_url: String,
    #[serde(default)]
    pub creator_username: String,
    #[serde(default)]
    pub creator_nickname: String,
    #[serde(default)]
    pub privacy_level_options: Vec<String>,
    #[serde(default)]
    pub comment_disabled: bool,
    #[serde(default)]
    pub duet_disabled: bool,
    #[serde(default)]
    pub stitch_disabled: bool,
    #[serde(default)]
    pub max_video_post_duration_sec: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_with_empty_data() {
        let json_data = r#"{"data":{},"error":{"code":"access_token_invalid","message":"The access token is invalid or not found in the request.","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: CreatorInfoResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.data.creator_avatar_url, "");
        assert_eq!(response.data.creator_username, "");
        assert_eq!(response.data.creator_nickname, "");
        assert!(response.data.privacy_level_options.is_empty());
        assert!(!response.data.comment_disabled);
        assert!(!response.data.duet_disabled);
        assert!(!response.data.stitch_disabled);
        assert_eq!(response.data.max_video_post_duration_sec, 0);
        assert_eq!(response.error.code, "access_token_invalid");
    }

    #[test]
    fn test_deserialize_with_data() {
        let json_data = r#"{"data":{"creator_avatar_url":"http://example.com/avatar.jpg","creator_username":"user123","creator_nickname":"User","privacy_level_options":["option1", "option2"],"comment_disabled":false,"duet_disabled":true,"stitch_disabled":false,"max_video_post_duration_sec":60},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: CreatorInfoResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(
            response.data.creator_avatar_url,
            "http://example.com/avatar.jpg"
        );
        assert_eq!(response.data.creator_username, "user123");
        assert_eq!(response.data.creator_nickname, "User");
        assert_eq!(
            response.data.privacy_level_options,
            vec!["option1", "option2"]
        );
        assert_eq!(response.data.comment_disabled, false);
        assert_eq!(response.data.duet_disabled, true);
        assert_eq!(response.data.stitch_disabled, false);
        assert_eq!(response.data.max_video_post_duration_sec, 60);
        assert_eq!(response.error.code, "none");
    }
}
