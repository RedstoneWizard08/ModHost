//! The UI config.

use std::collections::HashMap;

/// The UI config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// The app name/title.
    /// Defaults to `"ModHost"`
    pub app: String,

    /// The app tagline.
    /// Defaults to `"Your home for game mods"`
    pub tagline: String,

    /// Whether to show the beta badge on the website.
    /// Defaults to `true`
    pub show_beta: bool,

    /// The type of projects to use in translations (Mods or Packages).
    /// Defaults to [`ProjectKind::Mods`]
    pub project_kind: ProjectKind,

    /// The default theme to apply if the user hasn't changed it.
    /// Defaults to `"modhost"`
    pub default_theme: String,

    /// The file extensions the frontend allows you to upload.
    /// Defaults to `[".pak", ".jar", ".zip", ".tgz", ".tar.gz"]`
    pub project_file_formats: Vec<String>,

    /// The name for game beta versions (Beta or Snapshot).
    /// Defaults to [`BetaName::Beta`]
    pub game_beta_name: BetaName,

    /// The URL or file path to get the logo PNG from.
    /// If this is `"default"` it will use ModHost's logo.
    /// Defaults to `"default"`
    pub favicon_png: String,

    /// The URL or file path to get the favicon ICO from.
    /// If this is `"default"` it will use ModHost's favicon.
    /// Defaults to `"default"`
    pub favicon_ico: String,

    /// The CSS theme color for meta tags.
    /// Defaults to `"#068099"`
    pub theme_color: String,

    /// The base badge color, used in the first part of the badge.
    /// Defaults to `"#191d28"`.
    pub badge_base: String,

    /// The secondary badge color, used in the second part of the badge.
    /// Defaults to `"#068099"`.
    pub badge_secondary: String,
}

/// The type of project to use in translations (Mods or Packages).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum ProjectKind {
    /// Will show "mod" instead of "packages", etc.
    #[default]
    Mods,

    /// Will show "package" instead of "mod", etc.
    Packages,
}

/// The name for game beta versions (Beta or Snapshot).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum BetaName {
    /// Will show "beta" instead of "snapshot", etc.
    #[default]
    Beta,

    /// Will show "snapshot" instead of "beta", etc.
    Snapshot,
}

impl ProjectKind {
    /// Get the string form for translations.
    pub fn stringify(&self) -> &'static str {
        match self {
            Self::Mods => "mods",
            Self::Packages => "packages",
        }
    }
}

impl BetaName {
    /// Get the string form for translations.
    pub fn stringify(&self) -> &'static str {
        match self {
            Self::Beta => "beta",
            Self::Snapshot => "snapshot",
        }
    }
}

impl UIConfig {
    /// Get a map of environment variables for the UI.
    pub fn env(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("PUBLIC_APP".into(), self.app.clone());
        map.insert("PUBLIC_TAGLINE".into(), self.tagline.clone());
        map.insert("PUBLIC_SHOW_BETA".into(), self.show_beta.to_string());

        map.insert(
            "PUBLIC_PKG_TYPE".into(),
            self.project_kind.stringify().into(),
        );

        map.insert("PUBLIC_DEFAULT_THEME".into(), self.default_theme.clone());

        map.insert(
            "PUBLIC_PKG_FILE_FORMATS".into(),
            self.project_file_formats.join(","),
        );

        map.insert(
            "PUBLIC_GAME_BETA_NAME".into(),
            self.game_beta_name.stringify().into(),
        );

        map.insert("PUBLIC_THEME_COLOR".into(), self.theme_color.clone());

        map
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            app: "ModHost".into(),
            tagline: "Your home for game mods".into(),
            show_beta: true,
            project_kind: ProjectKind::Mods,
            default_theme: "modhost".into(),
            project_file_formats: vec![
                ".pak".into(),
                ".jar".into(),
                ".zip".into(),
                ".tgz".into(),
                ".tar.gz".into(),
            ],
            game_beta_name: BetaName::Beta,
            favicon_ico: "default".into(),
            favicon_png: "default".into(),
            theme_color: "#068099".into(),
            badge_base: "#191d28".into(),
            badge_secondary: "#068099".into(),
        }
    }
}
