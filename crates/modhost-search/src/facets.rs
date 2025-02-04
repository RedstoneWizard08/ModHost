//! The front-facing facet filter system.

use anyhow::anyhow;
use chrono::NaiveDateTime;
use modhost_core::Result;
use modhost_db::ProjectVisibility;

/// A facet/filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, ToResponse)]
pub enum Facet {
    /// Filters game versions. It will match any provided.
    GameVersions(Vec<String>),

    /// Filters mod loaders. It will match any provided.
    Loaders(Vec<String>),

    /// Filter by tags. It will match any provided.
    Tags(Vec<String>),

    /// Filter by a range of dates when the project was published.
    /// The first element is the minimum date, the second is the maximum.
    /// The comparison is `(item.published >= a && item.published <= b)`
    Published(NaiveDateTime, NaiveDateTime),

    /// Filter by a range of dates when the project was updated.
    /// The first element is the minimum date, the second is the maximum.
    /// The comparison is `(item.published >= a && item.published <= b)`
    Updated(NaiveDateTime, NaiveDateTime),

    /// Filter by a range of download counts for project.
    /// The first element is the minimum, the second is the maximum.
    /// The comparison is `(item.published >= a && item.published <= b)`
    Downloads(i32, i32),

    /// Filter by project visibility.
    /// This is used internally, and is not accepted in the front-facing search API.
    #[serde(skip)]
    Visibility(ProjectVisibility),

    /// Filter by authors. This will match if one of the project's authors
    /// has the ID provided.
    /// This is used internally, and is not accepted in the front-facing search API.
    #[serde(skip)]
    Author(i32),

    /// Provide a manual Meilisearch filter string.
    /// This is used internally, and is not accepted in the front-facing search API.
    #[serde(skip)]
    Manual(String),
}

/// A sort mode.
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
    Default,
)]
pub enum Sort {
    /// Sort projects with the default sorter.
    #[serde(rename = "none")]
    None,

    /// Sort projects by name, alphabetical.
    #[serde(rename = "name")]
    Name,

    /// Sort projects by published date.
    #[serde(rename = "published")]
    Published,

    /// Sort projects by updated date.
    #[serde(rename = "updated")]
    Updated,

    /// Sort projects by downloads.
    #[serde(rename = "downloads")]
    #[default]
    Downloads,
}

/// The sort direction.
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
    Default,
)]
pub enum SortMode {
    /// Sort projects in ascending order.
    #[serde(rename = "asc")]
    #[default]
    Ascending,

    /// Sort projects in descending order.
    #[serde(rename = "desc")]
    Descending,
}

impl Facet {
    /// Turn this facet into a Meilisearch filter string.
    pub fn into_filter_string(self) -> String {
        format!(
            "({})",
            match self {
                Self::Visibility(v) => format!("visibility = {}", v.as_str()),
                Self::GameVersions(v) => format!("game_versions IN [{}]", v.join(", ")),
                Self::Loaders(v) => format!("loaders IN [{}]", v.join(", ")),
                Self::Tags(v) => format!("tags IN [{}]", v.join(", ")),
                Self::Published(start, end) => format!(
                    "(created_at >= {}) AND (created_at <= {})",
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp()
                ),
                Self::Updated(start, end) => format!(
                    "(updated_at >= {}) AND (updated_at <= {})",
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp()
                ),
                Self::Downloads(start, end) =>
                    format!("(downloads >= {}) AND (downloads <= {})", start, end),
                Self::Author(v) => format!("author_ids IN [{}]", v),
                Self::Manual(s) => s,
            }
        )
    }

    /// Parse a facet.
    /// This will not parse the [`Facet::Visibility`], [`Facet::Author`], or [`Facet::Manual`]
    /// facets.
    pub fn parse(it: (String, Vec<String>)) -> Result<Facet> {
        match it.0.as_str() {
            // 'visibility', and 'author', and 'manual' can only be set by the system for security reasons
            "game_versions" => Ok(Facet::GameVersions(it.1)),
            "loaders" => Ok(Facet::Loaders(it.1)),
            "tags" => Ok(Facet::Tags(it.1)),

            "published" => {
                if it.1.len() == 2 {
                    Ok(Self::Published(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(
                        anyhow!("Invalid array length for 'published' facet: {}", it.1.len())
                            .into(),
                    )
                }
            }

            "updated" => {
                if it.1.len() == 2 {
                    Ok(Self::Updated(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(anyhow!("Invalid array length for 'updated' facet: {}", it.1.len()).into())
                }
            }

            "downloads" => {
                if it.1.len() == 2 {
                    Ok(Self::Downloads(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(
                        anyhow!("Invalid array length for 'downloads' facet: {}", it.1.len())
                            .into(),
                    )
                }
            }

            other => Err(anyhow!("Unknown facet type: {}", other).into()),
        }
    }
}

impl Sort {
    /// Get the field name to sort by.
    pub fn field(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Name => "name",
            Self::Published => "created_at",
            Self::Updated => "updated_at",
            Self::Downloads => "downloads",
        }
    }
}

impl SortMode {
    /// Get the Meilisearch sort mode.
    pub fn mode(&self) -> &'static str {
        match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        }
    }
}
