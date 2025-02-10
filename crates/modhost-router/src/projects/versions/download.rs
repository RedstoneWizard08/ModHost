//! The version download route.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::extract::CookieJar;
use diesel::{ExpressionMethods, SelectableHelper, update};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::AppError;
use modhost_core::Result;
use modhost_db::{
    Project, ProjectVersion, ProjectVisibility, get_version, project_versions, projects,
};
use modhost_db_util::{projects::get_full_project, vers::get_version_file};
use modhost_server_core::state::AppState;
use object_store::ObjectStore;

/// Download Project Version
///
/// Download a specific project version
#[utoipa::path(
    get,
    path = "/{version}/download/{file}",
    tag = "Versions",
    responses(
        (status = 307, description = "Redirecting to download"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("version" = String, Path, description = "The version ID/name/number."),
        ("file" = String, Path, description = "The file ID/name."),
    ),
)]
#[debug_handler]
pub async fn download_handler(
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
        .get(&format!("/{}", file.s3_id).into())
        .await?
        .bytes()
        .await?
        .to_vec();

    Ok(bytes)
}
