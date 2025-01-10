//! The main Meilisearch service.

use app_config::AppConfig;
use app_core::Result;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::Index;

/// The main Meilisearch service.
/// 99% of this crate's usage is through this struct.
#[derive(Debug, Clone)]
pub struct MeilisearchService {
    /// The underlying Meilisearch client.
    pub(crate) client: Client,

    /// The ID of the projects index.
    pub(crate) projects: String,
}

impl MeilisearchService {
    /// Create a new [`MeilisearchService`] from an [`AppConfig`].
    pub fn new(cfg: &AppConfig) -> Result<Self> {
        Ok(Self {
            client: Client::new(cfg.meilisearch.url(), Some(&cfg.meilisearch.key))?,
            projects: cfg.meilisearch.project_index.clone(),
        })
    }

    /// Get the projects [`Index`].
    pub fn projects(&self) -> Index {
        self.client.index(&self.projects)
    }
}
