#![warn(missing_docs)]
//! ModHost's database module, containing models and utilities.

#[macro_use]
extern crate utoipa;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod util;

pub use models::*;
pub use schema::*;
pub use util::*;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
};
use diesel_async_migrations::{EmbeddedMigrations, embed_migrations};
use modhost_core::{Result, utoipa_types};
use std::env;

/// The embedded SQL database migrations.
pub static MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// The async database pool type.
pub type DbPool = Pool<AsyncPgConnection>;

/// The async database connection type.
pub type DbConn = Object<AsyncPgConnection>;

/// Create an async connection to a database.
pub async fn create_connection(db_url: Option<String>) -> Result<DbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(Ok).unwrap_or_else(|| {
        embedded_db_url
            .map(Ok)
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(Pool::builder(AsyncDieselConnectionManager::new(db_url)).build()?)
}

/// Run the migrations on an async database connection via its pool.
pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    MIGRATIONS
        .run_pending_migrations(&mut pool.get().await?)
        .await?;

    Ok(())
}

utoipa_types![
    User,
    UserToken,
    NewUser,
    NewUserToken,
    ProjectManifest,
    Project,
    ProjectAuthor,
    ProjectRelation,
    ProjectVersion,
    ProjectVersionRef,
    ProjectVersionInit,
    NewProject,
    NewProjectVersion,
    RelationKind,
    ProjectData,
    ProjectVisibility,
    GalleryImage,
    NewGalleryImage,
    PublicGalleryImage,
    ProjectFile,
    NewProjectFile,
    ProjectVersionData,
    ModerationComment,
    ModerationQueueItem,
    ModerationQueueStatus,
];
