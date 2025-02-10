//! Utilities for working with projects.

use crate::moderation::get_moderation_queue_item;
use diesel::{
    BelongingToDsl, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use modhost_core::Result;
use modhost_db::{
    DbConn, GalleryImage, ModerationQueueStatus, Project, ProjectAuthor, ProjectData,
    ProjectVisibility, User, projects, users,
};

/// Utilities for working with a project.
pub trait ProjectUtils {
    /// Check if this project is publicly visible in search.
    async fn is_visible_in_search(&self, conn: &mut DbConn) -> Result<bool>;

    /// Check if this project is visible (public or unlisted).
    async fn is_visible(&self, conn: &mut DbConn) -> Result<bool>;

    /// Check if this project is visible to a user.
    async fn is_visible_to(&self, user: &User, conn: &mut DbConn) -> Result<bool>;
}

impl ProjectUtils for Project {
    async fn is_visible_in_search(&self, conn: &mut DbConn) -> Result<bool> {
        Ok(self.visibility == ProjectVisibility::Public
            && get_moderation_queue_item(self, conn).await?.status
                == ModerationQueueStatus::Approved)
    }

    async fn is_visible(&self, conn: &mut DbConn) -> Result<bool> {
        Ok((self.visibility == ProjectVisibility::Public
            || self.visibility == ProjectVisibility::Unlisted)
            && get_moderation_queue_item(self, conn).await?.status
                == ModerationQueueStatus::Approved)
    }

    async fn is_visible_to(&self, user: &User, conn: &mut DbConn) -> Result<bool> {
        Ok((self.visibility == ProjectVisibility::Public
            && get_moderation_queue_item(self, conn).await?.status
                == ModerationQueueStatus::Approved)
            || get_full_project(self.id.to_string(), conn)
                .await?
                .authors
                .contains(user))
    }
}

impl ProjectUtils for ProjectData {
    async fn is_visible_in_search(&self, conn: &mut DbConn) -> Result<bool> {
        self.clone().into_project().is_visible_in_search(conn).await
    }

    async fn is_visible(&self, conn: &mut DbConn) -> Result<bool> {
        self.clone().into_project().is_visible(conn).await
    }

    async fn is_visible_to(&self, user: &User, conn: &mut DbConn) -> Result<bool> {
        Ok((self.visibility == ProjectVisibility::Public
            && get_moderation_queue_item(&self.clone().into_project(), conn)
                .await?
                .status
                == ModerationQueueStatus::Approved)
            || self.authors.contains(user))
    }
}

/// Get a project by its ID or slug.
pub async fn get_project(id: impl AsRef<str>, conn: &mut DbConn) -> Result<Project> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let pkg = projects::table
            .find(id)
            .select(Project::as_select())
            .first(conn)
            .await
            .optional()?;

        if let Some(pkg) = pkg {
            return Ok(pkg);
        }
    }

    Ok(projects::table
        .filter(projects::slug.ilike(id))
        .select(Project::as_select())
        .first(conn)
        .await?)
}

/// Get the full data for a project by its ID or slug.
pub async fn get_full_project(id: impl AsRef<str>, conn: &mut DbConn) -> Result<ProjectData> {
    let proj = get_project(id, conn).await?;

    let authors = ProjectAuthor::belonging_to(&proj)
        .inner_join(users::table)
        .select(User::as_select())
        .load(conn)
        .await?;

    Ok(proj.with_authors(authors))
}

/// Get the gallery images for a project.
pub async fn get_gallery(pkg_id: impl AsRef<str>, conn: &mut DbConn) -> Result<Vec<GalleryImage>> {
    let proj = get_project(pkg_id, conn).await?;

    Ok(GalleryImage::belonging_to(&proj)
        .select(GalleryImage::as_select())
        .load(conn)
        .await?)
}
