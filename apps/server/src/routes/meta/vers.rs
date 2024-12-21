//! The meta endpoint for getting a list of game versions.

use crate::{state::AppState, Result};
use axum::{extract::State, Json};

/// A game version.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct GameVersion {
    /// The version's ID (or version number).
    pub id: String,

    /// Whether this version is a beta version.
    pub beta: bool,
}

/// Get Game Versions
///
/// Get a list of game versions.
#[utoipa::path(
    get,
    path = "/api/v1/meta/game_versions",
    tag = "Meta",
    responses(
        (status = 200, description = "Got game versions!", body = Vec<GameVersion>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn game_versions_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<GameVersion>>> {
    Ok(Json(state.game_versions))
}
