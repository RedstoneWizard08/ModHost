//! The project delete route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, delete};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{ProjectAuthor, project_authors, projects};
use modhost_db_util::projects::get_project;
use modhost_server_core::state::AppState;

/// Delete Project
///
/// Delete a project
#[utoipa::path(
    delete,
    path = "/{id}",
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
pub async fn delete_handler(
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

    if !authors.iter().any(|v| v.user_id == user.id) && !user.admin {
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
