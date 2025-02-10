//! ModHost's moderation routes.

use axum::{Router, routing::get};
use modhost_server_core::state::AppState;

pub mod queue;

/// Register moderation-related routes onto the router.
/// This should be nested at `/api/v1/moderation`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/queue", get(queue::list_queue))
        .route("/queue/pending", get(queue::list_queue_pending))
        .route("/queue/approved", get(queue::list_queue_approved))
        .route("/queue/under_review", get(queue::list_queue_under_review))
        .route("/queue/denied", get(queue::list_queue_denied))
        .with_state(state)
}

/// The spec for the moderation API.
/// Should be nested at `/api/v1/moderation`.
#[derive(OpenApi)]
#[openapi(paths(
    queue::list_queue,
    queue::list_queue_pending,
    queue::list_queue_approved,
    queue::list_queue_under_review,
    queue::list_queue_denied,
))]
pub struct ModerationApi;
