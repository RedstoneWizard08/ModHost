//! The project info route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::AppError;
use modhost_core::Result;
use modhost_db::{ProjectData, ProjectVisibility};
use modhost_db_util::projects::get_full_project;
use modhost_server_core::state::AppState;

/// Get Project
///
/// Get a project by its ID or slug.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "Projects",
    responses(
        (status = 200, description = "Information about the project", body = ProjectData),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project ID or slug"),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
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

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&pkg)?))?)
}
