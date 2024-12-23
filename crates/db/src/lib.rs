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

use app_core::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool as SyncPool, PooledConnection},
    PgConnection,
};
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use std::env;

/// The embedded SQL database migrations.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// The async database pool type.
pub type DbPool = Pool<AsyncPgConnection>;

/// The async database connection type.
pub type DbConn = Object<AsyncPgConnection>;

/// The synchronous database pool type.
#[deprecated]
pub type SyncDbPool = SyncPool<ConnectionManager<PgConnection>>;

/// The synchronous database connection type.
#[deprecated]
pub type SyncDbConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Create an async connection to a database.
pub async fn create_connection(db_url: Option<String>) -> Result<DbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(|v| Ok(v)).unwrap_or_else(|| {
        embedded_db_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(Pool::builder(AsyncDieselConnectionManager::new(db_url)).build()?)
}

/// Create a synchronous connection to a database.
#[deprecated]
#[allow(deprecated)]
pub fn create_sync_connection(db_url: Option<String>) -> Result<SyncDbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(|v| Ok(v)).unwrap_or_else(|| {
        embedded_db_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(SyncPool::builder()
        .test_on_check_out(true)
        .build(ConnectionManager::<PgConnection>::new(db_url))?)
}

/// Run the migrations on an async database connection via its pool.
pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    MIGRATIONS
        .run_pending_migrations(&mut pool.get().await?)
        .await?;

    Ok(())
}
