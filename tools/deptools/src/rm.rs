use crate::util::{get_crate_map, get_root_cargo_toml};
use anyhow::{Result, anyhow};
use std::fs;
use toml_edit::DocumentMut;

pub fn remove_crate(name: String, pkg: String) -> Result<()> {
    let (toml_path, mut toml) = get_root_cargo_toml()?;
    let crate_map = get_crate_map()?;

    if !crate_map.contains_key(&pkg) {
        return Err(anyhow!("Could not find path to package: {pkg}"));
    }

    let pkg_dir = crate_map.get(&pkg).unwrap();
    let pkg_path = pkg_dir.join("Cargo.toml");
    let mut pkg_toml = fs::read_to_string(&pkg_path)?.parse::<DocumentMut>()?;

    if !pkg_toml["dependencies"]
        .as_table()
        .unwrap()
        .contains_key(&name)
    {
        return Err(anyhow!("Dependency was not present in crate!"));
    }

    pkg_toml["dependencies"]
        .as_table_mut()
        .unwrap()
        .remove(&name);

    if let Some(table) = pkg_toml["dependencies"].as_inline_table_mut() {
        table.fmt()
    }

    if !toml["workspace"]["dependencies"]
        .as_table()
        .unwrap()
        .contains_key(&name)
    {
        return Err(anyhow!("Dependency was not present in workspace root!"));
    }

    let tomls = crate_map.values().filter_map(|v| {
        fs::read_to_string(v.join("Cargo.toml"))
            .ok()?
            .parse::<DocumentMut>()
            .ok()
    });

    if !tomls
        .filter_map(|v| v["dependencies"].as_table().cloned())
        .any(|v| v.contains_key(&name))
    {
        toml["workspace"]["dependencies"]
            .as_table_mut()
            .unwrap()
            .remove(&name);
    }

    if let Some(table) = toml["workspace"]["dependencies"].as_inline_table_mut() {
        table.fmt()
    }

    fs::write(toml_path, toml.to_string())?;
    fs::write(pkg_path, pkg_toml.to_string())?;

    Ok(())
}
