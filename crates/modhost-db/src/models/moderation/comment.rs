//! Moderation comment models.

use crate::{moderation_comment, Project, User};
use diesel::pg::Pg;

/// A moderation comment.
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
#[diesel(table_name = moderation_comment)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(Pg))]
pub struct ModerationComment {
    /// The comment ID.
    pub id: i32,

    /// The project this comment is for.
    pub project_id: i32,

    /// The user who wrote this comment.
    pub user_id: i32,

    /// Whether this message is from the system.
    pub is_system: bool,

    /// Whether this message is from a moderator.
    pub is_moderator: bool,

    /// The comment's text itself.
    pub comment: String,
}

/// An insertable moderation comment.
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
    Insertable,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = moderation_comment)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(Pg))]
pub struct NewModerationComment {
    /// The project this comment is for.
    pub project_id: i32,

    /// The user who wrote this comment.
    pub user_id: i32,

    /// Whether this message is from the system.
    pub is_system: bool,

    /// Whether this message is from a moderator.
    pub is_moderator: bool,

    /// The comment's text itself.
    pub comment: String,
}
