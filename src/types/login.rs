use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginError {
    pub status: bool,
    pub message: String
}