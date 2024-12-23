//! Models for Meilisearch.

use chrono::NaiveDateTime;
use db::{Package, PackageData, PackageVersion, PackageVisibility, User};
use itertools::Itertools;

/// A package for search indexing.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct MeiliPackage {
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

    /// The versions of this package.
    pub versions: Vec<PackageVersion>,

    /// A list of version IDs for this package.
    pub version_ids: Vec<i32>,

    /// This package's authors.
    pub authors: Vec<User>,

    /// A list of User IDs representing authors for this package.
    pub author_ids: Vec<i32>,

    /// A list of loaders this package supports (all versions).
    pub loaders: Vec<String>,

    /// A list of game versions this package supports (all versions).
    pub game_versions: Vec<String>,

    /// A list of tags for this package.
    pub tags: Vec<String>,
}

/// The search results type.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct SearchResults {
    /// The page number requested.
    pub page: usize,

    /// The total number of pages.
    pub pages: usize,

    /// The number of hits returned from the database.
    /// Most of the time, this will be the same as the pagination limit.
    pub hits: usize,

    /// The total number of items that matched the query.
    pub total: usize,

    /// The returned items.
    pub results: Vec<PackageData>,
}

impl MeiliPackage {
    /// Create a [`MeiliPackage`] from data from the database.
    pub fn from_data(pkg: Package, authors: Vec<User>, versions: Vec<PackageVersion>) -> Self {
        Self {
            id: pkg.id,
            name: pkg.name,
            slug: pkg.slug,
            readme: pkg.readme,
            description: pkg.description,
            created_at: pkg.created_at,
            updated_at: pkg.updated_at,
            downloads: pkg.downloads,
            source: pkg.source,
            issues: pkg.issues,
            wiki: pkg.wiki,
            visibility: pkg.visibility,
            license: pkg.license,
            version_ids: versions.iter().map(|v| v.id).collect_vec(),
            author_ids: authors.iter().map(|v| v.id).collect_vec(),
            loaders: versions
                .iter()
                .flat_map(|v| v.loaders.clone())
                .filter_map(|v| v)
                .sorted()
                .dedup()
                .collect_vec(),
            game_versions: versions
                .iter()
                .flat_map(|v| v.game_versions.clone())
                .filter_map(|v| v)
                .sorted()
                .dedup()
                .collect_vec(),
            tags: pkg.tags.into_iter().filter_map(|v| v).collect_vec(),
            authors,
            versions,
        }
    }

    /// Turn this into [`PackageData`].
    pub fn into_data(self) -> PackageData {
        PackageData {
            id: self.id,
            name: self.name,
            slug: self.slug,
            readme: self.readme,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
            downloads: self.downloads,
            source: self.source,
            issues: self.issues,
            wiki: self.wiki,
            visibility: self.visibility,
            license: self.license,
            authors: self.authors,
            tags: self.tags,
        }
    }
}
