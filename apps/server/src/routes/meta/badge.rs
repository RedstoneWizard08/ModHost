//! Meta endpoints for badges.

use crate::{state::AppState, util::versions::get_latest_version, Result};
use axum::{
    extract::{Path, State},
    response::Response,
};
use db::get_package;

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
        include_str!("../../assets/badges/version.svg"),
        version = version,
        site = state.config.ui.app,
        icon = state.icon_png_data_url,
        badge_base = state.config.ui.badge_base,
        badge_secondary = state.config.ui.badge_secondary,
    );

    Ok(Response::builder()
        .header("Content-Type", "image/svg+xml")
        .body(data.into())?)
}

/// Latest Version Badge
///
/// Get a badge for the latest version of a package.
#[utoipa::path(
    get,
    path = "/api/v1/meta/badge/latest/{package}",
    tag = "Meta",
    params(
        ("package" = String, description = "The package."),
    ),
    responses(
        (status = 200, description = "Created a badge!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn latest_version_handler(
    State(state): State<AppState>,
    Path(package): Path<String>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_latest_version(pkg.id, &mut conn).await?;

    let data = format!(
        include_str!("../../assets/badges/version.svg"),
        version = ver.version_number,
        site = state.config.ui.app,
        icon = state.icon_png_data_url,
        badge_base = state.config.ui.badge_base,
        badge_secondary = state.config.ui.badge_secondary,
    );

    Ok(Response::builder()
        .header("Content-Type", "image/svg+xml")
        .body(data.into())?)
}
