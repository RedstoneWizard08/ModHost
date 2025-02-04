//! The project create route.

use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::Result;
use modhost_db::{project_authors, projects, NewProject, Project, ProjectAuthor, ProjectData};
use modhost_db_util::projects::get_full_project;
use modhost_server_core::state::AppState;

/// Create Project
///
/// Create a project
#[utoipa::path(
    put,
    path = "/",
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
pub async fn create_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<NewProject>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if body.slug.is_empty() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new("Slug must not be empty!".to_string()))?);
    }

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
