use crate::models::LegacyManifest;
use anyhow::{anyhow, Result};
use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use http_body_util::BodyExt;
use octocrab::Octocrab;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};
use tar::{Archive, Builder as TarBuilder};

pub const PKGS_JSON: &str =
    "https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/refs/heads/main/pkgs.json";

/// Get the packages from KJSPKG. Will return a map of IDs to GitHub repositories.
pub async fn get_packages_map() -> Result<HashMap<String, String>> {
    Ok(reqwest::get(PKGS_JSON).await?.json().await?)
}

/// Get the owner of the package - this will be the person who has contributed most.
/// Returns a tuple of their username and their github user ID.
pub async fn get_github_owner(
    client: &Octocrab,
    owner: impl AsRef<str>,
    repo: impl AsRef<str>,
) -> Result<(String, u64)> {
    let owner = owner.as_ref();
    let repo = repo.as_ref();

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
    owner: impl AsRef<str>,
    repo: impl AsRef<str>,
    branch: &Option<impl AsRef<str>>,
    dir: &Option<impl AsRef<str>>,
) -> Result<Option<LegacyManifest>> {
    let owner = owner.as_ref();
    let repo = repo.as_ref();

    let branch = match branch {
        Some(it) => it.as_ref().into(),

        None => {
            let repo_info = client.repos(owner, repo).get().await?;

            repo_info.default_branch.unwrap_or("main".into())
        }
    };

    let path = match dir {
        Some(it) => format!("{}/.kjspkg", it.as_ref()),
        None => ".kjspkg".into(),
    };

    match client
        .repos(owner, repo)
        .raw_file(branch.clone(), path)
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
pub async fn get_readme(
    client: &Octocrab,
    owner: impl AsRef<str>,
    repo: impl AsRef<str>,
    branch: &Option<impl AsRef<str>>,
) -> Result<String> {
    let owner = owner.as_ref();
    let repo = repo.as_ref();

    let branch = match branch {
        Some(it) => it.as_ref().into(),

        None => {
            let repo_info = client.repos(owner, repo).get().await?;

            repo_info.default_branch.unwrap_or("main".into())
        }
    };

    Ok(client
        .repos(owner, repo)
        .get_readme()
        .r#ref(branch)
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
    owner: impl AsRef<str>,
    repo: impl AsRef<str>,
    branch: &Option<impl AsRef<str>>,
    dir: &Option<impl AsRef<str>>,
) -> Result<(String, Vec<u8>)> {
    let owner = owner.as_ref();
    let repo = repo.as_ref();

    let branch = match branch {
        Some(it) => it.as_ref().into(),

        None => {
            let repo_info = client.repos(owner, repo).get().await?;

            repo_info.default_branch.unwrap_or("main".into())
        }
    };

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

    let tarball = client
        .repos(owner, repo)
        .download_tarball(branch)
        .await?
        .into_body()
        .collect()
        .await?
        .to_bytes()
        .to_vec();

    let mut archive = Archive::new(GzDecoder::new(Cursor::new(tarball.clone())));
    let mut entry_iter = archive.entries()?;

    let dir_suffix = match dir {
        Some(it) => format!(
            "{}/",
            it.as_ref()
                .strip_prefix('/')
                .unwrap_or(it.as_ref())
                .strip_suffix('/')
                .unwrap_or(it.as_ref())
        ),

        None => String::new(),
    };

    let root_path = format!("{}-{}-{}/{}", owner, repo, &commit[0..7], dir_suffix);
    let mut gzip = GzEncoder::new(Vec::new(), Compression::default());

    {
        let mut out_archive = TarBuilder::new(&mut gzip);

        while let Some(Ok(mut entry)) = entry_iter.next() {
            let path = entry.path()?.into_owned();

            if path.starts_with(&root_path) {
                let path = path.strip_prefix(&root_path)?;

                if path.as_os_str() == "" {
                    continue;
                }

                let size = entry.size();
                let mut bytes = Vec::new();
                let mut header = entry.header().clone();

                entry.read_to_end(&mut bytes)?;
                header.set_path(path)?;
                header.set_size(size);
                header.set_cksum();
                out_archive.append(&header, Cursor::new(bytes))?;
            }
        }
    }

    let tarball = gzip.finish()?;

    Ok((commit, tarball))
}
