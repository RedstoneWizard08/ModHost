//! The remove admin route.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::extract::CookieJar;
use diesel::{ExpressionMethods, update};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{get_user, users};
use modhost_server_core::state::AppState;

/// Remove Admin
///
/// Demote a user from admin to a normal user.
/// If a user was not already an admin, this function will still succeed.
#[utoipa::path(
    delete,
    path = "/remove/{id}",
    tag = "Admin",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Removed!"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn remove_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Result<()> {
    let mut conn = state.pool.get().await?;
    let to_remove = get_user(user, &mut conn).await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    update(users::table)
        .filter(users::id.eq(to_remove.id))
        .set(users::admin.eq(false))
        .execute(&mut conn)
        .await?;

    Ok(())
}
