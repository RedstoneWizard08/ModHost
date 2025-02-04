//! ModHost's badge routes.

use axum::{body::Body, debug_handler, extract::Query, response::Response};
use modhost_core::Result;
use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::{generator::generate_badge, models::BadgeOptions};

/// Badge
///
/// Generate a badge
#[utoipa::path(
    get,
    path = "/badge",
    tag = "Meta",
    params(BadgeOptions),
    responses(
        (status = 200, description = "Badge generated!"),
    ),
)]
#[debug_handler]
pub async fn badge_route(Query(opts): Query<BadgeOptions>) -> Result<Response> {
    Ok(Response::builder()
        .header(CONTENT_TYPE, "image/svg+xml")
        .status(StatusCode::OK)
        .body(Body::new(generate_badge(opts).await?))?)
}
