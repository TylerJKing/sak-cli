use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimecastConfig {
    pub base_url: String,
    pub app_id: String,
    pub app_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mimecast: Option<MimecastConfig>,
    pub msgraph: Option<GraphConfig>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            std::fs::create_dir_all(config_path.parent().unwrap())?;
            std::fs::write(&config_path, toml::to_string(&Config { mimecast: None, msgraph: None })?)?;
        }
        let config_str = std::fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config_str)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        std::fs::write(config_path, toml::to_string(self)?)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("dev", "all-hands", "sak-cli")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}