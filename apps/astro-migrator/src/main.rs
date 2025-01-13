use anyhow::Result;
use astro_migrator::models::{Mod, ModsDump};
use diesel::{insert_into, SelectableHelper};
use diesel_async::RunQueryDsl;
use indicatif::ProgressIterator;
use itertools::Itertools;
use modhost::init_logger;
use modhost_config::get_config;
use modhost_db::{create_connection, run_migrations, users, NewUser, User};
use modhost_search::MeilisearchService;
use ron::ser::PrettyConfig;
use std::{fs, path::PathBuf};
use tracing::level_filters::LevelFilter;

#[tokio::main]
pub async fn main() -> Result<()> {
    init_logger(LevelFilter::INFO);

    let config = get_config()?;
    let pool = create_connection(Some(config.postgres.uri())).await?;

    run_migrations(&pool).await?;

    let pkgs = config.storage.projects()?;
    let imgs = config.storage.gallery()?;

    let mods_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mods");
    let raw = fs::read_to_string(mods_dir.join("mods.json"))?;
    let tags_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tags.ron");
    let data = serde_json::from_str::<ModsDump>(&raw)?;
    let dump: Vec<Mod> = data.into();

    let user = NewUser {
        github_id: -1,
        username: "ModHost Migrator".into(),
    };

    let id = insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(&mut pool.get().await?)
        .await?
        .id;

    let mut tags = Vec::new();

    for item in dump.into_iter().progress() {
        let (pkg, _) = item
            .upload_all(id, &mut pool.get().await?, &pkgs, &imgs)
            .await?;

        tags.extend(pkg.tags);
    }

    let tags = tags
        .into_iter()
        .filter_map(|v| v)
        .sorted()
        .dedup()
        .collect_vec();

    let search = MeilisearchService::new(&config)?;

    search.index_projects(&mut pool.get().await?).await?;

    fs::write(
        tags_file,
        ron::ser::to_string_pretty(&tags, PrettyConfig::default())?,
    )?;

    println!("Known tags written to tags.ron");

    Ok(())
}
