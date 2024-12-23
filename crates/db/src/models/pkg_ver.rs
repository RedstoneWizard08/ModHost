//! Package version-related models.

use crate::{
    models::pkg::Package,
    schema::{package_version_refs, package_versions},
};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

/// A package version.
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
#[diesel(table_name = package_versions)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct PackageVersion {
    /// The package version ID.
    pub id: i32,

    /// The package ID.
    pub package: i32,

    /// The display name of the version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// The file ID in the bucket.
    #[serde(skip)]
    pub file_id: String,

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

/// The initial data for creating a new package version in the database.
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
#[diesel(table_name = package_versions)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct NewPackageVersion {
    /// The package ID.
    pub package: i32,

    /// The display name of the version.
    pub name: String,

    /// The version number.
    pub version_number: String,

    /// The file ID in the bucket.
    pub file_id: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<Option<String>>,

    /// A list of game versions this works on.
    pub game_versions: Vec<Option<String>>,

    /// The number of downloads this version has.
    pub downloads: i32,
}

/// A reference to a package version.
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
#[diesel(table_name = package_version_refs)]
#[diesel(belongs_to(PackageVersion, foreign_key = value))]
#[diesel(check_for_backend(Pg))]
pub struct PackageVersionRef {
    /// The package version ID.
    pub value: i32,
}

/// The initial data for creating a new package version.
/// This should be formatted as "multipart/form-data".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse)]
pub struct PackageVersionInit {
    /// The name of the package.
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

    /// The file content.
    pub file: Vec<u8>,
}
