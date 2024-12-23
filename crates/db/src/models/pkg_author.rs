//! The package author relation model.

use crate::{
    models::{pkg::Package, user::User},
    schema::package_authors,
};
use diesel::pg::Pg;

/// A package author.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = package_authors)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(package, user_id))]
pub struct PackageAuthor {
    /// The package ID.
    pub package: i32,

    /// The user ID.
    pub user_id: i32,
}
