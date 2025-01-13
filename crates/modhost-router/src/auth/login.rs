//! The route to initiate the GitHub login flow.

use std::collections::HashMap;

use super::CALLBACK_URL;
use modhost_core::Result;
use axum::{
    body::Body,
    extract::State,
    http::{header::LOCATION, HeaderValue, StatusCode, Uri},
    response::Response,
};
use axum_extra::extract::Host;
use modhost_middleware::scheme::Scheme;
use modhost_server_core::state::AppState;
use oauth2::{CsrfToken, RedirectUrl, Scope};

/// GitHub Auth Login
///
/// Initiate the GitHub login flow.
#[utoipa::path(
    get,
    path = "/api/v1/auth/github/login",
    tag = "Auth",
    params(
        ("redirect_uri" = Option<String>, Query, description = "An optional custom URL to redirect to."),
    ),
    responses(
        (status = 307, description = "Redirecting to GitHub for login"),
    ),
)]
#[debug_handler]
pub async fn login_handler(
    State(state): State<AppState>,
    Host(host): Host,
    Scheme(scheme): Scheme,
    url: Uri,
) -> Result<Response> {
    let query = url::form_urlencoded::parse(url.query().unwrap_or_default().as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let callback_url_base = query
        .get("redirect_uri")
        .cloned()
        .unwrap_or("/".to_string());

    let callback_url = format!(
        "{}://{}{}?to={}",
        scheme, host, CALLBACK_URL, callback_url_base
    );

    let client = state
        .auth
        .set_redirect_uri(RedirectUrl::new(callback_url).unwrap());

    let (mut authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("public_repo".to_string()))
        .url();

    authorize_url
        .query_pairs_mut()
        .append_pair("prompt", "consent");

    let mut resp = Response::new(Body::empty());

    resp.headers_mut()
        .insert(LOCATION, HeaderValue::from_str(authorize_url.as_str())?);

    *resp.status_mut() = StatusCode::TEMPORARY_REDIRECT;

    Ok(resp)
}
