use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TikTokScope {
    #[serde(rename = "artist.certification.read")]
    ArtistCertificationRead,
    #[serde(rename = "artist.certification.update")]
    ArtistCertificationUpdate,
    #[serde(rename = "user.info.profile")]
    UserInfoProfile,
    #[serde(rename = "user.info.stats")]
    UserInfoStats,
    #[serde(rename = "video.list")]
    VideoList,
}

impl TikTokScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            TikTokScope::ArtistCertificationRead => "artist.certification.read",
            TikTokScope::ArtistCertificationUpdate => "artist.certification.update",
            TikTokScope::UserInfoProfile => "user.info.profile",
            TikTokScope::UserInfoStats => "user.info.stats",
            TikTokScope::VideoList => "video.list",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_key: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: HashSet<TikTokScope>,
    pub csrf_state: String,
    pub code_verifier: String,
    pub code_challenge: String,
}

impl OAuthConfig {
    pub fn new(
        client_key: &str,
        client_secret: &str,
        redirect_uri: &str,
        scopes: &[TikTokScope],
    ) -> Self {
        let csrf_state = generate_csrf_state();
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);

        Self {
            client_key: client_key.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: scopes.iter().cloned().collect(),
            csrf_state,
            code_verifier,
            code_challenge,
        }
    }

    pub fn authorization_url(&self) -> String {
        let mut url = Url::parse("https://www.tiktok.com/v2/auth/authorize/").unwrap();
        url.query_pairs_mut()
            .append_pair("client_key", &self.client_key)
            .append_pair("scope", &self.scopes_string())
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("state", &self.csrf_state)
            .append_pair("code_challenge", &self.code_challenge)
            .append_pair("code_challenge_method", "S256");

        url.to_string()
    }

    fn scopes_string(&self) -> String {
        self.scopes
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthCallback {
    pub code: Option<String>,
    pub scopes: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

fn generate_csrf_state() -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

fn generate_code_verifier() -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

fn generate_code_challenge(code_verifier: &str) -> String {
    use base64::{engine::general_purpose, Engine as _};
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(code_verifier);
    let result = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub open_id: String,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}
