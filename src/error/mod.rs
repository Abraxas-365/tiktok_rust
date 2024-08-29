use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: String,
    message: String,
    log_id: String,
}

#[derive(Error, Debug)]
pub enum TikTokApiError {
    #[error("Access token is invalid or not found in the request. Please refresh the token and retry. Log ID: {0}")]
    AccessTokenInvalid(String),

    #[error("This is the generic error code for TikTok internal errors. Please refer to the error message for details and notify TikTok support. Log ID: {0}")]
    InternalError(String),

    #[error("The uploaded file does not meet API specifications. Please correct the file and try again. Log ID: {0}")]
    InvalidFileUpload(String),

    #[error("One or more fields in request is invalid. Please refer to the error message for details. Log ID: {0}")]
    InvalidParams(String),

    #[error("The API rate limit was exceeded. Please try again later. Log ID: {0}")]
    RateLimitExceeded(String),

    #[error("The user did not authorize the scope required for completing this request. Please ask the user to authorize and then retry. Log ID: {0}")]
    ScopeNotAuthorized(String),

    #[error("Access token is invalid, some fields need additional scopes. Please refer to the error message for more details. Log ID: {0}")]
    ScopePermissionMissed(String),

    #[error("Unknown error occurred. Code: {0}, Message: {1}, Log ID: {2}")]
    Unknown(String, String, String),
}

// Function to map error response to custom error
impl From<ErrorResponse> for TikTokApiError {
    fn from(error: ErrorResponse) -> Self {
        match error.code.as_str() {
            "access_token_invalid" => TikTokApiError::AccessTokenInvalid(error.log_id),
            "internal_error" => TikTokApiError::InternalError(error.log_id),
            "invalid_file_upload" => TikTokApiError::InvalidFileUpload(error.log_id),
            "invalid_params" => TikTokApiError::InvalidParams(error.log_id),
            "rate_limit_exceeded" => TikTokApiError::RateLimitExceeded(error.log_id),
            "scope_not_authorized" => TikTokApiError::ScopeNotAuthorized(error.log_id),
            "scope_permission_missed" => TikTokApiError::ScopePermissionMissed(error.log_id),
            _ => TikTokApiError::Unknown(error.code, error.message, error.log_id),
        }
    }
}
