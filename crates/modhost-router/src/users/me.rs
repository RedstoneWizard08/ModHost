//! Routes concerning the current user.

use modhost_core::Result;
use axum::{body::Body, extract::State, http::HeaderMap, response::Response};
use axum_extra::extract::CookieJar;
use modhost_db::User;
use modhost_auth::get_user_from_req;
use modhost_server_core::state::AppState;

/// Current User
///
/// Get information about the current authenticated user.
#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    responses(
        (status = 200, description = "Found user!", body = User),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! Are you authenticated?"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn me_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
) -> Result<Response> {
    Ok(
        Response::builder().body(Body::new(serde_json::to_string_pretty(
            &get_user_from_req(&jar, &headers, &mut state.pool.get().await?).await?,
        )?))?,
    )
}
