pub mod models;
pub mod tags;
pub mod vers;

use once_cell::sync::Lazy;
use regex::Regex;

pub const PATCH_NOTES_URL: &str = "https://astroneer.wiki.gg/wiki/Special:Export/Patch_Notes";
pub const VER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^((\d+)\.)+\d+$").unwrap());
