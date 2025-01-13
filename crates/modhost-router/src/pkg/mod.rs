//! Routes concerning projects.

pub mod author;
pub mod gallery;
pub mod info;
pub mod search;
pub mod ver;

use axum::{
    routing::{delete, get, patch, put},
    Router,
};
use modhost_server_core::state::AppState;

/// Register project-related routes onto the router.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", put(info::create_project_handler))
        .route("/search", get(search::search_projects_handler))
        .route("/{id}", get(info::project_info_handler))
        .route("/{id}", patch(info::update_project_handler))
        .route("/{id}", delete(info::delete_project_handler))
        .route("/{id}/authors", get(author::list_authors_handler))
        .route("/{id}/authors", put(author::add_author_handler))
        .route("/{id}/authors", delete(author::remove_author_handler))
        .route("/{id}/versions", get(ver::list_versions_handler))
        .route("/{id}/versions", put(ver::create_version_handler))
        .route("/{id}/versions/latest", get(ver::latest_version_handler))
        .route("/{id}/versions/{version}", get(ver::version_info_handler))
        .route(
            "/{id}/versions/{version}",
            patch(ver::update_version_handler),
        )
        .route(
            "/{id}/versions/{version}",
            delete(ver::delete_version_handler),
        )
        .route(
            "/{id}/versions/{version}/download/{file}",
            get(ver::download_version_handler),
        )
        .route("/{id}/gallery", get(gallery::list_gallery_handler))
        .route("/{id}/gallery", put(gallery::upload_gallery_handler))
        .route("/{id}/gallery/{image}", get(gallery::gallery_info_handler))
        .route(
            "/{id}/gallery/{image}",
            patch(gallery::update_gallery_handler),
        )
        .route(
            "/{id}/gallery/{image}",
            delete(gallery::delete_gallery_handler),
        )
        .route("/s3/gallery/{id}", get(gallery::s3_image_handler))
        .with_state(state)
}
