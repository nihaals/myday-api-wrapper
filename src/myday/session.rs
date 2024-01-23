use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub(super) access_token: String,
    pub(super) expires_in: u64,
}

#[derive(Deserialize)]
pub(super) struct SessionResponse {
    pub(super) expires: String,
}

#[derive(Deserialize)]
pub(super) struct TokenClaims {
    pub(super) sid: String,
}
