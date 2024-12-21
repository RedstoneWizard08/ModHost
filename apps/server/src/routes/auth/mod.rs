//! Routes concerning authentication.
//! A lot of this was taken and HEAVILY modified from:
//! https://github.com/AbrarNitk/auth/blob/main/service/auth/src/github/mod.rs
//! (From the incredible post: https://medium.com/@abrar.nitk/rust-authentication-with-github-oauth-3c581fa274a1)

pub mod callback;
pub mod login;

use crate::state::AppState;
use axum::{routing::get, Router};
use callback::callback_handler;
use login::login_handler;

/// The relative URL for the GitHub auth callback.
pub const CALLBACK_URL: &str = "/api/v1/auth/github/callback";

/// Register auth-related routes
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/github/login", get(login_handler))
        .route("/github/callback", get(callback_handler))
        .with_state(state)
}
