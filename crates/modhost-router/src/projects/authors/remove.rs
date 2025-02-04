//! The remove author route.

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::{dsl::delete, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{get_user, project_authors, ProjectAuthor, ProjectData};
use modhost_db_util::projects::{get_full_project, get_project};
use modhost_server_core::state::AppState;

/// Remove Project Author
///
/// Remove an author from a project.
#[utoipa::path(
    delete,
    path = "/",
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
pub async fn remove_handler(
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
