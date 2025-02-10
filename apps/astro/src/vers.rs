use crate::{PATCH_NOTES_URL, VER_REGEX, models::MediaWiki};
use anyhow::Result;
use const_format::formatcp;
use itertools::Itertools;
use modhost::GameVersion;
use parse_wiki_text_2::{Configuration, Node, TableNode};
use reqwest::{Client, header::USER_AGENT};
use tracing::info;

pub const AMH_USER_AGENT: &str = formatcp!("AstroModHost/{}", env!("CARGO_PKG_VERSION"));

pub async fn get_astro_versions() -> Result<Vec<GameVersion>> {
    info!("Getting patch notes from wiki...");

    let client = Client::new();

    let res = client
        .get(PATCH_NOTES_URL)
        .header(USER_AGENT, AMH_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    info!("Parsing XML...");

    let txt = serde_xml_rs::from_str::<MediaWiki>(&res)?
        .page
        .revision
        .text;

    info!("Parsing MediaWiki dump...");

    let parsed = Configuration::default().parse(&txt)?;

    info!("Getting tables...");

    let tables = parsed
        .nodes
        .iter()
        .filter_map(|v| match v.clone() {
            Node::Table {
                attributes,
                captions,
                end,
                rows,
                start,
            } => Some(TableNode {
                attributes,
                captions,
                end,
                rows,
                start,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    info!("Getting cells...");

    let cells = tables
        .iter()
        .flat_map(|v| v.rows.iter().flat_map(|v| v.cells.clone()))
        .collect::<Vec<_>>();

    info!("Getting cells content...");

    let cell_texts = cells
        .iter()
        .flat_map(|v| {
            v.content.iter().filter_map(|v| match v {
                Node::Text {
                    end: _,
                    start: _,
                    value,
                } => Some(value),
                _ => None,
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    info!("Getting version numbers...");

    let nums = cell_texts
        .iter()
        .filter(|v| VER_REGEX.is_match(v))
        .cloned()
        .collect::<Vec<_>>();

    info!("Parsing version numbers...");

    let semvers = nums
        .iter()
        .map(|v| lenient_semver::parse(v).unwrap())
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    info!("Reformatting version numbers...");

    let fmt_semvers = semvers
        .iter()
        .map(|v| format!("{}.{}.{}.{}", v.major, v.minor, v.patch, v.build))
        .collect::<Vec<_>>();

    Ok(fmt_semvers
        .iter()
        .map(|v| GameVersion {
            id: v.clone(),
            beta: false,
        })
        .collect())
}
