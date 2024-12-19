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
    pub pageSize: i32,
    pub totalCount: i32,
    pub next: Option<String>,
}

// Message Tracking Models
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTrackingRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTrackingResult {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub received: String,
    pub status: String,
}

// TTP URL Protection Models
#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedUrlRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedUrl {
    pub url: String,
    pub action: String,
    pub comment: Option<String>,
    pub createdAt: String,
}

// Snapshot Models
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotCreateRequest {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotRestoreRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotExportRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub description: String,
    pub created: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}