//! The get user route.

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::extract::CookieJar;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{User, get_user};
use modhost_server_core::state::AppState;

/// Get User
///
/// Get a user.
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Admin",
    responses(
        (status = 200, description = "Got user info!", body = User),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn get_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    let found = get_user(id, &mut conn).await?;

    Ok(Json(found))
}
