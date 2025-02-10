//! Admin stats route.

use axum::{Json, extract::State, http::HeaderMap};
use axum_extra::extract::CookieJar;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use futures::StreamExt;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result, uptime_secs};
use modhost_db::{projects, users};
use modhost_server_core::state::AppState;
use object_store::ObjectStore;

/// Stats for admins.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct AdminStats {
    /// The number of projects created.
    pub projects: u64,

    /// The number of indexed projects in search.
    pub indexed_projects: u64,

    /// The number of users.
    pub users: u64,

    /// The instance uptime in seconds.
    pub uptime_secs: u64,

    /// The size of the projects bucket in bytes.
    pub projects_size_bytes: usize,

    /// The size of the gallery bucket in bytes.
    pub gallery_size_bytes: usize,
}

/// Stats
///
/// Get statistics about this ModHost instance.
#[utoipa::path(
    get,
    path = "/stats",
    tag = "Admin",
    responses(
        (status = 200, description = "Got stats!", body = AdminStats),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn stats_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AdminStats>> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    let mut stream = state.buckets.projects.list(None);
    let mut projects_size_bytes = 0;

    while let Some(obj) = stream.next().await {
        if let Ok(obj) = obj {
            projects_size_bytes += obj.size;
        }
    }

    let mut stream = state.buckets.gallery.list(None);
    let mut gallery_size_bytes = 0;

    while let Some(obj) = stream.next().await {
        if let Ok(obj) = obj {
            gallery_size_bytes += obj.size;
        }
    }

    Ok(Json(AdminStats {
        projects: projects::table.count().get_result::<i64>(&mut conn).await? as u64,
        indexed_projects: state
            .search
            .projects()
            .get_pagination()
            .await?
            .max_total_hits as u64,
        users: users::table.count().get_result::<i64>(&mut conn).await? as u64,
        uptime_secs: uptime_secs(),
        projects_size_bytes,
        gallery_size_bytes,
    }))
}
