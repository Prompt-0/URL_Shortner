use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShortenForm {
    pub url: String,
    pub custom_code: Option<String>,
    pub expires_at: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkJsonRequest {
    pub url: String,
    pub custom_code: Option<String>,
    pub expires_at: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LinkRecord {
    pub code: String,
    pub original_url: String,
    pub created_at: String,
    pub clicks: i64,
    pub expires_at: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub code: String,
    pub short_url: String,
    pub stats_url: String,
    pub original_url: String,
    pub created_at: String,
    pub clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}

impl LinkRecord {
    pub fn to_response(&self, base_url: &str) -> LinkResponse {
        let base = base_url.trim_end_matches('/');
        let short_url = format!("{base}/{}", self.code);
        let stats_url = format!("/stats/{}", self.code);

        LinkResponse {
            code: self.code.clone(),
            short_url,
            stats_url,
            original_url: self.original_url.clone(),
            created_at: self.created_at.clone(),
            clicks: self.clicks,
        }
    }
}
