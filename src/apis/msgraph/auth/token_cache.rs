use anyhow::Result;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: SystemTime,
}

pub struct TokenCache {
    keyring: Entry,
}

impl TokenCache {
    pub fn new() -> Result<Self> {
        let keyring = Entry::new("sak-cli", "msgraph")?;
        Ok(Self { keyring })
    }

    pub fn get_token(&self) -> Result<Option<CachedToken>> {
        match self.keyring.get_password() {
            Ok(data) => {
                let token: CachedToken = serde_json::from_str(&data)?;
                // Return None if token is expired or will expire in next 5 minutes
                if token.expires_at <= SystemTime::now() + Duration::from_secs(300) {
                    Ok(None)
                } else {
                    Ok(Some(token))
                }
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save_token(&self, token: &CachedToken) -> Result<()> {
        let data = serde_json::to_string(token)?;
        self.keyring.set_password(&data)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        match self.keyring.delete_password() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}