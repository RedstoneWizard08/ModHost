//! The user list route.

use axum::{Json, extract::State, http::HeaderMap};
use axum_extra::extract::CookieJar;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{User, users};
use modhost_server_core::state::AppState;

/// List Users
///
/// Get a list of all registered users.
#[utoipa::path(
    get,
    path = "/users/list",
    tag = "Admin",
    responses(
        (status = 200, description = "Got users!", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    Ok(Json(
        users::table
            .select(User::as_select())
            .load(&mut conn)
            .await?,
    ))
}
