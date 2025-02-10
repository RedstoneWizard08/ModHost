//! Moderation queue models.

use crate::{Project, User, schema::moderation_queue};
use diesel::pg::Pg;
use diesel_derive_enum::DbEnum;

/// The status of an item in the moderation queue.
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
#[ExistingTypePath = "crate::schema::sql_types::ModerationStatus"]
pub enum ModerationQueueStatus {
    /// This project has yet to be reviewed.
    #[default]
    Pending,

    /// This project has failed review.
    Denied,

    /// This project has been approved.
    Approved,

    /// This project is currently under review.
    UnderReview,
}

/// An item in the moderation queue.
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
#[diesel(table_name = moderation_queue)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = assigned_id))]
#[diesel(check_for_backend(Pg))]
pub struct ModerationQueueItem {
    /// The item ID.
    pub id: i32,

    /// The project this item is for.
    pub project_id: i32,

    /// The user this item has been assigned to.
    pub assigned_id: Option<i32>,

    /// The status of the item.
    pub status: ModerationQueueStatus,
}

/// An insertable item in the moderation queue.
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
#[diesel(table_name = moderation_queue)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = assigned_id))]
#[diesel(check_for_backend(Pg))]
pub struct NewModerationQueueItem {
    /// The project this item is for.
    pub project_id: i32,

    /// The user this item has been assigned to.
    pub assigned_id: Option<i32>,

    /// The status of the item.
    pub status: ModerationQueueStatus,
}
