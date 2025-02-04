//! The latest version route.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::AppError;
use modhost_core::Result;
use modhost_db::{ProjectVersion, ProjectVersionData, ProjectVisibility};
use modhost_db_util::{projects::get_full_project, vers::get_latest_full_version};
use modhost_server_core::state::AppState;

/// Get Latest Project Version
///
/// Get information about the latest project version
#[utoipa::path(
    get,
    path = "/versions/latest",
    tag = "Versions",
    responses(
        (status = 200, description = "Found latest version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn latest_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(project): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ProjectVersionData>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_project(project, &mut conn).await?;

    if pkg.visibility == ProjectVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    Ok(Json(get_latest_full_version(pkg.id, &mut conn).await?))
}
