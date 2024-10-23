use crate::{auth::get_user_from_req, state::AppState, HttpResult, User};
use axum::{body::Body, extract::State, http::HeaderMap, response::Response};
use axum_extra::extract::CookieJar;

/// Current User
///
/// Get information about the current user.
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
) -> HttpResult<Response> {
    Ok(
        Response::builder().body(Body::new(serde_json::to_string_pretty(
            &get_user_from_req(&jar, &headers, &mut state.pool.get().await?).await?,
        )?))?,
    )
}
