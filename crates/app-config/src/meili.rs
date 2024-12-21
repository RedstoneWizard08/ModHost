//! The Meilisearch configuration.

/// The Meilisearch configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilisearchConfig {
    /// The host to connect to Meilisearch with.
    /// Defaults to `"localhost"`
    pub host: String,

    /// The port to connect to Meilisearch with.
    /// Defaults to `7700`
    pub port: u16,

    /// The protocol to connect to Meilisearch with.
    /// Defaults to `"http"`
    pub protocol: String,

    /// The Meilisearch key.
    /// Defaults to `"CHANGE_ME"`
    pub key: String,

    /// The index name for packages.
    /// Defaults to `"packages"`
    pub pkg_index: String,
}

impl MeilisearchConfig {
    /// Get the Meilisearch URL to connect with.
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

impl Default for MeilisearchConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 7700,
            protocol: "http".into(),
            key: "CHANGE_ME".into(),
            pkg_index: "packages".into(),
        }
    }
}
