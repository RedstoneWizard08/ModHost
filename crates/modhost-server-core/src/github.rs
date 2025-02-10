//! Utilities for working with [`octocrab`].

use octocrab::Octocrab;

/// Create a GitHub API client.
pub fn create_github_client(token: impl AsRef<str>) -> octocrab::Result<Octocrab> {
    Octocrab::builder().personal_token(token.as_ref()).build()
}
