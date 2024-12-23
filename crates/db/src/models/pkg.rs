//! Package-related models.

use super::User;
use crate::schema::packages;
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel_derive_enum::DbEnum;
use itertools::Itertools;

/// A package's visibility.
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
pub enum PackageVisibility {
    /// The package is publicly visible.
    #[default]
    Public,

    /// The package is private.
    Private,

    /// The package is unlisted; it will not show up in search but can be accessed via ID or slug.
    Unlisted,
}

/// A package.
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
#[diesel(table_name = packages)]
#[diesel(check_for_backend(Pg))]
pub struct Package {
    /// The package's ID.
    pub id: i32,

    /// The package's name.
    pub name: String,

    /// The package's URL slug.
    pub slug: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// The date the package was created.
    pub created_at: NaiveDateTime,

    /// The date the package was last updated.
    pub updated_at: NaiveDateTime,

    /// The amount of downloads a package has.
    pub downloads: i32,

    /// An optional link to the package's source code.
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    pub wiki: Option<String>,

    /// The visibility of a package.
    pub visibility: PackageVisibility,

    /// The license the package is under.
    pub license: Option<String>,

    /// A list of tags for this package.
    pub tags: Vec<Option<String>>,
}

/// A model for creating a new package.
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
#[diesel(table_name = packages)]
#[diesel(check_for_backend(Pg))]
pub struct NewPackage {
    /// The package's URL slug.
    pub slug: String,

    /// The package's name.
    pub name: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// An optional link to the package's source code.
    #[serde(default)]
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    #[serde(default)]
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    #[serde(default)]
    pub wiki: Option<String>,

    /// The visibility of a package. Optional. Defaults to public.
    #[serde(default)]
    pub visibility: PackageVisibility,

    /// The license the package is under.
    #[serde(default)]
    pub license: Option<String>,

    /// A list of tags for this package.
    #[serde(default)]
    pub tags: Vec<Option<String>>,
}

/// A package with additional data.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct PackageData {
    /// The package's ID.
    pub id: i32,

    /// The package's name.
    pub name: String,

    /// The package's URL slug.
    pub slug: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// An optional link to the package's source code.
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    pub wiki: Option<String>,

    /// The date the package was created.
    pub created_at: NaiveDateTime,

    /// The date the package was last updated.
    pub updated_at: NaiveDateTime,

    /// The number of downloads the package has.
    pub downloads: i32,

    /// This package's authors.
    pub authors: Vec<User>,

    /// The visibility of a package.
    pub visibility: PackageVisibility,

    /// The license the package is under.
    pub license: Option<String>,

    /// A list of tags for this package.
    pub tags: Vec<String>,
}

impl Package {
    /// Turn this package into [`PackageData`] by providing a list of [`User`]s.
    pub fn with_authors(self, authors: Vec<User>) -> PackageData {
        PackageData {
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

impl PackageVisibility {
    /// Get the string form of this.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "Private",
            Self::Public => "Public",
            Self::Unlisted => "Unlisted",
        }
    }
}
