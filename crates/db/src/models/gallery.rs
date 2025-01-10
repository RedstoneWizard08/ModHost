//! Project gallery-related models.

use crate::{gallery_images, Project};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

/// A gallery image.
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
#[diesel(table_name = gallery_images)]
#[diesel(belongs_to(Project, foreign_key = project))]
#[diesel(check_for_backend(Pg))]
pub struct GalleryImage {
    /// The gallery image ID.
    pub id: i32,

    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// This image's ID in S3.
    pub s3_id: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,

    /// The date this version was created.
    pub created_at: NaiveDateTime,

    /// The date this version was last updated.
    pub updated_at: NaiveDateTime,
}

/// A gallery image, modified for public consumption (i.e. REST endpoints).
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct PublicGalleryImage {
    /// The gallery image ID.
    pub id: i32,

    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// A URL to access this image with.
    pub url: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,

    /// The date this version was created.
    pub created_at: NaiveDateTime,

    /// The date this version was last updated.
    pub updated_at: NaiveDateTime,
}

/// A gallery image for insertion.
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
    Associations,
    Insertable,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = gallery_images)]
#[diesel(belongs_to(Project, foreign_key = project))]
#[diesel(check_for_backend(Pg))]
pub struct NewGalleryImage {
    /// The project ID.
    pub project: i32,

    /// The display name of the version.
    pub name: String,

    /// This image's ID in S3.
    pub s3_id: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,
}
