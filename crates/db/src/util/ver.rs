//! Utilities for package versions.

use crate::{schema::package_versions, DbConn, PackageVersion};
use app_core::Result;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

/// Get a version by its ID, name, or version number.
pub async fn get_version(
    pkg: i32,
    id: impl AsRef<str>,
    conn: &mut DbConn,
) -> Result<PackageVersion> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let ver = package_versions::table
            .find(id)
            .select(PackageVersion::as_select())
            .first(conn)
            .await
            .optional()?;

        if let Some(ver) = ver {
            return Ok(ver);
        }
    }

    if let Some(ver) = package_versions::table
        .filter(
            package_versions::version_number
                .eq(id)
                .and(package_versions::package.eq(pkg)),
        )
        .select(PackageVersion::as_select())
        .first(conn)
        .await
        .optional()?
    {
        return Ok(ver);
    }

    Ok(package_versions::table
        .filter(
            package_versions::name
                .eq(id)
                .and(package_versions::package.eq(pkg)),
        )
        .select(PackageVersion::as_select())
        .first(conn)
        .await?)
}
