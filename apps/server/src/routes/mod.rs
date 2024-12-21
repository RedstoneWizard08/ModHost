//! ModHost's routes & handlers.

pub mod api;
pub mod auth;
pub mod meta;
pub mod pkg;
pub mod users;

use crate::{middleware::logger::logging_middleware, state::AppState};
use axum::{middleware::from_fn, Router};
use jsglue::{glue::Glue, util::is_debug};

/// Create the router for ModHost.
pub fn create_router(state: AppState, glue: Glue) -> Router {
    api::register(glue.register(Router::new(), is_debug()))
        .nest("/api/v1/auth", auth::router(state.clone()))
        .nest("/api/v1/users", users::router(state.clone()))
        .nest("/api/v1/packages", pkg::router(state.clone()))
        .nest("/api/v1/meta", meta::router(state.clone()))
        .layer(from_fn(logging_middleware))
        .with_state(state)
}
