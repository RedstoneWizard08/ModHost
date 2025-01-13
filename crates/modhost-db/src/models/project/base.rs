//! The project model itself.

use crate::{schema::projects, User};
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel_derive_enum::DbEnum;
use itertools::Itertools;

/// A project's visibility.
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
    ToSchema,
    ToResponse,
    DbEnum,
    Default,
)]
#[ExistingTypePath = "crate::schema::sql_types::Visibility"]
pub enum ProjectVisibility {
    /// The project is publicly visible.
    #[default]
    Public,

    /// The project is private.
    Private,

    /// The project is unlisted.
    /// It will not show up in search but can be accessed via ID or slug.
    Unlisted,
}

/// A project.
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct Project {
    /// The project's ID.
    pub id: i32,

    /// The project's name.
    pub name: String,

    /// The project's URL slug.
    pub slug: String,

    /// The project's README.
    pub readme: String,

    /// A short description of the project.
    pub description: String,

    /// The date the project was created.
    pub created_at: NaiveDateTime,

    /// The date the project was last updated.
    pub updated_at: NaiveDateTime,

    /// The amount of downloads a project has.
    pub downloads: i32,

    /// An optional link to the project's source code.
    pub source: Option<String>,

    /// An optional link to the project's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the project's wiki.
    pub wiki: Option<String>,

    /// The visibility of a project.
    pub visibility: ProjectVisibility,

    /// The license the project is under.
    pub license: Option<String>,

    /// A list of tags for this project.
    pub tags: Vec<Option<String>>,
}

/// A model for creating a new project.
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct NewProject {
    /// The project's URL slug.
    pub slug: String,

    /// The project's name.
    pub name: String,

    /// The project's README.
    pub readme: String,

    /// A short description of the project.
    pub description: String,

    /// An optional link to the project's source code.
    #[serde(default)]
    pub source: Option<String>,

    /// An optional link to the project's issue tracker.
    #[serde(default)]
    pub issues: Option<String>,

    /// An optional link to the project's wiki.
    #[serde(default)]
    pub wiki: Option<String>,

    /// The visibility of a project. Optional. Defaults to public.
    #[serde(default)]
    pub visibility: ProjectVisibility,

    /// The license the project is under.
    #[serde(default)]
    pub license: Option<String>,

    /// A list of tags for this project.
    #[serde(default)]
    pub tags: Vec<Option<String>>,
}

/// A project with additional data.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct ProjectData {
    /// The project's ID.
    pub id: i32,

    /// The project's name.
    pub name: String,

    /// The project's URL slug.
    pub slug: String,

    /// The project's README.
    pub readme: String,

    /// A short description of the project.
    pub description: String,

    /// An optional link to the project's source code.
    pub source: Option<String>,

    /// An optional link to the project's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the project's wiki.
    pub wiki: Option<String>,

    /// The date the project was created.
    pub created_at: NaiveDateTime,

    /// The date the project was last updated.
    pub updated_at: NaiveDateTime,

    /// The number of downloads the project has.
    pub downloads: i32,

    /// This project's authors.
    pub authors: Vec<User>,

    /// The visibility of a project.
    pub visibility: ProjectVisibility,

    /// The license the project is under.
    pub license: Option<String>,

    /// A list of tags for this project.
    pub tags: Vec<String>,
}

impl Project {
    /// Turn this into a [`ProjectData`] by providing a list of [`User`]s.
    pub fn with_authors(self, authors: Vec<User>) -> ProjectData {
        ProjectData {
            id: self.id,
            name: self.name,
            slug: self.slug,
            readme: self.readme,
            description: self.description,
            source: self.source,
            issues: self.issues,
            wiki: self.wiki,
            created_at: self.created_at,
            updated_at: self.updated_at,
            downloads: self.downloads,
            visibility: self.visibility,
            license: self.license,
            tags: self.tags.into_iter().filter_map(|v| v).collect_vec(),
            authors,
        }
    }
}

impl ProjectVisibility {
    /// Get the string form of this.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "Private",
            Self::Public => "Public",
            Self::Unlisted => "Unlisted",
        }
    }
}
