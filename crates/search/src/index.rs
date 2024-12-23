//! Utilities for indexing packages.

use crate::{MeiliPackage, MeilisearchService};
use anyhow::anyhow;
use app_core::Result;
use db::{
    package_authors, package_versions, packages, users, DbConn, Package, PackageVersion, User,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use meilisearch_sdk::documents::DocumentDeletionQuery;

impl MeilisearchService {
    /// Index all packages present in the database.
    /// THIS IS A DESTRUCTIVE ACTION! IT WILL DELETE ALL EXISTING DATA
    /// IN THE MEILISEARCH INDEX!
    pub async fn index_packages(&self, conn: &mut DbConn) -> Result<()> {
        // This is my baby abomination and I am so proud of it.
        let packages: Vec<MeiliPackage> = packages::table
            .inner_join(package_authors::table.inner_join(users::table))
            .inner_join(package_versions::table)
            .select((
                Package::as_select(),
                User::as_select(),
                PackageVersion::as_select(),
            ))
            .load::<(Package, User, PackageVersion)>(conn)
            .await?
            .into_iter()
            .into_group_map_by(|v: &(Package, User, PackageVersion)| v.0.clone())
            .into_iter()
            .map(|v: (Package, Vec<(Package, User, PackageVersion)>)| {
                (
                    v.0,
                    v.1.into_iter()
                        .map(|v| (v.1, v.2))
                        .unzip::<User, PackageVersion, Vec<User>, Vec<PackageVersion>>(),
                )
            })
            .map(|v| MeiliPackage::from_data(v.0, v.1 .0, v.1 .1))
            .collect_vec();

        let index = self.packages();

        index.delete_all_documents().await?;
        index.add_documents(packages.as_slice(), Some("id")).await?;

        Ok(())
    }

    /// Update a package in the Meilisearch index.
    pub async fn update_package(&self, pkg: i32, conn: &mut DbConn) -> Result<()> {
        // Abomination #2! It's so beautiful! I make Rust programmers worldwide upset!
        let data: MeiliPackage = packages::table
            .inner_join(package_authors::table.inner_join(users::table))
            .inner_join(package_versions::table)
            .select((
                Package::as_select(),
                User::as_select(),
                PackageVersion::as_select(),
            ))
            .filter(packages::id.eq(pkg))
            .load::<(Package, User, PackageVersion)>(conn)
            .await?
            .into_iter()
            .into_group_map_by(|v: &(Package, User, PackageVersion)| v.0.clone())
            .into_iter()
            .map(|v: (Package, Vec<(Package, User, PackageVersion)>)| {
                (
                    v.0,
                    v.1.into_iter()
                        .map(|v| (v.1, v.2))
                        .unzip::<User, PackageVersion, Vec<User>, Vec<PackageVersion>>(),
                )
            })
            .map(|v| MeiliPackage::from_data(v.0, v.1 .0, v.1 .1))
            .find(|v| v.id == pkg)
            .ok_or(anyhow!("Could not find package with ID {}!", pkg))?;

        self.packages()
            .add_or_replace(&[data], Some("id"))
            .await?
            .wait_for_completion(&self.client, None, None)
            .await?;

        Ok(())
    }

    /// Delete a package from the Meilisearch index.
    pub async fn delete_package(&self, pkg: i32) -> Result<()> {
        let index = self.packages();
        let mut query = DocumentDeletionQuery::new(&index);
        let filter = format!("id = {}", pkg);

        query.with_filter(&filter);
        index.delete_documents_with(&query).await?;

        Ok(())
    }
}
