//! Utilities for packages.

use crate::{
    schema::{packages, users},
    DbConn, GalleryImage, Package, PackageAuthor, PackageData, Result, User,
};
use diesel::{
    BelongingToDsl, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

/// Get a package by its ID or slug.
pub async fn get_package(id: impl AsRef<str>, conn: &mut DbConn) -> Result<Package> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let pkg = packages::table
            .find(id)
            .select(Package::as_select())
            .first(conn)
            .await
            .optional()?;

        if let Some(pkg) = pkg {
            return Ok(pkg);
        }
    }

    Ok(packages::table
        .filter(packages::slug.ilike(id))
        .select(Package::as_select())
        .first(conn)
        .await?)
}

/// Get the full data for a package by its ID or slug.
pub async fn get_full_package(id: impl AsRef<str>, conn: &mut DbConn) -> Result<PackageData> {
    let pkg = get_package(id, conn).await?;

    let authors = PackageAuthor::belonging_to(&pkg)
        .inner_join(users::table)
        .select(User::as_select())
        .load(conn)
        .await?;

    Ok(pkg.with_authors(authors))
}

/// Get the gallery images for a package.
pub async fn get_gallery(pkg_id: impl AsRef<str>, conn: &mut DbConn) -> Result<Vec<GalleryImage>> {
    let pkg = get_package(pkg_id, conn).await?;

    Ok(GalleryImage::belonging_to(&pkg)
        .select(GalleryImage::as_select())
        .load(conn)
        .await?)
}
