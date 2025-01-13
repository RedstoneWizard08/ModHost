//! Utilities for downloading & extracting [Bun](https://bun.sh).

use crate::Result;
use std::{
    env::consts::{ARCH, EXE_SUFFIX, OS},
    fs,
    io::{Cursor, Read, Write},
    path::PathBuf,
};
use tempfile::NamedTempFile;
use zip::ZipArchive;

/// The GitHub repo to get the Bun executable from.
pub const BUN_REPO: &str = "https://github.com/oven-sh/bun";

/// The Bun version to download.
pub const BUN_VERSION: &str = "bun-v1.1.36";

/// Get the base release URL for the download.
pub fn get_base_url() -> String {
    format!("{}/releases/download/{}", BUN_REPO, BUN_VERSION)
}

/// Get the target architecture for Bun.
pub fn get_bun_arch() -> &'static str {
    match ARCH {
        "x86_64" => "x64",
        "aarch64" => match OS {
            "windows" => panic!("Bun doesn't have a release for windows/aarch64!"),
            _ => "aarch64",
        },
        _ => panic!("Bun doesn't have a release for your arch ({})!", ARCH),
    }
}

/// Get the correct OS name for Bun.
pub fn get_bun_os() -> &'static str {
    match OS {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => panic!("Bun doesn't support your OS ({})!", OS),
    }
}

/// Get the Bun platform name.
pub fn get_bun_platform() -> String {
    format!("bun-{}-{}", get_bun_os(), get_bun_arch())
}

/// Get the URL to download Bun from.
pub fn get_bun_zip_url() -> String {
    format!("{}/{}.zip", get_base_url(), get_bun_platform())
}

/// Get the path to the Bun executable in the archive.
pub fn get_bun_exe_path_in_zip() -> String {
    format!("{}/bun{}", get_bun_platform(), EXE_SUFFIX)
}

/// Download & extract the Bun executable, returning the path to it.
pub async fn get_bun_exe() -> Result<PathBuf> {
    info!("Downloading Bun...");

    let data = reqwest::get(get_bun_zip_url()).await?.bytes().await?;

    info!("Extracting Bun...");

    let mut zip = ZipArchive::new(Cursor::new(data))?;
    let mut file = zip.by_name(&get_bun_exe_path_in_zip())?;
    let (mut temp, path) = NamedTempFile::new()?.keep()?;
    let mut data = Vec::new();

    file.read_to_end(&mut data)?;
    temp.write_all(data.as_slice())?;

    if OS != "windows" {
        use std::os::unix::fs::PermissionsExt;

        info!("Fixing permissions...");

        let mut perms = fs::metadata(&path)?.permissions();

        perms.set_mode(0o777);
        temp.set_permissions(perms)?;
    }

    Ok(path)
}
