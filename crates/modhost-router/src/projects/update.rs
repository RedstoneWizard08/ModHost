//! The project update route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use diesel::{update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{
    project_authors, projects, Project, ProjectAuthor, ProjectData, ProjectVisibility,
};
use modhost_db_util::projects::{get_full_project, get_project};
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

/// Update Project
///
/// Update a project's information.
#[utoipa::path(
    patch,
    path = "/{id}",
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
pub async fn update_handler(
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
