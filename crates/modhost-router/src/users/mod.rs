//! User-related routes.

pub mod info;
pub mod me;
pub mod pkg;

use axum::{Router, routing::get};
use modhost_server_core::state::AppState;

/// Register user-related endpoints.
/// Should be nested at `/api/v1/users`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/{id}", get(info::info_handler))
        .route("/{id}/projects", get(pkg::list_handler))
        .with_state(state)
}

/// The spec for the users API.
/// Should be nested at `/api/v1/users`.
#[derive(OpenApi)]
#[openapi(paths(me::me_handler, info::info_handler, pkg::list_handler,))]
pub struct UsersApi;
