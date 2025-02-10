//! The version list route.

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::AppError;
use modhost_core::Result;
use modhost_db::{ProjectVersionData, ProjectVisibility};
use modhost_db_util::{projects::get_full_project, vers::get_versions};
use modhost_server_core::state::AppState;

/// List Project Versions
///
/// List available versions for a specific project.
#[utoipa::path(
    get,
    path = "/",
    tag = "Versions",
    responses(
        (status = 200, description = "Found project versions!", body = Vec<ProjectVersionData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectVersionData>>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_project(id, &mut conn).await?;

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

    Ok(Json(get_versions(pkg.id, &mut conn).await?))
}
