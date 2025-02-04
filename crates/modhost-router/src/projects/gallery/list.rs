//! The gallery image list route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{ProjectVisibility, PublicGalleryImage};
use modhost_db_util::{
    gallery::transform_gallery,
    projects::{get_full_project, get_gallery},
};
use modhost_server_core::state::AppState;

/// Get Gallery Images
///
/// Get gallery images for a project.
#[utoipa::path(
    get,
    path = "/",
    tag = "Gallery",
    responses(
        (status = 200, description = "The project's gallery images.", body = Vec<PublicGalleryImage>),
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
    let pkg = get_full_project(id.clone(), &mut conn).await?;

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
        .body(Body::new(serde_json::to_string(&transform_gallery(
            get_gallery(id, &mut conn).await?,
        ))?))?)
}
