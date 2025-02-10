//! Routes concerning project authors.

use axum::{
    Router,
    routing::{delete, get, put},
};
use modhost_server_core::state::AppState;

pub mod add;
pub mod list;
pub mod remove;

/// Register project authors API routes.
/// Should be nested at `/api/v1/projects/{id}/authors`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list::list_handler))
        .route("/", put(add::add_handler))
        .route("/", delete(remove::remove_handler))
        .with_state(state)
}

/// The spec for the project authors API.
/// Should be nested at `/api/v1/projects/{id}/authors`.
#[derive(OpenApi)]
#[openapi(paths(add::add_handler, list::list_handler, remove::remove_handler,))]
pub struct ProjectAuthorsApi;
