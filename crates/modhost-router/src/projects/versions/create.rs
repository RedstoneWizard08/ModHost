//! The version create route.

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use diesel::{insert_into, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{
    project_authors, project_versions, projects, version_files, NewProjectFile, NewProjectVersion,
    Project, ProjectAuthor, ProjectFile, ProjectVersion, ProjectVersionInit,
};
use modhost_db_util::projects::get_project;
use modhost_server_core::state::AppState;
use semver::Version;
use sha1::{Digest, Sha1};

/// Upload Project Version
///
/// Upload a new project version
#[utoipa::path(
    put,
    path = "/",
    tag = "Versions",
    responses(
        (status = 200, description = "Created project version!", body = ProjectVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    request_body(content = inline(ProjectVersionInit), description = "The version data", content_type = "multipart/form-data"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_handler(
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
