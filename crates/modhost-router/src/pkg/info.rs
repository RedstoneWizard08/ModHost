//! Routes concerning project information.

use modhost_core::AppError;
use modhost_core::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use modhost_db::{
    get_full_project, get_project, project_authors, projects, NewProject, Project, ProjectAuthor,
    ProjectData, ProjectVisibility,
};
use diesel::{
    delete, insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_server_core::state::AppState;

/// A partial project for updating a project.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PartialProject {
    /// The display name of the project.
    #[serde(default)]
    pub name: Option<String>,

    /// The project's readme.
    #[serde(default)]
    pub readme: Option<String>,

    /// A short description of the project.
    #[serde(default)]
    pub description: Option<String>,

    /// The project's source code URL.
    #[serde(default)]
    pub source: Option<String>,

    /// The project's issues URL.
    #[serde(default)]
    pub issues: Option<String>,

    /// The project's wiki URL.
    #[serde(default)]
    pub wiki: Option<String>,

    /// The project's visibility.
    #[serde(default)]
    pub visibility: Option<ProjectVisibility>,

    /// The project's license.
    #[serde(default)]
    pub license: Option<String>,

    /// The project's tags.
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// Create Project
///
/// Create a project
#[utoipa::path(
    put,
    path = "/api/v1/projects",
    tag = "Projects",
    responses(
        (status = 200, description = "Project created successfully!", body = ProjectData),
        (status = 401, description = "Project already exists!"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    request_body(content = NewProject, description = "Information about the project to create"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_project_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<NewProject>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if let Some(_) = projects::table
        .filter(projects::slug.eq(body.slug.clone()))
        .select(Project::as_select())
        .first(&mut conn)
        .await
        .optional()?
    {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Project with that slug already exists!".to_string(),
            ))?);
    }

    let pkg = insert_into(projects::table)
        .values(&body)
        .returning(Project::as_returning())
        .get_result(&mut conn)
        .await?;

    insert_into(project_authors::table)
        .values(&ProjectAuthor {
            project: pkg.id,
            user_id: user.id,
        })
        .execute(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder().body(Body::new(serde_json::to_string(
        &get_full_project(pkg.id.to_string(), &mut conn).await?,
    )?))?)
}

/// Get Project
///
/// Get a project by its ID or slug.
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}",
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
pub async fn project_info_handler(
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

/// Update Project
///
/// Update a project's information.
#[utoipa::path(
    patch,
    path = "/api/v1/projects/{id}",
    tag = "Projects",
    responses(
        (status = 200, description = "Project updated successfully!", body = ProjectData),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    request_body(content = PartialProject, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_project_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(data): Json<PartialProject>,
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

    let pkg = update(projects::table)
        .filter(projects::id.eq(pkg.id))
        .set((
            projects::name.eq(data.name.unwrap_or(pkg.name)),
            projects::readme.eq(data.readme.unwrap_or(pkg.readme)),
            projects::description.eq(data.description.unwrap_or(pkg.description)),
            projects::source.eq(data.source.map(|v| Some(v)).unwrap_or(pkg.source)),
            projects::issues.eq(data.issues.map(|v| Some(v)).unwrap_or(pkg.issues)),
            projects::wiki.eq(data.wiki.map(|v| Some(v)).unwrap_or(pkg.wiki)),
            projects::visibility.eq(data.visibility.unwrap_or(pkg.visibility)),
            projects::license.eq(data.license.map(|v| Some(v)).unwrap_or(pkg.license)),
            projects::tags.eq(data
                .tags
                .map(|v| v.into_iter().map(|v| Some(v)).collect::<Vec<_>>())
                .unwrap_or(pkg.tags)),
        ))
        .returning(Project::as_select())
        .get_result(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_project(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}

/// Delete Project
///
/// Delete a project
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{id}",
    tag = "Projects",
    responses(
        (status = 200, description = "Project deleted successfully!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Error: project might not exist, or another error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_project_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
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

    delete(projects::table)
        .filter(projects::id.eq(pkg.id))
        .execute(&mut conn)
        .await?;

    state.search.delete_project(pkg.id).await?;

    Ok(Response::builder().body(Body::new("Deleted project successfully!".to_string()))?)
}
