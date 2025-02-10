//! The version update route.

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, update};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{ProjectAuthor, ProjectVersion, get_version, project_authors, project_versions};
use modhost_db_util::projects::get_project;
use modhost_server_core::state::AppState;
use semver::Version;

/// Information for updaing a project version.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PartialProjectVersion {
    /// The display name of the version.
    #[serde(default)]
    pub name: Option<String>,

    /// The version number.
    /// This must be a string confirming to the [SemVer](https://semver.org/) standard.
    #[serde(default)]
    pub version_number: Option<String>,

    /// The version changelog.
    #[serde(default)]
    pub changelog: Option<String>,

    /// The mod loaders this version works on.
    #[serde(default)]
    pub loaders: Option<Vec<String>>,

    /// The game versions this version works on.
    #[serde(default)]
    pub game_versions: Option<Vec<String>>,
}

/// Update Project Version
///
/// Update information about project version
#[utoipa::path(
    patch,
    path = "/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Updated project version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    request_body(content = PartialProjectVersion, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, version)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(data): Json<PartialProjectVersion>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_project(project, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

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

    if let Some(ver_num) = &data.version_number {
        Version::parse(ver_num)?;
    }

    let ver = update(project_versions::table)
        .filter(project_versions::id.eq(ver.id))
        .set((
            project_versions::name.eq(data.name.unwrap_or(ver.name)),
            project_versions::version_number.eq(data.version_number.unwrap_or(ver.version_number)),
            project_versions::changelog.eq(data.changelog.map(Some).unwrap_or(ver.changelog)),
            project_versions::loaders.eq(data
                .loaders
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.loaders)),
            project_versions::game_versions.eq(data
                .game_versions
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.game_versions)),
            project_versions::updated_at.eq(Utc::now().naive_utc()),
        ))
        .returning(ProjectVersion::as_select())
        .get_result(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}
