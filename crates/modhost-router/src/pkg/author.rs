//! Routes concerning project authors.

use modhost_core::AppError;
use modhost_core::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use modhost_db::{
    get_full_project, get_project, get_user, project_authors, ProjectAuthor, ProjectData,
    ProjectVisibility, User,
};
use diesel::{
    dsl::delete, insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_server_core::state::AppState;

/// Get Project Authors
///
/// Get a project's authors by its ID or slug.
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/authors",
    tag = "Projects",
    responses(
        (status = 200, description = "A list of project authors", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project ID or slug"),
    ),
)]
#[debug_handler]
pub async fn list_authors_handler(
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

/// Add Project Author
///
/// Add an author to a project.
#[utoipa::path(
    put,
    path = "/api/v1/projects/{id}/authors",
    tag = "Projects",
    responses(
        (status = 200, description = "Project updated successfully!", body = ProjectData),
        (status = UNAUTHORIZED, description = "You do not have access to modify this project!"),
        (status = BAD_REQUEST, description = "The user is already a member of the project!"),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    request_body(content = String, description = "The ID/username of the author to add."),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn add_author_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    body: String,
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

    let to_add = get_user(body, &mut conn).await?;

    if authors.iter().find(|v| v.user_id == to_add.id).is_some() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Author is already a member of the project!".to_string(),
            ))?);
    }

    insert_into(project_authors::table)
        .values(&ProjectAuthor {
            project: pkg.id,
            user_id: to_add.id,
        })
        .execute(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_project(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}

/// Remove Project Author
///
/// Remove an author from a project.
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{id}/authors",
    tag = "Projects",
    responses(
        (status = 200, description = "Project updated successfully!", body = ProjectData),
        (status = UNAUTHORIZED, description = "You do not have access to modify this project!"),
        (status = BAD_REQUEST, description = "The user is not a member of the project!"),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    request_body(content = String, description = "The ID/username of the author to remove."),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn remove_author_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    body: String,
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

    let to_remove = get_user(body, &mut conn).await?;

    if authors.iter().find(|v| v.user_id == to_remove.id).is_none() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Author is not a member of the project!".to_string(),
            ))?);
    }

    delete(project_authors::table)
        .filter(
            project_authors::project
                .eq(pkg.id)
                .and(project_authors::user_id.eq(to_remove.id)),
        )
        .execute(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_project(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}
