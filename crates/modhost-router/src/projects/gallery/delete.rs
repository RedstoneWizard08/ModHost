//! The delete gallery image route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, delete};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{GalleryImage, ProjectAuthor, gallery_images, get_gallery_image, project_authors};
use modhost_db_util::projects::get_project;
use modhost_server_core::state::AppState;

/// Delete Gallery Image
///
/// Delete a gallery image
#[utoipa::path(
    delete,
    path = "/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Deleted gallery image!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("image" = String, Path, description = "The gallery image ID number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, image)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_project(project, &mut conn).await?;
    let img = get_gallery_image(image, &mut conn).await?;

    let authors = project_authors::table
        .filter(project_authors::project.eq(pkg.id))
        .select(ProjectAuthor::as_select())
        .load(&mut conn)
        .await?;

    if !authors.iter().any(|v| v.user_id == user.id) && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let all_referencing = gallery_images::table
        .filter(gallery_images::s3_id.eq(img.s3_id.clone()))
        .select(GalleryImage::as_select())
        .load(&mut conn)
        .await?;

    if all_referencing.len() <= 1 {
        state
            .buckets
            .gallery
            .delete_object(format!("/{}", img.s3_id))
            .await?;
    }

    delete(gallery_images::table)
        .filter(gallery_images::id.eq(img.id))
        .execute(&mut conn)
        .await?;

    Ok(Response::builder().body(Body::new("Deleted gallery image successfully!".to_string()))?)
}
