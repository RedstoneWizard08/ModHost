use crate::models::LegacyManifest;
use anyhow::{anyhow, Result};
use http_body_util::BodyExt;
use itertools::Itertools;
use octocrab::Octocrab;
use std::collections::HashMap;

pub const PKGS_JSON: &str =
    "https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/refs/heads/main/pkgs.json";

/// Get the packages from KJSPKG. Will return a map of IDs to GitHub repositories.
pub async fn get_packages_map() -> Result<HashMap<String, String>> {
    Ok(reqwest::get(PKGS_JSON).await?.json().await?)
}

/// Get the owner of the package - this will be the person who has contributed most.
/// Returns a tuple of their username and their github user ID.
pub async fn get_github_owner(client: &Octocrab, repo: impl AsRef<str>) -> Result<(String, u64)> {
    let repo = repo.as_ref();

    let (owner, repo) = repo
        .split("/")
        .collect_tuple()
        .ok_or(anyhow!("Could not parse repo: {}", repo))?;

    let contribs = client
        .repos(owner, repo)
        .list_contributors()
        .send()
        .await?
        .items;

    let user = contribs
        .first()
        .ok_or(anyhow!("No contributors for repo: {}", repo))?
        .clone()
        .author;

    Ok((user.login, *user.id))
}

/// Get the [`LegacyManifest`] for a repository.
pub async fn get_manifest(
    client: &Octocrab,
    repo: impl AsRef<str>,
) -> Result<Option<LegacyManifest>> {
    let repo = repo.as_ref();

    let (owner, repo) = repo
        .split("/")
        .collect_tuple()
        .ok_or(anyhow!("Could not parse repo: {}", repo))?;

    let repo_info = client.repos(owner, repo).get().await?;
    let branch = repo_info.default_branch.unwrap_or("main".into());

    match client
        .repos(owner, repo)
        .raw_file(branch.clone(), ".kjspkg")
        .await
    {
        Ok(resp) => {
            if !resp.status().is_success() {
                let body = resp.into_body().collect().await?.to_bytes();
                let body = String::from_utf8(body.to_vec())?;

                warn!(
                    "Could not fetch '.kjspkg' from branch '{}' in repo '{}/{}': {}",
                    branch, owner, repo, body
                );

                Ok(None)
            } else {
                let body = resp.into_body().collect().await?.to_bytes();

                Ok(Some(serde_json::from_slice(body.to_vec().as_slice())?))
            }
        }

        Err(err) => {
            warn!(
                "Could not fetch '.kjspkg' from branch '{}' in repo '{}/{}': {}",
                branch, owner, repo, err
            );

            Ok(None)
        }
    }
}

/// Get the readme for a repositoriy.
pub async fn get_readme(client: &Octocrab, repo: impl AsRef<str>) -> Result<String> {
    let repo = repo.as_ref();

    let (owner, repo) = repo
        .split("/")
        .collect_tuple()
        .ok_or(anyhow!("Could not parse repo: {}", repo))?;

    Ok(client
        .repos(owner, repo)
        .get_readme()
        .send()
        .await?
        .decoded_content()
        .ok_or(anyhow!(
            "Could not get readme content for repo: {}/{}",
            owner,
            repo
        ))?)
}

/// Get the tarball for a repository.
/// Returns a tuple with the commit SHA and the tarball itself.
pub async fn get_package_tarball(
    client: &Octocrab,
    repo: impl AsRef<str>,
) -> Result<(String, Vec<u8>)> {
    let repo = repo.as_ref();

    let (owner, repo) = repo
        .split("/")
        .collect_tuple()
        .ok_or(anyhow!("Could not parse repo: {}", repo))?;

    let repo_info = client.repos(owner, repo).get().await?;
    let branch = repo_info.default_branch.unwrap_or("main".into());
    let commit = client
        .repos(owner, repo)
        .list_branches()
        .send()
        .await?
        .items
        .into_iter()
        .find(|v| v.name == branch)
        .ok_or(anyhow!(
            "Could not get info for branch '{}' in repo '{}/{}'!",
            branch,
            owner,
            repo
        ))?
        .commit
        .sha;

    Ok((
        commit,
        client
            .repos(owner, repo)
            .download_tarball(branch)
            .await?
            .into_body()
            .collect()
            .await?
            .to_bytes()
            .to_vec(),
    ))
}
