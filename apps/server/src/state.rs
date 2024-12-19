//! Structs for the server's shared state.

use crate::{
    routes::meta::{loaders::ModLoader, tags::Tag, vers::GameVersion},
    Result,
};
use app_config::AppConfig;
use axum::body::Bytes;
use db::DbPool;
use oauth2::basic::BasicClient;
use s3::Bucket;
use search::MeilisearchService;
use std::sync::Arc;

/// S3 bucket state. This contains references to the buckets used by the server.
#[derive(Clone)]
pub struct BucketState {
    /// A reference to the bucket for packages.
    pub packages: Box<Bucket>,

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
    /// This is set with [`crate::ModHost::loaders`].
    pub loaders: Vec<ModLoader>,

    /// A list of available game versions.
    /// This is set with [`crate::ModHost::versions`].
    pub game_versions: Vec<GameVersion>,

    /// A list of available tags.
    /// This is set with [`crate::ModHost::tags`].
    pub tags: Vec<Tag>,

    /// The Meilisearch service, used for the search endpoint.
    pub search: MeilisearchService,

    /// A verifier method the server uses to verify files when uploading.
    /// This should be able to verify based on bytes alone (check the file headers).
    /// This function returns a [`bool`] indicating whether the file is valid or not.
    pub verifier: Arc<Box<dyn Fn(Bytes) -> bool + Send + Sync>>,
}

impl AppState {
    /// Instantiate a new [`AppState`] instance.
    pub fn new(
        pool: DbPool,
        config: &AppConfig,
        verifier: Box<dyn Fn(Bytes) -> bool + Send + Sync>,
    ) -> Result<Self> {
        Ok(Self {
            pool,
            auth: config.auth.github()?,
            buckets: BucketState {
                packages: config.storage.packages()?,
                gallery: config.storage.gallery()?,
            },
            config: config.clone(),
            loaders: vec![],
            game_versions: vec![],
            tags: vec![],
            verifier: Arc::new(verifier),
            search: MeilisearchService::new(config)?,
        })
    }
}
