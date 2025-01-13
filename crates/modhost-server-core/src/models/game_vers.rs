//! Models relating to game versions.

/// A game version.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct GameVersion {
    /// The version's ID (or version number).
    pub id: String,

    /// Whether this version is a beta version.
    pub beta: bool,
}
