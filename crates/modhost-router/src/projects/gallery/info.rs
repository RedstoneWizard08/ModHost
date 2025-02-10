//! The gallery image info route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{ProjectVisibility, PublicGalleryImage, get_gallery_image};
use modhost_db_util::{gallery::transform_gallery_image, projects::get_full_project};
use modhost_server_core::state::AppState;

/// Get Gallery Image
///
/// Get information about a specific gallery image
#[utoipa::path(
    get,
    path = "/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Found gallery image!", body = PublicGalleryImage),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("image" = String, Path, description = "The image ID."),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((id, image)): Path<(String, String)>,
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

    let img = get_gallery_image(image, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&transform_gallery_image(
            img,
        ))?))?)
}
