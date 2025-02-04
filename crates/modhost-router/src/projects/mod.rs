//! Routes concerning projects.

pub mod authors;
pub mod create;
pub mod delete;
pub mod gallery;
pub mod info;
pub mod search;
pub mod update;
pub mod versions;

use axum::{
    routing::{delete, get, patch, put},
    Router,
};
use modhost_server_core::state::AppState;

/// Register project-related routes onto the router.
/// This should be nested at `/api/v1/projects`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", put(create::create_handler))
        .route("/search", get(search::search_handler))
        .route("/{id}", get(info::info_handler))
        .route("/{id}", patch(update::update_handler))
        .route("/{id}", delete(delete::delete_handler))
        .nest("/{id}/authors", authors::router(state.clone()))
        .nest("/{id}/gallery", gallery::router(state.clone()))
        .nest("/{id}/versions", versions::router(state.clone()))
        .with_state(state)
}

/// The spec for the projects API.
/// Should be nested at `/api/v1/projects`.
#[derive(OpenApi)]
#[openapi(
    paths(
        create::create_handler,
        delete::delete_handler,
        info::info_handler,
        search::search_handler,
        update::update_handler,
    ),
    nest(
        (path = "/{id}/authors", api = authors::ProjectAuthorsApi),
        (path = "/{id}/gallery", api = gallery::ProjectGalleryApi),
        (path = "/{id}/versions", api = versions::ProjectVersionsApi),
    ),
)]
pub struct ProjectsApi;
