//! The add admin route.

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

/// Add Admin
///
/// Promote a user to admin.
#[utoipa::path(
    put,
    path = "/add/{id}",
    tag = "Admin",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Added!"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn add_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Result<()> {
    let mut conn = state.pool.get().await?;
    let to_add = get_user(user, &mut conn).await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if !user.admin {
        return Err(AppError::NoAccess);
    }

    update(users::table)
        .filter(users::id.eq(to_add.id))
        .set(users::admin.eq(true))
        .execute(&mut conn)
        .await?;

    Ok(())
}
