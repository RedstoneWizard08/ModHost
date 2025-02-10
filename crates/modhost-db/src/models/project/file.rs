//! Project file-related models.

use crate::{ProjectVersion, schema::version_files};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

/// A project file.
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
#[diesel(table_name = version_files)]
#[diesel(belongs_to(ProjectVersion, foreign_key = version_id))]
#[diesel(check_for_backend(Pg))]
pub struct ProjectFile {
    /// The version file ID.
    pub id: i32,

    /// The file name.
    pub file_name: String,

    /// The SHA-1 hash of the version file.
    pub sha1: String,

    /// The ID to get the file from S3.
    pub s3_id: String,

    /// An ID of the project version this file belongs to.
    pub version_id: i32,

    /// The size of the file in bytes.
    pub size: i64,

    /// The date this file was uploaded.
    pub uploaded_at: NaiveDateTime,
}

/// The initial data for creating a new project file in the database.
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
#[diesel(table_name = version_files)]
#[diesel(belongs_to(ProjectVersion, foreign_key = version_id))]
#[diesel(check_for_backend(Pg))]
pub struct NewProjectFile {
    /// The file name.
    pub file_name: String,

    /// The SHA-1 hash of the version file.
    pub sha1: String,

    /// The ID to get the file from S3.
    pub s3_id: String,

    /// An ID of the project version this file belongs to.
    pub version_id: i32,

    /// The size of the file in bytes.
    pub size: i64,
}
