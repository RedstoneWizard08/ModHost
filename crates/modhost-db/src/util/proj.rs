//! Utilities for working with projects.

use crate::{
    schema::{projects, users},
    DbConn, GalleryImage, Project, ProjectAuthor, ProjectData, Result, User,
};
use diesel::{
    BelongingToDsl, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

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
