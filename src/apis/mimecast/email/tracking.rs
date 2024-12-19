use crate::apis::mimecast::{common::MimecastResponse, MimecastClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

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

impl MimecastClient {
    pub async fn track_message(&mut self, query: &str) -> Result<MimecastResponse<MessageTrackingResult>> {
        let payload = MessageTrackingRequest {
            query: query.to_string(),
            start: None,
            end: None,
        };
        self.request("/api/message-finder/search", &payload).await
    }
}