#![warn(missing_docs)]
//! ModHost's routes & handlers.

#[macro_use]
extern crate axum;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate utoipa;

pub mod api;
pub mod auth;
pub mod meta;
pub mod moderation;
pub mod openapi;
pub mod projects;
pub mod users;

use axum::{Router, middleware::from_fn};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use jsglue::{glue::Glue, util::is_debug};
use modhost_config::AppConfig;
use modhost_middleware::logger::logging_middleware;
use modhost_server_core::state::AppState;
use utoipa::openapi::OpenApi;

/// Create the OpenAPI spec.
pub fn create_api_spec(config: &AppConfig) -> OpenApi {
    openapi::build_openapi(config)
}

/// Create the router for ModHost.
pub fn create_router(spec: &OpenApi, state: AppState, glue: Glue) -> Router {
    api::register(spec, glue.register(Router::new(), is_debug()))
        .nest("/api/v1/auth", auth::router(state.clone()))
        .nest("/api/v1/users", users::router(state.clone()))
        .nest("/api/v1/projects", projects::router(state.clone()))
        .nest("/api/v1/meta", meta::router(state.clone()))
        .nest("/api/v1/moderation", moderation::router(state.clone()))
        .layer(from_fn(logging_middleware))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .with_state(state)
}

modhost_core::utoipa_types![
    api::JsonQueryParams,
    projects::search::SearchQuery,
    projects::update::PartialProject,
    projects::versions::update::PartialProjectVersion,
    projects::gallery::create::GalleryImageUpload,
    projects::gallery::update::PartialGalleryImage,
];
