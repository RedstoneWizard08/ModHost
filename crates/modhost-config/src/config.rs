//! The main config models.

use crate::{AuthConfigs, MeilisearchConfig, PostgresConfig, StorageConfig, UIConfig};
use modhost_core::Result;
use std::fs;

/// The main ModHost configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// The ModHost server configuration.
    pub server: ServerConfig,

    /// The database ([PostgreSQL](https://postgresql.org/)) configuration.
    pub postgres: PostgresConfig,

    /// The authentication configuration.
    pub auth: AuthConfigs,

    /// The S3 storage configuration.
    pub storage: StorageConfig,

    /// UI configuration.
    pub ui: UIConfig,

    /// The Meilisearch configuration.
    pub meilisearch: MeilisearchConfig,
}

/// The server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// The host to listen on.
    /// Defaults to `"127.0.0.1"`
    pub host: String,

    /// The port to listen on.
    /// Defaults to `4000`
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 4000,
        }
    }
}

impl AppConfig {
    /// Save the configuration to a file (`ModHost.toml` in the current working directory).
    pub fn save(&self) -> Result<()> {
        fs::write("ModHost.toml", toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
