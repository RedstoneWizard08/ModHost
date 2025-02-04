//! Routes concerning project versions.

use axum::{
    routing::{delete, get, patch, put},
    Router,
};
use modhost_server_core::state::AppState;

pub mod create;
pub mod delete;
pub mod download;
pub mod info;
pub mod latest;
pub mod list;
pub mod update;

/// Register project versions API routes.
/// Should be nested at `/api/v1/projects/{id}/versions`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list::list_handler))
        .route("/", put(create::create_handler))
        .route("/latest", get(latest::latest_handler))
        .route("/{version}", get(info::info_handler))
        .route("/{version}", patch(update::update_handler))
        .route("/{version}", delete(delete::delete_handler))
        .route(
            "/{version}/download/{file}",
            get(download::download_handler),
        )
        .with_state(state)
}

/// The spec for the project versions API.
/// Should be nested at `/api/v1/projects/{id}/versions`.
#[derive(OpenApi)]
#[openapi(paths(
    create::create_handler,
    delete::delete_handler,
    download::download_handler,
    info::info_handler,
    list::list_handler,
    update::update_handler,
    latest::latest_handler,
))]
pub struct ProjectVersionsApi;
