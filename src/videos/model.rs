use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Video {
    pub id: i64,
    pub create_time: i64,
    pub username: Option<String>,
    pub region_code: Option<RegionCode>,
    pub video_description: Option<String>,
    pub music_id: Option<i64>,
    pub like_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub share_count: Option<i64>,
    pub view_count: Option<i64>,
    pub effect_ids: Option<Vec<String>>,
    pub hashtag_names: Option<Vec<String>>,
    pub playlist_id: Option<i64>,
    pub voice_to_text: Option<String>,
    pub is_stem_verified: Option<bool>,
    pub video_duration: Option<i64>,
    pub favourites_count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryVideoResponseData {
    pub videos: Vec<Video>,
    pub cursor: i64,
    pub has_more: bool,
    pub search_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryVideoResponse {
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