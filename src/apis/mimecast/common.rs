use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MimecastResponse<T> {
    pub meta: ResponseMeta,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMeta {
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    pub next: Option<String>,
}