use std::{collections::HashMap, env};

use crate::{
    data::get_version_str,
    fetcher::{get_github_owner, get_manifest, get_package_tarball, get_packages_map, get_readme},
};
use anyhow::Result;
use diesel::{insert_into, SelectableHelper};
use diesel_async::RunQueryDsl;
use indicatif::ProgressIterator;
use modhost::init_logger;
use modhost_config::get_config;
use modhost_db::{
    create_connection, project_authors, project_versions, projects, run_migrations, users,
    version_files, NewProject, NewProjectFile, NewProjectVersion, NewUser, Project, ProjectAuthor,
    ProjectVersion, ProjectVisibility, User,
};
use octocrab::Octocrab;
use sha1::{Digest, Sha1};
use tracing::level_filters::LevelFilter;

pub async fn run() -> Result<()> {
    let _ = dotenvy::dotenv();

    init_logger(LevelFilter::INFO);

    let token = env::var("MIGRATOR_TOKEN")
        .expect("Could not find a GitHub token! Is the MIGRATOR_TOKEN environment variable unset?");

    let config = get_config()?;
    let pool = create_connection(Some(config.postgres.uri())).await?;

    run_migrations(&pool).await?;

    let mut conn = pool.get().await?;
    let pkgs = config.storage.projects()?;
    // let imgs = config.storage.gallery()?;
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let packages = get_packages_map().await?;
    let mut added_users = HashMap::new();

    for (id, repo) in packages.into_iter().progress() {
        let mut split = repo.split(&['/'][..]);
        let owner = split.next().unwrap();
        let mut repo = split.next().unwrap();
        let mut branch: Option<String> = None;
        let mut dir: Option<String> = None;

        if repo.contains('$') {
            let (repo_split, dir_split) = repo.split_once('$').unwrap();

            repo = repo_split.into();
            dir = Some(dir_split.into());
        }

        if repo.contains('@') {
            let (repo_split, branch_split) = repo.split_once('@').unwrap();

            repo = repo_split.into();
            branch = Some(branch_split.into());
        }

        if let Some(dir_s) = dir.clone() {
            if dir_s.contains('@') && branch.is_none() {
                let (dir_split, branch_split) = dir_s.split_once('@').unwrap();

                dir = Some(dir_split.into());
                branch = Some(branch_split.into());
            }
        }

        // let branch_or_default = branch.clone().unwrap_or("[default branch]".into());

        // info!("Uploading {owner}/{repo} (path {dir:?}) at branch {branch_or_default:?}");

        let (author_name, author_id) = get_github_owner(&octocrab, owner, repo).await?;
        let manifest = get_manifest(&octocrab, owner, repo, &branch, &dir).await?;

        if let Some(manifest) = manifest {
            let readme = get_readme(&octocrab, owner, repo, &branch)
                .await
                .unwrap_or("During migration from the old KJSPKG, no README was found.".into());

            let (commit, tarball) =
                get_package_tarball(&octocrab, owner, repo, &branch, &dir).await?;

            if !added_users.contains_key(&author_id) {
                let user = NewUser {
                    github_id: author_id as i32,
                    username: author_name,
                };

                let user: User = insert_into(users::table)
                    .values(user)
                    .returning(User::as_returning())
                    .get_result(&mut conn)
                    .await?;

                added_users.insert(author_id, user.id);
            }

            let user_id = *added_users.get(&author_id).unwrap();

            let project = NewProject {
                slug: id.clone(),
                name: id.clone(),
                description: manifest.description,
                issues: Some(format!("https://github.com/{}/issues", repo)),
                source: Some(format!("https://github.com/{}", repo)),
                wiki: Some(format!("https://github.com/{}/wiki", repo)),
                license: None,
                readme,
                tags: Vec::new(),
                visibility: ProjectVisibility::Public,
            };

            let project: Project = insert_into(projects::table)
                .values(project)
                .returning(Project::as_returning())
                .get_result(&mut conn)
                .await?;

            let author = ProjectAuthor {
                project: project.id,
                user_id,
            };

            insert_into(project_authors::table)
                .values(author)
                .execute(&mut conn)
                .await?;

            let mut hasher = Sha1::new();

            hasher.update(&tarball);

            let file_id = format!("{:x}", hasher.finalize());

            pkgs.put_object(format!("/{}", &file_id), &tarball).await?;

            let version = NewProjectVersion {
                name: commit.clone(),
                version_number: format!("0.0.0+{}", commit),
                changelog: Some("Migrated from the old KJSPKG.".into()),
                downloads: 0,
                loaders: manifest.modloaders.into_iter().map(|v| Some(v)).collect(),
                project: project.id,
                game_versions: manifest
                    .versions
                    .into_iter()
                    .filter_map(|v| get_version_str(v))
                    .map(|v| Some(v))
                    .collect(),
            };

            let version: ProjectVersion = insert_into(project_versions::table)
                .values(version)
                .returning(ProjectVersion::as_returning())
                .get_result(&mut conn)
                .await?;

            let file = NewProjectFile {
                version_id: version.id,
                file_name: format!("{}-{}.tar.gz", id, commit),
                s3_id: file_id.clone(),
                sha1: file_id,
                size: tarball.len() as i64,
            };

            insert_into(version_files::table)
                .values(file)
                .execute(&mut conn)
                .await?;
        }
    }

    Ok(())
}
