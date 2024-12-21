//! Utilities for package versions.

use anyhow::anyhow;
use app_core::Result;
use db::{package_versions, DbConn, PackageVersion};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use semver::Version;

/// Get a package's latest version.
pub async fn get_latest_version(pkg: i32, conn: &mut DbConn) -> Result<PackageVersion> {
    let mut versions = package_versions::table
        .filter(package_versions::package.eq(pkg))
        .select(PackageVersion::as_select())
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
