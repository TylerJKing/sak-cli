use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::apis::msgraph::auth::InteractiveAuthFlow;

const API_BASE_URL: &str = "https://graph.microsoft.com/v1.0";

pub struct GraphClient {
    client: Client,
    auth: InteractiveAuthFlow,
}

impl GraphClient {
    pub fn new(client_id: &str, client_secret: Option<&str>) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            auth: InteractiveAuthFlow::new(client_id, client_secret)?,
        })
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.raw_get(endpoint).await?;
        Ok(response.json().await?)
    }

    pub async fn post<T, R>(&self, endpoint: &str, payload: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self.raw_post(endpoint, payload).await?;
        Ok(response.json().await?)
    }

    pub async fn patch<T, R>(&self, endpoint: &str, payload: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self.raw_patch(endpoint, payload).await?;
        Ok(response.json().await?)
    }

    pub async fn delete(&self, endpoint: &str) -> Result<()> {
        self.raw_delete(endpoint).await?;
        Ok(())
    }

    async fn raw_get(&self, endpoint: &str) -> Result<Response> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let token = self.auth.get_token().await?;
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Graph API request failed: {}", error_text));
        }

        Ok(response)
    }

    async fn raw_post<T>(&self, endpoint: &str, payload: &T) -> Result<Response>
    where
        T: Serialize,
    {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let token = self.auth.get_token().await?;
        
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
            return Err(anyhow::anyhow!("Graph API request failed: {}", error_text));
        }

        Ok(response)
    }

    async fn raw_patch<T>(&self, endpoint: &str, payload: &T) -> Result<Response>
    where
        T: Serialize,
    {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let token = self.auth.get_token().await?;
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        let response = self.client
            .patch(&url)
            .headers(headers)
            .json(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Graph API request failed: {}", error_text));
        }

        Ok(response)
    }

    async fn raw_delete(&self, endpoint: &str) -> Result<Response> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let token = self.auth.get_token().await?;
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        let response = self.client
            .delete(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Graph API request failed: {}", error_text));
        }

        Ok(response)
    }

    pub fn clear_token_cache(&self) -> Result<()> {
        self.auth.clear_token_cache()
    }
}