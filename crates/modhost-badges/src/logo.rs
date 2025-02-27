//! SimpleIcons logo resolver.

/// Check if a logo ID is valid with SimpleIcons.
pub async fn resolve_simpleicons_logo(id: impl AsRef<str>) -> Option<String> {
    let url = format!("https://simpleicons.org/icons/{}.svg", id.as_ref());
    let status = reqwest::get(&url).await.ok()?.status().as_u16();

    if (200..400).contains(&status) {
        Some(url)
    } else {
        None
    }
}
