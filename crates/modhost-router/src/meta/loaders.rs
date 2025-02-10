//! The meta endpoint for getting a list of mod loaders.

use axum::{Json, extract::State};
use modhost_core::Result;
use modhost_server_core::{models::ModLoader, state::AppState};

/// Get Mod Loaders
///
/// Get a list of mod loaders.
#[utoipa::path(
    get,
    path = "/loaders",
    tag = "Meta",
    responses(
        (status = 200, description = "Got mod loaders!", body = Vec<ModLoader>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn loaders_handler(State(state): State<AppState>) -> Result<Json<Vec<ModLoader>>> {
    Ok(Json(state.loaders))
}
