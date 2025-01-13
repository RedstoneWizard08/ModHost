//! Models relating to tags.

/// A tag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct Tag {
    /// The ID of the tag.
    pub id: String,

    /// The tag's display name.
    pub name: String,

    /// The tag's Iconify icon name.
    pub icon: String,
}
