//! Routes concerning authentication.
//! A lot of this was taken and HEAVILY modified from:
//! https://github.com/AbrarNitk/auth/blob/main/service/auth/src/github/mod.rs
//! (From the incredible post: https://medium.com/@abrar.nitk/rust-authentication-with-github-oauth-3c581fa274a1)

pub mod callback;
pub mod login;

use axum::{Router, routing::get};
use modhost_server_core::state::AppState;

/// The relative URL for the GitHub auth callback.
pub const CALLBACK_URL: &str = "/api/v1/auth/github/callback";

/// Register auth-related routes.
/// Should be nested at `/api/v1/auth`.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/github/login", get(login::login_handler))
        .route("/github/callback", get(callback::callback_handler))
        .with_state(state)
}

/// The spec for the auth API.
/// Should be nested at `/api/v1/auth`.
#[derive(OpenApi)]
#[openapi(paths(login::login_handler, callback::callback_handler,))]
pub struct AuthApi;
