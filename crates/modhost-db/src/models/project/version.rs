//! Project version-related models.

use crate::{
    schema::{project_version_refs, project_versions},
    Project,
};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

use super::ProjectFile;

/// A project version.
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
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = project_versions)]
#[diesel(belongs_to(Project, foreign_key = project))]
#[diesel(check_for_backend(Pg))]
pub struct ProjectVersion {
    /// The project version ID.
    pub id: i32,

    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<Option<String>>,

    /// A list of game versions this works on.
    pub game_versions: Vec<Option<String>>,

    /// The date this version was created.
    pub created_at: NaiveDateTime,

    /// The date this version was last updated.
    pub updated_at: NaiveDateTime,

    /// The number of downloads this version has.
    pub downloads: i32,
}

/// The initial data for creating a new project version in the database.
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
    Queryable,
    Selectable,
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = project_versions)]
#[diesel(belongs_to(Project, foreign_key = project))]
#[diesel(check_for_backend(Pg))]
pub struct NewProjectVersion {
    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<Option<String>>,

    /// A list of game versions this works on.
    pub game_versions: Vec<Option<String>>,

    /// The number of downloads this version has.
    pub downloads: i32,
}

/// A reference to a project version.
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
    Queryable,
    Selectable,
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = project_version_refs)]
#[diesel(belongs_to(ProjectVersion, foreign_key = value))]
#[diesel(check_for_backend(Pg))]
pub struct ProjectVersionRef {
    /// The project version ID.
    pub value: i32,
}

/// The initial data for creating a new project version.
/// This should be formatted as "multipart/form-data".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse)]
pub struct ProjectVersionInit {
    /// The name of the project version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// An optional changelog.
    pub changelog: Option<String>,

    /// A list of loaders this version works on.
    /// This should be a comma-separated list in the request.
    pub loaders: String,

    /// A list of game versions this works on.
    /// This should be a comma-separated list in the request.
    pub game_versions: String,

    /// The file name.
    pub file_name: String,

    /// The file content.
    #[schema(content_media_type = "application/octet-stream")]
    pub file: Vec<u8>,
}

/// A project version with a list of its files.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct ProjectVersionData {
    /// The project version ID.
    pub id: i32,

    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<Option<String>>,

    /// A list of game versions this works on.
    pub game_versions: Vec<Option<String>>,

    /// The date this version was created.
    pub created_at: NaiveDateTime,

    /// The date this version was last updated.
    pub updated_at: NaiveDateTime,

    /// The number of downloads this version has.
    pub downloads: i32,

    /// This version's files.
    pub files: Vec<ProjectFile>,
}

impl ProjectVersion {
    /// Transform this into [`ProjectVersionData`] with a list of [`ProjectFile`]s.
    pub fn with_files(self, files: Vec<ProjectFile>) -> ProjectVersionData {
        ProjectVersionData {
            id: self.id,
            project: self.project,
            name: self.name,
            version_number: self.version_number,
            changelog: self.changelog,
            loaders: self.loaders,
            game_versions: self.game_versions,
            created_at: self.created_at,
            updated_at: self.updated_at,
            downloads: self.downloads,
            files,
        }
    }
}
