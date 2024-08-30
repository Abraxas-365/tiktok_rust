use derive_builder::Builder;
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

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct Condition {
    pub field_name: String,
    pub operation: String,
    pub field_values: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct Query {
    pub and: Option<Vec<Condition>>,
    pub or: Option<Vec<Condition>>,
    pub not: Option<Vec<Condition>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Builder)]
#[builder(setter(into, strip_option))]
pub struct QueryRequest {
    pub query: Query,
    pub start_date: String,
    pub end_date: String,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
    pub search_id: Option<String>,
    pub is_random: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Video {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub create_time: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub region_code: Option<RegionCode>,
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
pub struct QueryVideoResponseData {
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
    pub cursor: i64,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub search_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryVideoResponse {
    #[serde(default)]
    pub data: QueryVideoResponseData,
    pub error: ErrorResponse,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RegionCode {
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
pub struct VideoCommentsRequest {
    pub video_id: i64,
    pub max_count: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoCommentsResponse {
    pub data: ResearchVideoCommentsData,
    pub error: ErrorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResearchVideoCommentsData {
    pub comments: Vec<CommentObject>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentObject {
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
        let response: QueryVideoResponse = serde_json::from_str(json_data).unwrap();
        assert!(response.data.videos.is_empty());
        assert_eq!(response.data.cursor, 0);
        assert_eq!(response.data.has_more, false);
        assert_eq!(response.data.search_id, None);
        assert_eq!(response.error.code, "access_token_invalid");
    }

    #[test]
    fn test_deserialize_with_data() {
        let json_data = r#"{"data":{"videos":[],"cursor":12345,"has_more":true,"search_id":"search123"},"error":{"code":"none","message":"No error","log_id":"20240829190020EBE0D84CCEA6DE000CB6"}}"#;
        let response: QueryVideoResponse = serde_json::from_str(json_data).unwrap();
        assert!(response.data.videos.is_empty());
        assert_eq!(response.data.cursor, 12345);
        assert_eq!(response.data.has_more, true);
        assert_eq!(response.data.search_id, Some("search123".to_string()));
        assert_eq!(response.error.code, "none");
    }
}
