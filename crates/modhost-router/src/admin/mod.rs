//! Admin routes.

use axum::{
    Router,
    routing::{delete, get, put},
};
use modhost_server_core::state::AppState;

pub mod add;
pub mod list;
pub mod projects;
pub mod remove;
pub mod stats;
pub mod stats_ws;
pub mod users;

/// Register admin-related routes onto the router.
/// This should be nested at `/api/v1/admin`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/stats", get(stats::stats_handler))
        .route("/add/{id}", put(add::add_handler))
        .route("/remove/{id}", delete(remove::remove_handler))
        .route("/list", get(list::list_handler))
        .route("/users/list", get(users::list::list_handler))
        .route("/users/{id}", get(users::get::get_handler))
        .route("/users/{id}", delete(users::delete::delete_handler))
        .route("/stats/ws", get(stats_ws::stats_socket_handler))
        .with_state(state)
}

/// The spec for the admin API.
/// Should be nested at `/api/v1/admin`.
#[derive(OpenApi)]
#[openapi(paths(
    stats::stats_handler,
    add::add_handler,
    remove::remove_handler,
    list::list_handler,
    users::list::list_handler,
    users::get::get_handler,
    users::delete::delete_handler,
    stats_ws::stats_socket_handler,
))]
pub struct AdminApi;
