//! Routes concerning user info.

use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use modhost_core::Result;
use modhost_db::{get_user, User};
use modhost_server_core::state::AppState;

/// Get User
///
/// Get information about a user.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Found user!", body = User),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! The user may not exist!"),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response> {
    Ok(
        Response::builder().body(Body::new(serde_json::to_string_pretty(
            &get_user(id, &mut state.pool.get().await?).await?,
        )?))?,
    )
}
