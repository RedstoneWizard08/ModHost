//! Project relation-related models.

use crate::{schema::project_relations, ProjectVersion, ProjectVersionRef};
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
/// A relation between projects.
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
#[diesel(table_name = project_relations)]
#[diesel(belongs_to(ProjectVersion, foreign_key = project))]
#[diesel(belongs_to(ProjectVersionRef, foreign_key = dependency))]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(project, dependency, kind))]
pub struct ProjectRelation {
    /// The project ID.
    pub project: i32,

    /// The dependency version ID.
    pub dependency: i32,

    /// The relation kind.
    pub kind: RelationKind,
}

/// The kind of relation between projects.
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
