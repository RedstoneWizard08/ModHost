//! The list authors route.

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
use modhost_db::{ProjectVisibility, User};
use modhost_db_util::projects::get_full_project;
use modhost_server_core::state::AppState;

/// Get Project Authors
///
/// Get a project's authors by its ID or slug.
#[utoipa::path(
    get,
    path = "/",
    tag = "Projects",
    responses(
        (status = 200, description = "A list of project authors", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
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
        .body(Body::new(serde_json::to_string(&pkg.authors)?))?)
}
