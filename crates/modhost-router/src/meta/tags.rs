//! The meta endpoint for getting a list of tags.

use axum::{extract::State, Json};
use modhost_core::Result;
use modhost_server_core::{models::Tag, state::AppState};

/// Get Tags
///
/// Get a list of available tags.
#[utoipa::path(
    get,
    path = "/tags",
    tag = "Meta",
    responses(
        (status = 200, description = "Got tags!", body = Vec<Tag>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn tags_handler(State(state): State<AppState>) -> Result<Json<Vec<Tag>>> {
    Ok(Json(state.tags))
}
