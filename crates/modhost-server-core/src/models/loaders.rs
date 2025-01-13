//! Models relating to mod loaders.

/// A mod loader.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct ModLoader {
    /// The ID of the loader.
    pub id: String,

    /// The display name of the loader.
    pub name: String,
}
