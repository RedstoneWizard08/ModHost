//! Routes concerning project versions.

use crate::{
    auth::get_user_from_req, state::AppState, util::versions::get_latest_full_version, Result,
};
use anyhow::anyhow;
use app_core::AppError;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use db::{
    get_full_project, get_project, get_version, project_authors, project_versions, projects,
    version_files, NewProjectFile, NewProjectVersion, Project, ProjectAuthor, ProjectFile,
    ProjectVersion, ProjectVersionData, ProjectVersionInit, ProjectVisibility,
};
use db_util::vers::{get_full_version, get_version_file, get_versions};
use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use semver::Version;
use sha1::{Digest, Sha1};

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

/// List Project Versions
///
/// List available versions for a specific project.
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Found project versions!", body = Vec<ProjectVersionData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project ID whose versions we are looking for."),
    ),
)]
#[debug_handler]
pub async fn list_versions_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectVersionData>>> {
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

    Ok(Json(get_versions(pkg.id, &mut conn).await?))
}

/// Get Project Version
///
/// Get information about a specific project version
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Found project version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
)]
#[debug_handler]
pub async fn version_info_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<ProjectVersionData>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_project(project, &mut conn).await?;

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

    Ok(Json(get_full_version(pkg.id, version, &mut conn).await?))
}

/// Get Latest Project Version
///
/// Get information about the latest project version
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/versions/latest",
    tag = "Versions",
    responses(
        (status = 200, description = "Found latest version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
    ),
)]
#[debug_handler]
pub async fn latest_version_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(project): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ProjectVersionData>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_project(project, &mut conn).await?;

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

    Ok(Json(get_latest_full_version(pkg.id, &mut conn).await?))
}

/// Download Project Version
///
/// Download a specific project version
#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}/versions/{version}/download/{file}",
    tag = "Versions",
    responses(
        (status = 307, description = "Redirecting to download"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
        ("file" = String, Path, description = "The file ID/name."),
    ),
)]
#[debug_handler]
pub async fn download_version_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, version, file)): Path<(String, String, String)>,
    State(state): State<AppState>,
) -> Result<Vec<u8>> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_project(project, &mut conn).await?;

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

    let ver = get_version(pkg.id, version, &mut conn).await?;
    let file = get_version_file(ver.id, file, &mut conn).await?;

    update(projects::table)
        .filter(projects::id.eq(pkg.id))
        .set((
            projects::downloads.eq(pkg.downloads + 1),
            projects::updated_at.eq(pkg.updated_at),
        ))
        .returning(Project::as_returning())
        .get_result(&mut conn)
        .await?;

    update(project_versions::table)
        .filter(project_versions::id.eq(ver.id))
        .set((
            project_versions::downloads.eq(ver.downloads + 1),
            project_versions::updated_at.eq(ver.updated_at),
        ))
        .returning(ProjectVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    let bytes = state
        .buckets
        .projects
        .get_object(format!("/{}", file.s3_id))
        .await?
        .into_bytes()
        .to_vec();

    Ok(bytes)
}

/// Upload Project Version
///
/// Upload a new project version
#[utoipa::path(
    put,
    path = "/api/v1/projects/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Created project version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
    ),
    request_body(content = ProjectVersionInit, description = "The version data"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_version_handler(
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
    let mut version_number = None;
    let mut changelog = None;
    let mut loaders = None;
    let mut game_versions = None;
    let mut file = None;
    let mut file_name = None;

    while let Ok(Some(field)) = data.next_field().await {
        match field
            .name()
            .ok_or(anyhow!("Could not find a name for a field!"))?
        {
            "name" => name = Some(field.text().await?),
            "version_number" => version_number = Some(field.text().await?),
            "changelog" => changelog = Some(field.text().await?),
            "loaders" => {
                loaders = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "game_versions" => {
                game_versions = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "file" => file = Some(field.bytes().await?),
            "file_name" => file_name = Some(field.text().await?),
            _ => {}
        }
    }

    if name.is_none() {
        Err(anyhow!("Missing field: 'name'"))?;
    }

    if version_number.is_none() {
        Err(anyhow!("Missing field: 'version_number'"))?;
    }

    if loaders.is_none() {
        Err(anyhow!("Missing field: 'loaders'"))?;
    }

    if game_versions.is_none() {
        Err(anyhow!("Missing field: 'game_versions'"))?;
    }

    if file.is_none() {
        Err(anyhow!("Missing field: 'file'"))?;
    }

    if file_name.is_none() {
        Err(anyhow!("Missing field: 'file_name'"))?;
    }

    let name = name.unwrap();
    let version_number = version_number.unwrap();
    let loaders = loaders.unwrap();
    let game_versions = game_versions.unwrap();
    let file = file.unwrap();
    let file_name = file_name.unwrap();

    Version::parse(&version_number)?;

    if !(state.verifier)(file.clone()) {
        Err(anyhow!("Invalid project!"))?;
    }

    let mut hasher = Sha1::new();

    hasher.update(&file);

    let file_id = format!("{:x}", hasher.finalize());

    state
        .buckets
        .projects
        .put_object(format!("/{}", file_id), &file)
        .await?;

    let data = NewProjectVersion {
        project: pkg.id,
        name,
        version_number,
        changelog,
        loaders,
        game_versions,
        downloads: 0,
    };

    update(projects::table)
        .filter(projects::id.eq(pkg.id))
        .set(projects::updated_at.eq(Utc::now().naive_utc()))
        .returning(Project::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    let ver = insert_into(project_versions::table)
        .values(&data)
        .returning(ProjectVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    let file = NewProjectFile {
        file_name,
        sha1: file_id.clone(),
        s3_id: file_id,
        version_id: ver.id,
        size: file.len() as i64,
    };

    insert_into(version_files::table)
        .values(&file)
        .returning(ProjectFile::as_returning())
        .get_result(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Update Project Version
///
/// Update information about project version
#[utoipa::path(
    patch,
    path = "/api/v1/projects/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Updated project version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    request_body(content = PartialProjectVersion, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_version_handler(
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

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
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
            project_versions::changelog
                .eq(data.changelog.map(|v| Some(v)).unwrap_or(ver.changelog)),
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

/// Delete Project Version
///
/// Delete a project version
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Deleted project version!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The project that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_version_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((project, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_project(project, &mut conn).await?;
    let ver = get_full_version(pkg.id, version, &mut conn).await?;

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

    for file in ver.files {
        let all_referencing = version_files::table
            .filter(version_files::s3_id.eq(file.s3_id.clone()))
            .select(ProjectFile::as_select())
            .load(&mut conn)
            .await?;

        if all_referencing.len() <= 1 {
            state
                .buckets
                .projects
                .delete_object(format!("/{}", file.s3_id))
                .await?;
        }
    }

    delete(project_versions::table)
        .filter(project_versions::id.eq(ver.id))
        .execute(&mut conn)
        .await?;

    state.search.update_project(pkg.id, &mut conn).await?;

    Ok(Response::builder().body(Body::new(
        "Deleted project version successfully!".to_string(),
    ))?)
}
