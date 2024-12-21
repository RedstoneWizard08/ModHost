//! Routes concerning package authors.

use crate::{auth::get_user_from_req, state::AppState, Result};
use app_core::AppError;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use db::{
    get_full_package, get_package, get_user, package_authors, PackageAuthor, PackageData,
    PackageVisibility, User,
};
use diesel::{
    dsl::delete, insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

/// Get Package Authors
///
/// Get a package's authors by its ID or slug.
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/authors",
    tag = "Packages",
    responses(
        (status = 200, description = "A list of package authors", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package ID or slug"),
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
    let pkg = get_full_package(id, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
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

/// Add Package Author
///
/// Add an author to a package.
#[utoipa::path(
    put,
    path = "/api/v1/packages/{id}/authors",
    tag = "Packages",
    responses(
        (status = 200, description = "Package updated successfully!", body = PackageData),
        (status = UNAUTHORIZED, description = "You do not have access to modify this package!"),
        (status = BAD_REQUEST, description = "The user is already a member of the project!"),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    request_body(content = String, description = "The ID/username of the author to add."),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn add_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    body: String,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let to_add = get_user(body, &mut conn).await?;

    if authors.iter().find(|v| v.user_id == to_add.id).is_some() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Author is already a member of the project!".to_string(),
            ))?);
    }

    insert_into(package_authors::table)
        .values(&PackageAuthor {
            package: pkg.id,
            user_id: to_add.id,
        })
        .execute(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_package(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}

/// Remove Package Author
///
/// Remove an author from a package.
#[utoipa::path(
    delete,
    path = "/api/v1/packages/{id}/authors",
    tag = "Packages",
    responses(
        (status = 200, description = "Package updated successfully!", body = PackageData),
        (status = UNAUTHORIZED, description = "You do not have access to modify this package!"),
        (status = BAD_REQUEST, description = "The user is not a member of the project!"),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    request_body(content = String, description = "The ID/username of the author to remove."),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn remove_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    body: String,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let to_remove = get_user(body, &mut conn).await?;

    if authors.iter().find(|v| v.user_id == to_remove.id).is_none() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Author is not a member of the project!".to_string(),
            ))?);
    }

    delete(package_authors::table)
        .filter(
            package_authors::package
                .eq(pkg.id)
                .and(package_authors::user_id.eq(to_remove.id)),
        )
        .execute(&mut conn)
        .await?;

    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_package(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}
