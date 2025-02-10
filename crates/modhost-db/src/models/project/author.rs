//! The project author relation model.

use crate::{Project, User, schema::project_authors};
use diesel::pg::Pg;

/// A project author.
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
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = project_authors)]
#[diesel(belongs_to(Project, foreign_key = project))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(project, user_id))]
pub struct ProjectAuthor {
    /// The project ID.
    pub project: i32,

    /// The user ID.
    pub user_id: i32,
}
