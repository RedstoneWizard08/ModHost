//! Structs for the server's shared state.

use modhost_config::AppConfig;
use modhost_core::Result;
use axum::body::Bytes;
use base64::{prelude::BASE64_STANDARD, Engine};
use modhost_db::DbPool;
use modhost_ui::DEFAULT_FAVICON_PNG;
use oauth2::basic::BasicClient;
use s3::Bucket;
use modhost_search::MeilisearchService;
use std::{fs, sync::Arc};

use crate::models::{GameVersion, ModLoader, Tag};

/// S3 bucket state. This contains references to the buckets used by the server.
#[derive(Clone)]
pub struct BucketState {
    /// A reference to the bucket for projects.
    pub projects: Box<Bucket>,

    /// A reference to the bucket for gallery images.
    pub gallery: Box<Bucket>,
}

/// The server's shared state.
#[derive(Clone)]
pub struct AppState {
    /// The database pool.
    pub pool: DbPool,

    /// The [`BasicClient`] for GitHub OAuth2 calls.
    pub auth: BasicClient,

    /// References to buckets used by the server.
    pub buckets: BucketState,

    /// The app's configuration.
    pub config: AppConfig,

    /// A list of available mod loaders.
    /// This is set with [`modhost::ModHost::loaders`].
    pub loaders: Vec<ModLoader>,

    /// A list of available game versions.
    /// This is set with [`modhost::ModHost::versions`].
    pub game_versions: Vec<GameVersion>,

    /// A list of available tags.
    /// This is set with [`modhost::ModHost::tags`].
    pub tags: Vec<Tag>,

    /// The Meilisearch service, used for the search endpoint.
    pub search: MeilisearchService,

    /// A verifier method the server uses to verify files when uploading.
    /// This should be able to verify based on bytes alone (check the file headers).
    /// This function returns a [`bool`] indicating whether the file is valid or not.
    pub verifier: Arc<Box<dyn Fn(Bytes) -> bool + Send + Sync>>,

    /// The data URL of the icon PNG file (`data:image/png;base64,...`).
    pub icon_png_data_url: String,
}

impl AppState {
    /// Instantiate a new [`AppState`] instance.
    pub async fn new(
        pool: DbPool,
        config: &AppConfig,
        verifier: Box<dyn Fn(Bytes) -> bool + Send + Sync>,
    ) -> Result<Self> {
        let icon_data = if config.ui.favicon_png == "default" {
            DEFAULT_FAVICON_PNG.to_vec()
        } else if !config.ui.favicon_png.starts_with("http") {
            fs::read(&config.ui.favicon_png)?
        } else {
            reqwest::get(&config.ui.favicon_png)
                .await?
                .bytes()
                .await?
                .to_vec()
        };

        let icon_b64 = BASE64_STANDARD.encode(icon_data);

        Ok(Self {
            pool,
            auth: config.auth.github()?,
            buckets: BucketState {
                projects: config.storage.projects()?,
                gallery: config.storage.gallery()?,
            },
            config: config.clone(),
            loaders: vec![],
            game_versions: vec![],
            tags: vec![],
            verifier: Arc::new(verifier),
            search: MeilisearchService::new(config)?,
            icon_png_data_url: format!("data:image/png;base64,{}", icon_b64),
        })
    }
}
