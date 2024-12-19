use crate::apis::mimecast::{common::MimecastResponse, MimecastClient};
use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl MimecastClient {
    pub async fn get_managed_url(&mut self, url: &str) -> Result<MimecastResponse<ManagedUrl>> {
        let payload = ManagedUrlRequest {
            url: url.to_string(),
            action: None,
            comment: None,
        };
        self.request("/api/ttp/url/get-managed-url", &payload).await
    }

    pub async fn create_managed_url(
        &mut self,
        url: &str,
        action: &str,
        comment: Option<String>,
    ) -> Result<MimecastResponse<ManagedUrl>> {
        let payload = ManagedUrlRequest {
            url: url.to_string(),
            action: Some(action.to_string()),
            comment,
        };
        self.request("/api/ttp/url/create-managed-url", &payload).await
    }
}