#![deprecated]
#![allow(deprecated)]
//! Synchronous utilities.

use crate::{packages, users, Package, PackageAuthor, PackageData, Result, SyncDbConn, User};
use diesel::{
    BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

/// Synchronously get a package by its ID or slug.
pub fn get_package_sync(id: impl AsRef<str>, conn: &mut SyncDbConn) -> Result<Package> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let pkg = packages::table
            .find(id)
            .select(Package::as_select())
            .first(conn)
            .optional()?;

        if let Some(pkg) = pkg {
            return Ok(pkg);
        }
    }

    Ok(packages::table
        .filter(packages::slug.eq(id))
        .select(Package::as_select())
        .first(conn)?)
}

/// Synchronously get the full data for a package by its ID or slug.
pub fn get_full_package_sync(id: impl AsRef<str>, conn: &mut SyncDbConn) -> Result<PackageData> {
    let pkg = get_package_sync(id, conn)?;

    let authors = PackageAuthor::belonging_to(&pkg)
        .inner_join(users::table)
        .select(User::as_select())
        .load(conn)?;

    Ok(pkg.with_authors(authors))
}
