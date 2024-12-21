//! ModHost's utilities.

pub mod gallery;
pub mod sanitize;
pub mod scheme;
pub mod versions;

use octocrab::Octocrab;

/// Create a GitHub API client.
pub fn create_github_client(token: impl AsRef<str>) -> octocrab::Result<Octocrab> {
    Ok(Octocrab::builder().personal_token(token.as_ref()).build()?)
}
