//! The gallery image update route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use diesel::{update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{
    gallery_images, get_gallery_image, project_authors, GalleryImage, ProjectAuthor,
    PublicGalleryImage,
};
use modhost_db_util::{gallery::transform_gallery_image, projects::get_project};
use modhost_server_core::state::AppState;

/// Data for updating a gallery image.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct PartialGalleryImage {
    /// The display name of the image.
    #[serde(default)]
    pub name: Option<String>,

    /// An optional markdown-formatted description.
    #[serde(default)]
    pub description: Option<String>,

    /// The order of this image.
    #[serde(default)]
    pub ordering: Option<i32>,
}

/// Update Gallery Image
///
/// Update gallery image metadata
#[utoipa::path(
    patch,
    path = "/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Updated gallery image!", body = PublicGalleryImage),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("image" = String, Path, description = "The gallery image ID."),
    ),
    request_body(content = PartialGalleryImage, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, image)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(data): Json<PartialGalleryImage>,
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

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let img = update(gallery_images::table)
        .filter(gallery_images::id.eq(img.id))
        .set((
            gallery_images::name.eq(data.name.unwrap_or(img.name)),
            gallery_images::ordering.eq(data.ordering.unwrap_or(img.ordering)),
            gallery_images::description
                .eq(data.description.map(|v| Some(v)).unwrap_or(img.description)),
            gallery_images::updated_at.eq(Utc::now().naive_utc()),
        ))
        .returning(GalleryImage::as_select())
        .get_result(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&transform_gallery_image(
            img,
        ))?))?)
}
