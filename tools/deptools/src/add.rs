use crate::util::{get_crate_map, get_root_cargo_toml};
use anyhow::{Result, anyhow};
use crates_io_api::AsyncClient;
use std::{fs, time::Duration};
use toml_edit::{DocumentMut, InlineTable, array, value};

pub async fn add_crate(
    name: String,
    ver: Option<String>,
    features: Vec<String>,
    pkg: String,
) -> Result<()> {
    let (toml_path, mut toml) = get_root_cargo_toml()?;
    let crate_map = get_crate_map()?;

    if !crate_map.contains_key(&pkg) {
        return Err(anyhow!("Could not find path to package: {pkg}"));
    }

    let pkg_dir = crate_map.get(&pkg).unwrap();
    let pkg_path = pkg_dir.join("Cargo.toml");
    let mut pkg_toml = fs::read_to_string(&pkg_path)?.parse::<DocumentMut>()?;
    let client = AsyncClient::new("ModHost/DepTools v0.1.0", Duration::from_millis(1))?;
    let item_min = client.get_crate(&name).await?;
    let item_name = item_min.crate_data.name;
    let item = client.full_crate(&name, false).await?;
    let web_ver = item.versions.first().unwrap();
    let ver = ver.unwrap_or(web_ver.num.clone());

    let ver = item_min
        .versions
        .iter()
        .find(|v| v.num == ver)
        .ok_or(anyhow!(
            "Could not find version {ver} of crate {}!",
            item_name
        ))?;

    let mut unknown_feats = Vec::new();

    for feat in &features {
        if !ver.features.contains_key(feat) {
            unknown_feats.push(feat.clone());
        }
    }

    if !unknown_feats.is_empty() {
        return Err(anyhow!(
            "Missing features in crate: {}",
            unknown_feats.join(", ")
        ));
    }

    toml["workspace"]["dependencies"][&item_name]["version"] = value(ver.num.clone());
    toml["workspace"]["dependencies"][&item_name]["features"] = array();

    for (idx, feat) in features.into_iter().enumerate() {
        toml["workspace"]["dependencies"][&item_name]["features"][idx] = value(feat);
    }

    if let Some(table) = toml["workspace"]["dependencies"][&item_name].as_inline_table_mut() {
        table.fmt()
    }

    pkg_toml["dependencies"][&item_name] = value(InlineTable::new());
    pkg_toml["dependencies"][&item_name]["workspace"] = value(true);

    if let Some(table) = pkg_toml["dependencies"].as_inline_table_mut() {
        table.fmt()
    }

    fs::write(toml_path, toml.to_string())?;
    fs::write(pkg_path, pkg_toml.to_string())?;

    Ok(())
}
