//! Utilities for project versions.

use crate::{schema::project_versions, DbConn, ProjectVersion};
use modhost_core::Result;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

/// Get a version by its ID, name, or version number.
pub async fn get_version(
    project: i32,
    id: impl AsRef<str>,
    conn: &mut DbConn,
) -> Result<ProjectVersion> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let ver = project_versions::table
            .find(id)
            .select(ProjectVersion::as_select())
            .first(conn)
            .await
            .optional()?;

        if let Some(ver) = ver {
            return Ok(ver);
        }
    }

    if let Some(ver) = project_versions::table
        .filter(
            project_versions::version_number
                .eq(id)
                .and(project_versions::project.eq(project)),
        )
        .select(ProjectVersion::as_select())
        .first(conn)
        .await
        .optional()?
    {
        return Ok(ver);
    }

    Ok(project_versions::table
        .filter(
            project_versions::name
                .eq(id)
                .and(project_versions::project.eq(project)),
        )
        .select(ProjectVersion::as_select())
        .first(conn)
        .await?)
}
