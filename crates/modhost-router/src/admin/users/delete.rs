//! The delete user route.

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::extract::CookieJar;
use diesel::{ExpressionMethods, SelectableHelper, delete};
use diesel_async::RunQueryDsl;
use modhost_auth::get_user_from_req;
use modhost_core::{AppError, Result};
use modhost_db::{User, get_user, users};
use modhost_server_core::state::AppState;

/// Delete User
///
/// Delete a user account.
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Admin",
    responses(
        (status = 200, description = "Deleted!", body = User),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
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

    let to_delete = get_user(id, &mut conn).await?;

    Ok(Json(
        delete(users::table)
            .filter(users::id.eq(to_delete.id))
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await?,
    ))
}
