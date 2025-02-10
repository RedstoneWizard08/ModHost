//! Routes concerning project galleries.

use axum::{
    Router,
    routing::{delete, get, patch, put},
};
use modhost_server_core::state::AppState;

pub mod create;
pub mod delete;
pub mod download;
pub mod info;
pub mod list;
pub mod update;

/// Register project gallery API routes.
/// Should be nested at `/api/v1/projects/{id}/gallery`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list::list_handler))
        .route("/", put(create::create_handler))
        .route("/{image}", get(info::info_handler))
        .route("/{image}", patch(update::update_handler))
        .route("/{image}", delete(delete::delete_handler))
        .route("/{image}/download", get(download::download_handler))
        .with_state(state)
}

/// The spec for the project gallery API.
/// Should be nested at `/api/v1/projects/{id}/gallery`.
#[derive(OpenApi)]
#[openapi(paths(
    create::create_handler,
    delete::delete_handler,
    download::download_handler,
    info::info_handler,
    list::list_handler,
    update::update_handler,
))]
pub struct ProjectGalleryApi;
