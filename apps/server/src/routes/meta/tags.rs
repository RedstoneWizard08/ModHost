//! The meta endpoint for getting a list of tags.

use crate::{state::AppState, Result};
use axum::{extract::State, Json};

/// A tag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct Tag {
    /// The ID of the tag.
    pub id: String,

    /// The tag's display name.
    pub name: String,

    /// The tag's Iconify icon name.
    pub icon: String,
}

/// Get Tags
///
/// Get a list of available tags.
#[utoipa::path(
    get,
    path = "/api/v1/meta/tags",
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
