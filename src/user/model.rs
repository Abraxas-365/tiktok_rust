use serde::{Deserialize, Serialize};

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
