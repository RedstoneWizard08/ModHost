use anyhow::{Result, anyhow};
use glob::glob;
use std::{
    collections::HashMap,
    env::{current_dir, set_current_dir},
    fs,
    path::PathBuf,
};
use toml_edit::DocumentMut;

pub fn get_root_cargo_toml() -> Result<(PathBuf, DocumentMut)> {
    let mut dir = current_dir().ok();

    while let Some(cur) = &dir {
        let toml_path = cur.join("Cargo.toml");

        if !toml_path.exists() {
            if cur.to_str().unwrap() == "/" {
                dir = None;
            } else {
                dir = cur.parent().map(|v| v.to_path_buf());
            }

            continue;
        }

        let content = fs::read_to_string(toml_path)?;
        let doc = content.parse::<DocumentMut>()?;

        if doc.contains_key("workspace") {
            break;
        }

        if cur.to_str().unwrap() == "/" {
            dir = None;
            break;
        }

        dir = cur.parent().map(|v| v.to_path_buf());
    }

    let root = dir.ok_or(anyhow!("Could not find workspace root!"))?;
    let toml_path = root.join("Cargo.toml");
    let content = fs::read_to_string(&toml_path)?;
    let toml = content.parse::<DocumentMut>()?;

    Ok((toml_path, toml))
}

pub fn get_crate_map() -> Result<HashMap<String, PathBuf>> {
    let (toml_path, toml) = get_root_cargo_toml()?;
    let root_dir = toml_path
        .parent()
        .ok_or(anyhow!("Could not find workspace root!"))?
        .to_path_buf();

    let members = toml["workspace"]["members"]
        .as_array()
        .ok_or(anyhow!("'workspace.members' was not an array!"))?;

    let members = members
        .into_iter()
        .filter_map(|v| v.as_str().map(|v| v.to_string()))
        .collect::<Vec<_>>();

    set_current_dir(root_dir)?;

    let crate_dirs = members
        .into_iter()
        .filter_map(|v| {
            glob(&v)
                .map(|v| v.filter_map(|v| v.ok()).collect::<Vec<_>>())
                .ok()
        })
        .flatten()
        .filter(|v| v.join("Cargo.toml").exists())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    for dir in crate_dirs {
        let path = dir.join("Cargo.toml");
        let toml = fs::read_to_string(&path)?.parse::<DocumentMut>()?;

        if !toml.contains_key("package") {
            continue;
        }

        let name = toml["package"]["name"]
            .as_str()
            .ok_or(anyhow!(
                "Could not convert 'package.name' to a string in {}!",
                path.to_string_lossy()
            ))?
            .to_string();

        map.insert(name, dir);
    }

    Ok(map)
}
