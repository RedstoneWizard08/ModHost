//! Moderation data utilities.

use diesel::{insert_into, update, BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use modhost_core::Result;
use modhost_db::{
    moderation_comment, moderation_queue, DbConn, ModerationComment, ModerationQueueItem,
    ModerationQueueStatus, NewModerationComment, NewModerationQueueItem, Project, User,
};

/// Get the entire queue of pending moderation items.
pub async fn get_pending_moderation_queue(conn: &mut DbConn) -> Result<Vec<ModerationQueueItem>> {
    Ok(moderation_queue::table
        .select(ModerationQueueItem::as_select())
        .filter(moderation_queue::status.eq(ModerationQueueStatus::Pending))
        .load(conn)
        .await?)
}

/// Get the entire queue of approved moderation items.
pub async fn get_approved_moderation_queue(conn: &mut DbConn) -> Result<Vec<ModerationQueueItem>> {
    Ok(moderation_queue::table
        .select(ModerationQueueItem::as_select())
        .filter(moderation_queue::status.eq(ModerationQueueStatus::Approved))
        .load(conn)
        .await?)
}

/// Get the entire queue of denied moderation items.
pub async fn get_denied_moderation_queue(conn: &mut DbConn) -> Result<Vec<ModerationQueueItem>> {
    Ok(moderation_queue::table
        .select(ModerationQueueItem::as_select())
        .filter(moderation_queue::status.eq(ModerationQueueStatus::Denied))
        .load(conn)
        .await?)
}

/// Get the entire queue of under review moderation items.
pub async fn get_under_review_moderation_queue(
    conn: &mut DbConn,
) -> Result<Vec<ModerationQueueItem>> {
    Ok(moderation_queue::table
        .select(ModerationQueueItem::as_select())
        .filter(moderation_queue::status.eq(ModerationQueueStatus::UnderReview))
        .load(conn)
        .await?)
}

/// Get the entire moderation queue.
pub async fn get_moderation_queue(conn: &mut DbConn) -> Result<Vec<ModerationQueueItem>> {
    Ok(moderation_queue::table
        .select(ModerationQueueItem::as_select())
        .load(conn)
        .await?)
}

/// Get the moderation queue item for a project.
pub async fn get_moderation_queue_item(
    project: &Project,
    conn: &mut DbConn,
) -> Result<ModerationQueueItem> {
    Ok(ModerationQueueItem::belonging_to(project)
        .select(ModerationQueueItem::as_select())
        .first(conn)
        .await?)
}

/// Get or create the moderation queue item for a project.
pub async fn get_or_create_moderation_queue_item(
    project: &Project,
    conn: &mut DbConn,
) -> Result<ModerationQueueItem> {
    let existing = ModerationQueueItem::belonging_to(project)
        .select(ModerationQueueItem::as_select())
        .load(conn)
        .await?;

    match existing.into_iter().next() {
        Some(it) => Ok(it),
        None => Ok(insert_into(moderation_queue::table)
            .values(NewModerationQueueItem {
                assigned_id: None,
                project_id: project.id,
                status: ModerationQueueStatus::Pending,
            })
            .returning(ModerationQueueItem::as_returning())
            .get_result(conn)
            .await?),
    }
}

/// Set the moderation status for a project.
pub async fn set_moderation_status(
    project: &Project,
    status: ModerationQueueStatus,
    conn: &mut DbConn,
) -> Result<ModerationQueueItem> {
    let item = get_moderation_queue_item(project, conn).await?;

    Ok(update(moderation_queue::table)
        .filter(moderation_queue::id.eq(item.id))
        .set(moderation_queue::status.eq(status))
        .returning(ModerationQueueItem::as_returning())
        .get_result(conn)
        .await?)
}

/// Set the assigned moderator for a project.
pub async fn set_assigned_moderator(
    project: &Project,
    assigned: i32,
    conn: &mut DbConn,
) -> Result<ModerationQueueItem> {
    let item = get_moderation_queue_item(project, conn).await?;

    Ok(update(moderation_queue::table)
        .filter(moderation_queue::id.eq(item.id))
        .set(moderation_queue::assigned_id.eq(assigned))
        .returning(ModerationQueueItem::as_returning())
        .get_result(conn)
        .await?)
}

/// Get the moderation comments for a project.
pub async fn get_moderation_comments(
    project: &Project,
    conn: &mut DbConn,
) -> Result<Vec<ModerationComment>> {
    Ok(ModerationComment::belonging_to(project)
        .select(ModerationComment::as_select())
        .load(conn)
        .await?)
}

/// Create a new moderation comment on a project.
pub async fn create_moderation_comment(
    project: &Project,
    user: &User,
    comment: String,
    conn: &mut DbConn,
) -> Result<ModerationComment> {
    Ok(insert_into(moderation_comment::table)
        .values(NewModerationComment {
            user_id: user.id,
            project_id: project.id,
            is_moderator: user.moderator || user.admin,
            is_system: user.id == -1,
            comment,
        })
        .returning(ModerationComment::as_returning())
        .get_result(conn)
        .await?)
}
