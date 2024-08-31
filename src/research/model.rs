use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserInfoRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserInfoResponse {
    #[serde(default)]
    pub data: ResearchUserInfoData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResearchUserInfoData {
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
pub struct ResearchLikedVideosRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchLikedVideosResponse {
    pub data: ResearchUserLikedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserLikedVideosData {
    pub user_liked_videos: Vec<ResearchVideo>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchPinnedVideosRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchPinnedVideosResponse {
    pub data: ResearchPinnedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchPinnedVideosData {
    pub user_pinned_videos: Vec<ResearchVideo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserFollowersRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserFollowersResponse {
    #[serde(default)]
    pub data: ResearchUserFollowerData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResearchUserFollowerData {
    #[serde(default)]
    pub user_followers: Vec<ResearchUserInfo>,
    #[serde(default)]
    pub cursor: i64,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserInfo {
    pub display_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserFollowingRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserFollowingResponse {
    pub data: ResearchUserFollowingData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchUserFollowingData {
    pub user_following: Vec<ResearchUserInfo>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchRepostedVideosRequest {
    pub username: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchRepostedVideosResponse {
    pub data: ResearchRepostedVideosData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchRepostedVideosData {
    pub user_reposted_videos: Vec<ResearchVideo>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResearchVideoField {
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

impl ResearchVideoField {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResearchVideoField::Id => "id",
            ResearchVideoField::CreateTime => "create_time",
            ResearchVideoField::Username => "username",
            ResearchVideoField::RegionCode => "region_code",
            ResearchVideoField::VideoDescription => "video_description",
            ResearchVideoField::MusicId => "music_id",
            ResearchVideoField::LikeCount => "like_count",
            ResearchVideoField::CommentCount => "comment_count",
            ResearchVideoField::ShareCount => "share_count",
            ResearchVideoField::ViewCount => "view_count",
            ResearchVideoField::HashtagNames => "hashtag_names",
            ResearchVideoField::IsStemVerified => "is_stem_verified",
            ResearchVideoField::FavouritesCount => "favourites_count",
            ResearchVideoField::VideoDuration => "video_duration",
        }
    }
}

impl ToString for ResearchVideoField {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct ResearchCondition {
    pub field_name: String,
    pub operation: String,
    pub field_values: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct ResearchQuery {
    pub and: Option<Vec<ResearchCondition>>,
    pub or: Option<Vec<ResearchCondition>>,
    pub not: Option<Vec<ResearchCondition>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct QueryRequest {
    pub query: ResearchQuery,
    pub start_date: String,
    pub end_date: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
    pub search_id: Option<String>,
    pub is_random: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ResearchVideo {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub create_time: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub region_code: Option<ResearchRegionCode>,
    #[serde(default)]
    pub video_description: Option<String>,
    #[serde(default)]
    pub music_id: Option<i64>,
    #[serde(default)]
    pub like_count: Option<i64>,
    #[serde(default)]
    pub comment_count: Option<i64>,
    #[serde(default)]
    pub share_count: Option<i64>,
    #[serde(default)]
    pub view_count: Option<i64>,
    #[serde(default)]
    pub effect_ids: Option<Vec<String>>,
    #[serde(default)]
    pub hashtag_names: Option<Vec<String>>,
    #[serde(default)]
    pub playlist_id: Option<i64>,
    #[serde(default)]
    pub voice_to_text: Option<String>,
    #[serde(default)]
    pub is_stem_verified: Option<bool>,
    #[serde(default)]
    pub video_duration: Option<i64>,
    #[serde(default)]
    pub favourites_count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ResearchQueryVideoResponseData {
    #[serde(default)]
    pub videos: Vec<ResearchVideo>,
    #[serde(default)]
    pub cursor: i64,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub search_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResearchQueryVideoResponse {
    #[serde(default)]
    pub data: ResearchQueryVideoResponseData,
    pub error: ErrorResponse,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ResearchRegionCode {
    FR,
    TH,
    MM,
    BD,
    IT,
    NP,
    IQ,
    BR,
    US,
    KW,
    VN,
    AR,
    KZ,
    GB,
    UA,
    TR,
    ID,
    PK,
    NG,
    KH,
    PH,
    EG,
    QA,
    MY,
    ES,
    JO,
    MA,
    SA,
    TW,
    AF,
    EC,
    MX,
    BW,
    JP,
    LT,
    TN,
    RO,
    LY,
    IL,
    DZ,
    CG,
    GH,
    DE,
    BJ,
    SN,
    SK,
    BY,
    NL,
    LA,
    BE,
    DO,
    TZ,
    LK,
    NI,
    LB,
    IE,
    RS,
    HU,
    PT,
    GP,
    CM,
    HN,
    FI,
    GA,
    BN,
    SG,
    BO,
    GM,
    BG,
    SD,
    TT,
    OM,
    FO,
    MZ,
    ML,
    UG,
    RE,
    PY,
    GT,
    CI,
    SR,
    AO,
    AZ,
    LR,
    CD,
    HR,
    SV,
    MV,
    GY,
    BH,
    TG,
    SL,
    MK,
    KE,
    MT,
    MG,
    MR,
    PA,
    IS,
    LU,
    HT,
    TM,
    ZM,
    CR,
    NO,
    AL,
    ET,
    GW,
    AU,
    KR,
    UY,
    JM,
    DK,
    AE,
    MD,
    SE,
    MU,
    SO,
    CO,
    AT,
    GR,
    UZ,
    CL,
    GE,
    PL,
    CA,
    CZ,
    ZA,
    AI,
    VE,
    KG,
    PE,
    CH,
    LV,
    PR,
    NZ,
    TL,
    BT,
    MN,
    FJ,
    SZ,
    VU,
    BF,
    TJ,
    BA,
    AM,
    TD,
    SI,
    CY,
    MW,
    EE,
    XK,
    ME,
    KY,
    YE,
    LS,
    ZW,
    MC,
    GN,
    BS,
    PF,
    NA,
    VI,
    BB,
    BZ,
    CW,
    PS,
    FM,
    PG,
    BI,
    AD,
    TV,
    GL,
    KM,
    AW,
    TC,
    CV,
    MO,
    VC,
    NE,
    WS,
    MP,
    DJ,
    RW,
    AG,
    GI,
    GQ,
    AS,
    AX,
    TO,
    KN,
    LC,
    NC,
    LI,
    SS,
    IR,
    SY,
    IM,
    SC,
    VG,
    SB,
    DM,
    KI,
    UM,
    SX,
    GD,
    MH,
    BQ,
    YT,
    ST,
    CF,
    BM,
    SM,
    PW,
    GU,
    HK,
    IN,
    CK,
    AQ,
    WF,
    JE,
    MQ,
    CN,
    GF,
    MS,
    GG,
    TK,
    FK,
    PM,
    NU,
    MF,
    ER,
    NF,
    VA,
    IO,
    SH,
    BL,
    CU,
    NR,
    TP,
    BV,
    EH,
    PN,
    TF,
    RU,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchVideoCommentsRequest {
    pub video_id: i64,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchVideoCommentsResponse {
    pub data: ResearchVideoCommentsData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchVideoCommentsData {
    pub comments: Vec<ResearchCommentObject>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchCommentObject {
    pub id: i64,
    pub text: String,
    pub video_id: i64,
    pub parent_comment_id: Option<i64>,
    pub like_count: i64,
    pub reply_count: i64,
    pub create_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_with_empty_data() {
        let json_data = r#"{"data":{},"error":{"code":"access_token_invalid","message":"The access token is invalid or not found in the request.","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: ResearchQueryVideoResponse = serde_json::from_str(json_data).unwrap();
        assert!(response.data.videos.is_empty());
        assert_eq!(response.data.cursor, 0);
        assert_eq!(response.data.has_more, false);
        assert_eq!(response.data.search_id, None);
        assert_eq!(response.error.code, "access_token_invalid");
    }

    #[test]
    fn test_deserialize_with_data() {
        let json_data = r#"{"data":{"videos":[],"cursor":12345,"has_more":true,"search_id":"search123"},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: ResearchQueryVideoResponse = serde_json::from_str(json_data).unwrap();
        assert!(response.data.videos.is_empty());
        assert_eq!(response.data.cursor, 12345);
        assert_eq!(response.data.has_more, true);
        assert_eq!(response.data.search_id, Some("search123".to_string()));
        assert_eq!(response.error.code, "none");
    }

    #[test]
    fn test_deserialize_with_empty_data_user() {
        let json_data = r#"{"data":{},"error":{"code":"access_token_invalid","message":"The access token is invalid or not found in the request.","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: ResearchUserInfoResponse = serde_json::from_str(json_data).unwrap();
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
    fn test_deserialize_with_data_user() {
        let json_data = r#"{"data":{"display_name":"John Doe","bio_description":"This is a bio","avatar_url":"http://example.com/avatar.jpg","is_verified":true,"follower_count":1000,"following_count":500,"likes_count":3000,"video_count":50},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: ResearchUserInfoResponse = serde_json::from_str(json_data).unwrap();
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
