//! ModHost's worker, providing token invalidation.

use chrono::Utc;
use diesel::{QueryDsl, SelectableHelper, delete};
use diesel_async::RunQueryDsl;
use jsglue::abort::ABORT_HANDLES;
use modhost_core::Result;
use modhost_db::{DbPool, UserToken, user_tokens};
use tokio::task::JoinHandle;

/// Start the worker service and get a handle to its thread.
pub fn run_worker(pool: DbPool) -> JoinHandle<Result<()>> {
    info!("Starting worker...");

    let handle = tokio::spawn(async move { worker_loop(pool).await });
    let abort = handle.abort_handle();

    // Hook into Glue's exit handler.
    ABORT_HANDLES.lock().unwrap().push(abort);

    handle
}

/// The internal worker loop.
/// This function will never return unless an error occurs.
async fn worker_loop(pool: DbPool) -> Result<()> {
    let mut conn = pool.get().await?;

    loop {
        let tkns = user_tokens::table
            .select(UserToken::as_select())
            .load(&mut conn)
            .await?;

        for token in tkns {
            let time = Utc::now().timestamp_millis();

            if time >= token.expires.and_utc().timestamp_millis() {
                info!("Found expired token (id: {}). Deleting...", token.id);

                delete(&token).execute(&mut conn).await?;
            }
        }
    }
}
