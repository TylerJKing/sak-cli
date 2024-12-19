use crate::apis::mimecast::{common::MimecastResponse, MimecastClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

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

impl MimecastClient {
    pub async fn create_snapshot(&mut self, description: &str) -> Result<MimecastResponse<Snapshot>> {
        let payload = SnapshotCreateRequest {
            description: description.to_string(),
        };
        self.request("/api/config-snapshot/create", &payload).await
    }

    pub async fn list_snapshots(&mut self, start: Option<String>, limit: Option<i32>) -> Result<MimecastResponse<Snapshot>> {
        let payload = SnapshotListRequest { start, limit };
        self.request("/api/config-snapshot/list", &payload).await
    }

    pub async fn restore_snapshot(&mut self, id: &str) -> Result<MimecastResponse<Snapshot>> {
        let payload = SnapshotRestoreRequest {
            id: id.to_string(),
        };
        self.request("/api/config-snapshot/restore", &payload).await
    }

    pub async fn export_snapshot(&mut self, id: &str) -> Result<MimecastResponse<Snapshot>> {
        let payload = SnapshotExportRequest {
            id: id.to_string(),
        };
        self.request("/api/config-snapshot/export", &payload).await
    }
}