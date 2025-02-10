//! Utilities for working with project versions.

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use modhost_core::{AppError, Result};
use modhost_db::{
    DbConn, ProjectFile, ProjectVersion, ProjectVersionData, project_versions, version_files,
};
use semver::Version;

/// Get a list of versions for a project.
pub async fn get_versions(project: i32, conn: &mut DbConn) -> Result<Vec<ProjectVersionData>> {
    Ok((project_versions::table
        .inner_join(version_files::table)
        .select((ProjectVersion::as_select(), ProjectFile::as_select()))
        .filter(project_versions::project.eq(project))
        .load::<(ProjectVersion, ProjectFile)>(conn)
        .await? as Vec<(ProjectVersion, ProjectFile)>)
        .into_iter()
        .into_group_map()
        .into_iter()
        .map(|v| v.0.with_files(v.1))
        .collect_vec())
}

/// Get the full version data for a project.
pub async fn get_full_version(
    project: i32,
    ver: impl AsRef<str>,
    conn: &mut DbConn,
) -> Result<ProjectVersionData> {
    let ver = ver.as_ref();

    let mut query = project_versions::table
        .inner_join(version_files::table)
        .select((ProjectVersion::as_select(), ProjectFile::as_select()))
        .filter(project_versions::project.eq(project))
        .into_boxed();

    if let Ok(ver_num) = ver.parse::<i32>() {
        query = query.filter(
            project_versions::id.eq(ver_num).or(project_versions::name
                .eq(ver)
                .or(project_versions::version_number.eq(ver))),
        );
    } else {
        query = query.filter(
            project_versions::name
                .eq(ver)
                .or(project_versions::version_number.eq(ver)),
        );
    }

    (query.load::<(ProjectVersion, ProjectFile)>(conn).await? as Vec<(ProjectVersion, ProjectFile)>)
        .into_iter()
        .into_group_map()
        .into_iter()
        .map(|v| v.0.with_files(v.1))
        .next()
        .ok_or(AppError::NotFound)
}

/// Get a version file.
pub async fn get_version_file(
    ver: i32,
    file: impl AsRef<str>,
    conn: &mut DbConn,
) -> Result<ProjectFile> {
    let file = file.as_ref();

    let mut query = version_files::table
        .select(ProjectFile::as_select())
        .filter(version_files::version_id.eq(ver))
        .into_boxed();

    if let Ok(file_id) = file.parse::<i32>() {
        query = query.filter(
            version_files::id
                .eq(file_id)
                .or(version_files::file_name.eq(file)),
        );
    } else {
        query = query.filter(version_files::file_name.eq(file));
    }

    Ok(query.first::<ProjectFile>(conn).await?)
}

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

    versions.last().cloned().ok_or(AppError::NoVersions)
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

    versions.last().cloned().ok_or(AppError::NoVersions)
}
