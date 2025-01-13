pub fn get_version_str(id: u16) -> Option<String> {
    match id {
        2 => Some("1.12".into()),
        6 => Some("1.16".into()),
        8 => Some("1.18".into()),
        9 => Some("1.19".into()),
        10 => Some("1.20".into()),
        11 => Some("1.21".into()),
        _ => None,
    }
}
