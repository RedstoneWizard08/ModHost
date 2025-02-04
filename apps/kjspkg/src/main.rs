use axum::body::Bytes;
use flate2::read::GzDecoder;
use modhost::{GameVersion, Result};
use modhost_db::ProjectManifest;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use tar::Archive;

pub const PISTON_META_ENDPOINT: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

quickhost::quickhost! {
    versions = [crate::get_minecraft_versions().await?];
    loaders = [modhost::loaders!["Forge", "Fabric", "Quilt", "NeoForge"]];
    verifier = [crate::verify_project];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PistonManifest {
    pub latest: MinecraftLatestVersions,
    pub versions: Vec<MinecraftVersionInfo>,
}

pub async fn get_minecraft_versions() -> Result<Vec<GameVersion>> {
    let manifest: PistonManifest = reqwest::get(PISTON_META_ENDPOINT).await?.json().await?;

    Ok(manifest
        .versions
        .iter()
        .map(|v| GameVersion {
            id: v.id.clone(),
            beta: v.kind != "release",
        })
        .collect())
}

pub fn verify_project(bytes: Bytes) -> bool {
    let mut data = GzDecoder::new(Cursor::new(bytes));
    let mut gunzip = Vec::new();

    if let Err(_) = data.read_to_end(&mut gunzip) {
        return false;
    }

    let mut archive = Archive::new(Cursor::new(gunzip));

    if let Ok(entries) = archive.entries() {
        for entry in entries {
            if let Ok(mut entry) = entry {
                if entry.path().unwrap_or_default().to_str().unwrap() == "kjspkg.json" {
                    let mut data = String::new();

                    if let Err(_) = entry.read_to_string(&mut data) {
                        return false;
                    }

                    if let Err(_) = serde_json::from_str::<ProjectManifest>(&data) {
                        return false;
                    }

                    return true;
                }
            }
        }
    }

    false
}
