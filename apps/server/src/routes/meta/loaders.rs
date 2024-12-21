//! The meta endpoint for getting a list of mod loaders.

use crate::{state::AppState, Result};
use axum::{extract::State, Json};

/// A mod loader.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct ModLoader {
    /// The ID of the loader.
    pub id: String,

    /// The display name of the loader.
    pub name: String,
}

/// Get Mod Loaders
///
/// Get a list of mod loaders.
#[utoipa::path(
    get,
    path = "/api/v1/meta/loaders",
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
