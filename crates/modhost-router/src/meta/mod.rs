//! Meta-related endpoints.

pub mod badge;
pub mod loaders;
pub mod tags;
pub mod vers;

use axum::{Router, routing::get};
use modhost_server_core::state::AppState;

/// Register metadata-related endpoints.
/// Should be nested at `/api/v1/meta`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/badge", get(modhost_badges::routes::badge_route))
        .route("/badge/version/{version}", get(badge::version_handler))
        .route(
            "/badge/latest/{project}",
            get(badge::latest_version_badge_handler),
        )
        .route("/loaders", get(loaders::loaders_handler))
        .route("/game_versions", get(vers::game_versions_handler))
        .route("/tags", get(tags::tags_handler))
        .with_state(state)
}

/// The spec for the metadata API.
/// Should be nested at `/api/v1/meta`.
#[derive(OpenApi)]
#[openapi(paths(
    modhost_badges::routes::badge_route,
    badge::version_handler,
    badge::latest_version_badge_handler,
    loaders::loaders_handler,
    vers::game_versions_handler,
    tags::tags_handler,
))]
pub struct MetadataApi;
