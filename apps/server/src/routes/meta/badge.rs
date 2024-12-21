//! Meta endpoints for badges.

use crate::{state::AppState, util::sanitize::HtmlSanitize, Result};
use axum::{
    extract::{Path, State},
    response::Response,
};

// TODO: https://github.com/cgbur/badge-maker?

/// Version Badge
///
/// Get a badge for a specific version of a package.
#[utoipa::path(
    get,
    path = "/api/v1/meta/badge/version/{version}",
    tag = "Meta",
    params(
        ("version" = String, description = "The version."),
    ),
    responses(
        (status = 200, description = "Created a badge!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn version_handler(
    State(state): State<AppState>,
    Path(version): Path<String>,
) -> Result<Response> {
    let data = format!(
        include_str!("./version_badge.svg"),
        version = version,
        site = state.config.ui.app
    )
    .html_sanitize();

    Ok(Response::builder()
        .header("Content-Type", "image/svg+xml")
        .body(data.into())?)
}
