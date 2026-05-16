use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginationMeta {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub total: Option<u64>,
    #[serde(rename = "totalPages", alias = "total_pages")]
    pub total_pages: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    #[serde(default)]
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub status: Option<u16>,
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Clone)]
pub struct RawResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl RawResponse {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}
