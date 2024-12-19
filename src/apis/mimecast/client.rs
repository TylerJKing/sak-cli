use crate::config::MimecastConfig;
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

pub struct MimecastClient {
    config: MimecastConfig,
    client: Client,
    token: Option<(String, SystemTime)>,
}

impl MimecastClient {
    pub fn new(config: MimecastConfig) -> Result<Self> {
        let client = Client::builder()
            .build()?;
        Ok(Self {
            config,
            client,
            token: None,
        })
    }

    async fn ensure_token(&mut self) -> Result<String> {
        // Check if we have a valid token
        if let Some((token, expiry)) = &self.token {
            if SystemTime::now()
                .duration_since(*expiry)
                .unwrap_or(Duration::from_secs(0))
                < Duration::from_secs(300)
            {
                return Ok(token.clone());
            }
        }

        // Get new token
        let auth = format!("{}:{}", self.config.app_id, self.config.app_key);
        let auth_header = format!("Basic {}", BASE64.encode(auth.as_bytes()));

        let mut form = std::collections::HashMap::new();
        form.insert("grant_type", "client_credentials");

        let response = self.client
            .post(format!("{}/oauth/token", self.config.base_url))
            .header(AUTHORIZATION, auth_header)
            .form(&form)
            .send()
            .await
            .context("Failed to get access token")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to get access token: {}", error_text));
        }

        let token_response: TokenResponse = response.json().await?;
        let expiry = SystemTime::now() + Duration::from_secs(token_response.expires_in);
        
        self.token = Some((token_response.access_token.clone(), expiry));
        Ok(token_response.access_token)
    }

    pub async fn request<T, R>(&mut self, endpoint: &str, payload: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let token = self.ensure_token().await?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API request failed: {}", error_text));
        }

        Ok(response.json().await?)
    }
}