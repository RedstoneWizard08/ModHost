//! Options for badge generation.

/// The badge style.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Default,
    ToSchema,
    ToResponse,
)]
pub enum BadgeStyle {
    /// The plastic style.
    #[default]
    #[serde(rename = "plastic")]
    Plastic,

    /// The flat style.
    #[serde(rename = "flat")]
    Flat,

    /// The flat square style.
    #[serde(rename = "flat-square")]
    FlatSquare,

    /// The "for the badge" style.
    #[serde(rename = "for-the-badge")]
    ForTheBadge,

    /// The "social" style.
    #[serde(rename = "social")]
    Social,
}

/// Options for badge generation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
    IntoParams,
)]
pub struct BadgeOptions {
    /// The label text.
    pub label_text: String,

    /// The label background color. Can be any valid CSS color code or name.
    pub label_color: Option<String>,

    /// The label's title.
    pub label_title: Option<String>,

    /// The label's link when clicked.
    pub label_link: Option<String>,

    /// The message text.
    pub msg_text: String,

    /// The message background color. Can be any valid CSS color code or name.
    pub msg_color: Option<String>,

    /// The message's title.
    pub msg_title: Option<String>,

    /// The message's link when clicked.
    pub msg_link: Option<String>,

    /// The link of the badge when clicked.
    pub link: Option<String>,

    /// The badge's title.
    pub title: Option<String>,

    /// The badge style.
    pub style: Option<BadgeStyle>,

    /// Must be a logo ID on [SimpleIcons](https://simpleicons.org).
    pub logo: Option<String>,
}
