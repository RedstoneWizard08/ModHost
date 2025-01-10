//! Meta-related endpoints.

pub mod badge;
pub mod loaders;
pub mod tags;
pub mod vers;

use crate::state::AppState;
use axum::{routing::get, Router};

/// Register meta-related endpoints.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/badge/version/:version", get(badge::version_handler))
        .route(
            "/badge/latest/:project",
            get(badge::latest_version_badge_handler),
        )
        .route("/loaders", get(loaders::loaders_handler))
        .route("/game_versions", get(vers::game_versions_handler))
        .route("/tags", get(tags::tags_handler))
        .with_state(state)
}
