//! Admin routes.

use axum::{Router, routing::get};
use modhost_server_core::state::AppState;

pub mod stats;

/// Register admin-related routes onto the router.
/// This should be nested at `/api/v1/admin`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/stats", get(stats::stats_handler))
        .with_state(state)
}

/// The spec for the admin API.
/// Should be nested at `/api/v1/admin`.
#[derive(OpenApi)]
#[openapi(paths(stats::stats_handler))]
pub struct AdminApi;
