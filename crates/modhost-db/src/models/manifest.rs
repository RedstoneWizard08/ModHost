//! The project manifest model.

/// A manifest for a project.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct ProjectManifest {
    /// The project name
    pub name: String,

    /// The project authors
    pub authors: Vec<String>,

    /// The project version
    pub version: String,

    /// The project description
    pub description: String,

    /// The loaders this project works on
    pub loaders: Vec<String>,

    /// The game versions this project works on
    pub game_versions: Vec<String>,

    /// This project's dependencies
    pub dependencies: Vec<String>,

    /// This project's incompatibilities
    pub incompatibilities: Vec<String>,
}
