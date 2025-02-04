//! The version delete route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::{delete, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{project_authors, project_versions, version_files, ProjectAuthor, ProjectFile};
use modhost_db_util::{projects::get_project, vers::get_full_version};
use modhost_server_core::state::AppState;

/// Delete Project Version
///
/// Delete a project version
#[utoipa::path(
    delete,
    path = "/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Deleted project version!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
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
