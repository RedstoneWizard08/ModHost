//! Routes concerning packages.

pub mod author;
pub mod gallery;
pub mod info;
pub mod search;
pub mod ver;

use crate::state::AppState;
use axum::{
    routing::{delete, get, patch, put},
    Router,
};

/// Register package-related routes onto the router.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", put(info::create_handler))
        .route("/search", get(search::search_handler))
        .route("/:id", get(info::info_handler))
        .route("/:id", patch(info::update_handler))
        .route("/:id", delete(info::delete_handler))
        .route("/:id/authors", get(author::list_handler))
        .route("/:id/authors", put(author::add_handler))
        .route("/:id/authors", delete(author::remove_handler))
        .route("/:id/versions", get(ver::list_handler))
        .route("/:id/versions", put(ver::create_handler))
        .route("/:id/versions/latest", get(ver::latest_handler))
        .route("/:id/versions/:version", get(ver::info_handler))
        .route("/:id/versions/:version", patch(ver::update_handler))
        .route("/:id/versions/:version", delete(ver::delete_handler))
        .route(
            "/:id/versions/:version/download",
            get(ver::download_handler),
        )
        .route("/:id/gallery", get(gallery::list_handler))
        .route("/:id/gallery", put(gallery::upload_handler))
        .route("/:id/gallery/:image", get(gallery::info_handler))
        .route("/:id/gallery/:image", patch(gallery::update_handler))
        .route("/:id/gallery/:image", delete(gallery::delete_handler))
        .route("/s3/gallery/:id", get(gallery::s3_image_handler))
        .with_state(state)
}
