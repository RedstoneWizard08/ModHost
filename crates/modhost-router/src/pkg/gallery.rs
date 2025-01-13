//! Routes concerning project galleries.

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{
    gallery_images, get_full_project, get_gallery, get_gallery_image, get_project, project_authors,
    projects, GalleryImage, NewGalleryImage, Project, ProjectAuthor, ProjectVisibility,
    PublicGalleryImage,
};
use modhost_db_util::gallery::{get_image, transform_gallery, transform_gallery_image};
use modhost_server_core::state::AppState;
use sha1::{Digest, Sha1};

/// The data for uploading a gallery image.
/// This should be formatted as "multipart/form-data".
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct GalleryImageUpload {
    /// The project ID.
    pub project: i32,

    /// The display name of the image.
    pub name: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,

    /// The image file data itself.
    pub file: Vec<u8>,
}

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

/// Get Gallery Images
///
/// Get gallery images for a project.
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/gallery",
    tag = "Gallery",
    responses(
        (status = 200, description = "The project's gallery images.", body = Vec<PublicGalleryImage>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project ID or slug"),
    ),
)]
#[debug_handler]
pub async fn list_gallery_handler(
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

/// Get Gallery Image
///
/// Get information about a specific gallery image
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/gallery/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Found gallery image!", body = PublicGalleryImage),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this image is for."),
        ("image" = String, Path, description = "The image ID."),
    ),
)]
#[debug_handler]
pub async fn gallery_info_handler(
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

/// Get Gallery Image Data
///
/// Get a gallery image file data from S3.
/// A URL to this endpoint should be returned by any other gallery endpoints.
#[utoipa::path(
    get,
    path = "/api/v1/projects/s3/gallery/{id}",
    tag = "Gallery",
    responses(
        (status = 200, description = "The gallery image.", body = Vec<u8>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: image might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The gallery image's S3 ID."),
    ),
)]
#[debug_handler]
pub async fn s3_image_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Vec<u8>> {
    Ok(get_image(id, &state.buckets.gallery).await?)
}

/// Upload Gallery Image
///
/// Upload a gallery image
#[utoipa::path(
    put,
    path = "/api/v1/projects/{id}/gallery",
    tag = "Gallery",
    responses(
        (status = 200, description = "Created gallery image!", body = PublicGalleryImage),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this gallery image is for."),
    ),
    request_body(content = GalleryImageUpload, description = "The gallery image metadata"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn upload_gallery_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    mut data: Multipart,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_project(id, &mut conn).await?;

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

    let mut name = None;
    let mut description = None;
    let mut ordering = None;
    let mut file = None;

    while let Ok(Some(field)) = data.next_field().await {
        match field
            .name()
            .ok_or(anyhow!("Could not find a name for a field!"))?
        {
            "name" => name = Some(field.text().await?),
            "description" => description = Some(field.text().await?),
            "ordering" => ordering = Some(field.text().await?),
            "file" => file = Some(field.bytes().await?),
            _ => {}
        }
    }

    if name.is_none() {
        Err(anyhow!("Missing field: 'name'"))?;
    }

    if file.is_none() {
        Err(anyhow!("Missing field: 'file'"))?;
    }

    let name = name.unwrap();
    let ordering = ordering.unwrap_or("-1".into()).parse()?;
    let file = file.unwrap();
    let file_format = imghdr::from_bytes(&file).ok_or(anyhow!("Invalid image file!"))?;
    let mut hasher = Sha1::new();

    hasher.update(&file);

    let file_id = format!("{:x}", hasher.finalize());
    let file_name = format!("{}.{}", file_id, file_format);

    state
        .buckets
        .gallery
        .put_object(format!("/{}", file_name), &file)
        .await?;

    let data = NewGalleryImage {
        project: pkg.id,
        name,
        description,
        ordering,
        s3_id: file_name,
    };

    update(projects::table)
        .filter(projects::id.eq(pkg.id))
        .set(projects::updated_at.eq(Utc::now().naive_utc()))
        .returning(Project::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    let image = insert_into(gallery_images::table)
        .values(&data)
        .returning(GalleryImage::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&transform_gallery_image(
            image,
        ))?))?)
}

/// Delete Gallery Image
///
/// Delete a gallery image
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{id}/gallery/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Deleted gallery image!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this gallery image is for."),
        ("image" = String, Path, description = "The gallery image ID number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_gallery_handler(
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

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
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

/// Update Gallery Image
///
/// Update gallery image metadata
#[utoipa::path(
    patch,
    path = "/api/v1/projects/{id}/gallery/{image}",
    tag = "Gallery",
    responses(
        (status = 200, description = "Updated gallery image!", body = PublicGalleryImage),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this gallery image is for."),
        ("image" = String, Path, description = "The gallery image ID."),
    ),
    request_body(content = PartialGalleryImage, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_gallery_handler(
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
