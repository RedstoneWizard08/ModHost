//! User-related routes.

pub mod info;
pub mod me;
pub mod pkg;

use axum::{routing::get, Router};
use modhost_server_core::state::AppState;

/// Register user-related endpoints.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/:id", get(info::info_handler))
        .route("/:id/projects", get(pkg::list_handler))
        .with_state(state)
}
