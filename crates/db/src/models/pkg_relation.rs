//! Package relation-related models.

use crate::{
    models::pkg_ver::{PackageVersion, PackageVersionRef},
    schema::package_relations,
};
use diesel::{
    backend::Backend,
    deserialize::{FromSql, Result as DeResult},
    expression::AsExpression,
    internal::derives::as_expression::Bound,
    pg::Pg,
    serialize::{Output, Result as SerResult, ToSql},
    sql_types::Integer,
    Queryable,
};

// TODO: Do something with this xD
/// A relation between packages.
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
    Queryable,
    Selectable,
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = package_relations)]
#[diesel(belongs_to(PackageVersion, foreign_key = package))]
#[diesel(belongs_to(PackageVersionRef, foreign_key = dependency))]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(package, dependency, kind))]
pub struct PackageRelation {
    /// The package ID.
    pub package: i32,

    /// The dependency version ID.
    pub dependency: i32,

    /// The relation kind.
    pub kind: RelationKind,
}

/// The kind of relation between packages.
#[repr(i32)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
)]
pub enum RelationKind {
    /// A dependency relation.
    #[default]
    Dependency = 0,

    /// An incompatibility relation.
    Incompatibility = 1,
}

impl ToSql<Integer, Pg> for RelationKind {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerResult {
        match self {
            Self::Dependency => <i32 as ToSql<Integer, Pg>>::to_sql(&0, out),
            Self::Incompatibility => <i32 as ToSql<Integer, Pg>>::to_sql(&1, out),
        }
    }
}

impl FromSql<Integer, Pg> for RelationKind {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> DeResult<Self> {
        match <i32 as FromSql<Integer, Pg>>::from_sql(bytes)? {
            0 => Ok(Self::Dependency),
            1 => Ok(Self::Incompatibility),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl Queryable<Integer, Pg> for RelationKind {
    type Row = Self;

    fn build(row: Self::Row) -> DeResult<Self> {
        Ok(row)
    }
}

impl AsExpression<Integer> for RelationKind {
    type Expression = Bound<Integer, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Integer> for &'a RelationKind {
    type Expression = Bound<Integer, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
