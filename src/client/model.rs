use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenErrorResponse {
    pub error: String,
    pub error_description: String,
    pub log_id: String,
}
