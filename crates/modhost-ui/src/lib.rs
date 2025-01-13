#![warn(missing_docs)]
//! ModHost's dynamic UI compiler.
//! This allows us to build the frontend when the server starts to allow
//! for customization of several variables.
//! The frontend's source code is embedded in this crate.

#[macro_use]
extern crate tracing;

#[cfg(not(debug_assertions))]
mod bun;

use modhost_config::AppConfig;
use modhost_core::Result;
use std::{fs, path::PathBuf};

/// The default favicon.ico bytes.
pub const DEFAULT_FAVICON_ICO: &[u8] = include_bytes!("./assets/modhost.ico");

/// The default favicon.png bytes.
pub const DEFAULT_FAVICON_PNG: &[u8] = include_bytes!("./assets/modhost.png");

/// The embedded source for the frontend.
/// Yes, the entire source code is embedded in the binary.
#[cfg(not(debug_assertions))]
pub const UI_SOURCE: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../ui");

/// Build the frontend.
/// In debug builds, this uses the config and a root path to the source
/// to write the favicon files.
/// In release builds, this will:
///  - Download & extract [Bun](https://bun.sh)
///  - Extract the UI source to a temp directory
///  - Write the favicon files
///  - Install dependencies
///  - Build the UI
/// In release builds, this will return a [`PathBuf`] containing the full
/// path to the built UI. In debug builds this will return `()`.
#[cfg(debug_assertions)]
pub async fn build_ui(config: &AppConfig, dir: &PathBuf) -> Result<()> {
    if config.ui.favicon_ico == "default" {
        info!("Downloading favicon.ico...");

        fs::write(dir.join("static/favicon.ico"), DEFAULT_FAVICON_ICO)?;
    } else {
        info!("Downloading favicon.ico...");

        let data = if config.ui.favicon_ico.starts_with("http") {
            reqwest::get(&config.ui.favicon_ico).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_ico)?.into()
        };

        fs::write(dir.join("static/favicon.ico"), data)?;
    }

    if config.ui.favicon_png == "default" {
        info!("Downloading favicon.png...");

        fs::write(dir.join("static/favicon.png"), DEFAULT_FAVICON_PNG)?;
    } else {
        info!("Downloading favicon.png...");

        let data = if config.ui.favicon_png.starts_with("http") {
            reqwest::get(&config.ui.favicon_png).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_png)?.into()
        };

        fs::write(dir.join("static/favicon.png"), data)?;
    }

    Ok(())
}

/// Build the frontend.
/// In debug builds, this uses the config and a root path to the source
/// to write the favicon files.
/// In release builds, this will:
///  - Download & extract [Bun](https://bun.sh)
///  - Extract the UI source to a temp directory
///  - Write the favicon files
///  - Install dependencies
///  - Build the UI
/// In release builds, this will return a [`PathBuf`] containing the full
/// path to the built UI. In debug builds this will return `()`.
#[cfg(not(debug_assertions))]
pub async fn build_ui(config: &AppConfig) -> Result<PathBuf> {
    use tempfile::TempDir;
    use tokio::process::Command;

    let bun_exe = crate::bun::get_bun_exe().await?;

    info!("Extracting UI...");

    let dir = TempDir::new()?.into_path();

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    info!("Extracting to: {:?}", dir);

    UI_SOURCE.extract(&dir)?;

    if config.ui.favicon_ico == "default" {
        info!("Downloading favicon.ico...");

        fs::write(dir.join("static/favicon.ico"), DEFAULT_FAVICON_ICO)?;
    } else {
        info!("Downloading favicon.ico...");

        let data = if config.ui.favicon_ico.starts_with("http") {
            reqwest::get(&config.ui.favicon_ico).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_ico)?.into()
        };

        fs::write(dir.join("static/favicon.ico"), data)?;
    }

    if config.ui.favicon_png == "default" {
        info!("Downloading favicon.png...");

        fs::write(dir.join("static/favicon.png"), DEFAULT_FAVICON_PNG)?;
    } else {
        info!("Downloading favicon.png...");

        let data = if config.ui.favicon_png.starts_with("http") {
            reqwest::get(&config.ui.favicon_png).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_png)?.into()
        };

        fs::write(dir.join("static/favicon.png"), data)?;
    }

    info!("Running `bun install`...");

    Command::new(&bun_exe)
        .arg("install")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Running `bun run sync`...");

    Command::new(&bun_exe)
        .arg("--bun")
        .arg("run")
        .arg("sync")
        .env("NODE_ENV", "production")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Running `bun run build`...");

    Command::new(&bun_exe)
        .arg("--bun")
        .arg("run")
        .arg("dist")
        .env("NODE_ENV", "production")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Successfully built the UI!");

    fs::remove_file(bun_exe)?;

    let dir_clone = dir.as_os_str().to_os_string();

    ctrlc::set_handler(move || {
        info!("Caught exit! Cleaning up...");

        fs::remove_dir_all(&dir_clone).unwrap();
        jsglue::abort::on_exit();
    })
    .unwrap();

    Ok(dir.join("build"))
}
