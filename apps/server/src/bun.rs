use crate::Result;
use std::{
    env::consts::{ARCH, EXE_SUFFIX, OS},
    fs,
    io::{Cursor, Read, Write},
    path::PathBuf,
};
use tempfile::NamedTempFile;
use zip::ZipArchive;

pub const BUN_REPO: &str = "https://github.com/oven-sh/bun";
pub const BUN_VERSION: &str = "bun-v1.1.36";

pub fn get_base_url() -> String {
    format!("{}/releases/download/{}", BUN_REPO, BUN_VERSION)
}

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

pub fn get_bun_os() -> &'static str {
    match OS {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => panic!("Bun doesn't support your OS ({})!", OS),
    }
}

pub fn get_bun_platform() -> String {
    format!("bun-{}-{}", get_bun_os(), get_bun_arch())
}

pub fn get_bun_zip_url() -> String {
    format!("{}/{}.zip", get_base_url(), get_bun_platform())
}

pub fn get_bun_exe_path_in_zip() -> String {
    format!("{}/bun{}", get_bun_platform(), EXE_SUFFIX)
}

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
