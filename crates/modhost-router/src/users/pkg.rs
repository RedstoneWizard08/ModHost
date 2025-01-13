//! Routes concerning user projects.

use modhost_core::Result;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use modhost_db::{get_user, ProjectData};
use modhost_db_util::users::get_user_projects;
use modhost_auth::get_user_from_req;
use modhost_server_core::state::AppState;

/// Get User Projects
///
/// Get a user's projects.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/projects",
    tag = "Users",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Found projects!", body = Vec<ProjectData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! The user may not exist!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<ProjectData>>> {
    let mut conn = state.pool.get().await?;
    let user = get_user(id, &mut conn).await?;

    Ok(Json(
        get_user_projects(
            get_user_from_req(&jar, &headers, &mut conn).await.ok(),
            user.id,
            &mut conn,
        )
        .await?,
    ))
}
