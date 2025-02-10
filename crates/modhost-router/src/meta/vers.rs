//! The meta endpoint for getting a list of game versions.

use axum::{Json, extract::State};
use modhost_core::Result;
use modhost_server_core::{models::GameVersion, state::AppState};

/// Get Game Versions
///
/// Get a list of game versions.
#[utoipa::path(
    get,
    path = "/game_versions",
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
