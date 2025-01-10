//! Utilities for project versions.

use anyhow::anyhow;
use app_core::Result;
use db::{project_versions, DbConn, ProjectVersion, ProjectVersionData};
use db_util::vers::get_versions;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use semver::Version;

/// Get a project's latest version.
pub async fn get_latest_version(project: i32, conn: &mut DbConn) -> Result<ProjectVersion> {
    let mut versions = project_versions::table
        .filter(project_versions::project.eq(project))
        .select(ProjectVersion::as_select())
        .load(conn)
        .await?;

    versions.sort_by(|a, b| {
        Version::parse(&a.version_number)
            .unwrap()
            .cmp(&Version::parse(&b.version_number).unwrap())
    });

    versions
        .last()
        .cloned()
        .ok_or(anyhow!("Could not find latest version!").into())
}

/// Get a project's latest version.
pub async fn get_latest_full_version(
    project: i32,
    conn: &mut DbConn,
) -> Result<ProjectVersionData> {
    let mut versions = get_versions(project, conn).await?;

    versions.sort_by(|a, b| {
        Version::parse(&a.version_number)
            .unwrap()
            .cmp(&Version::parse(&b.version_number).unwrap())
    });

    versions
        .last()
        .cloned()
        .ok_or(anyhow!("Could not find latest version!").into())
}
