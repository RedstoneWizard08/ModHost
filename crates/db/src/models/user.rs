//! User-related models.

use crate::schema::{user_tokens, users};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

/// A user.
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    /// The user's ID.
    pub id: i32,

    /// The user's username.
    pub username: String,

    /// The user's GitHub ID.
    pub github_id: i32,

    /// Are they an admin?
    pub admin: bool,

    /// Are they a moderator?
    pub moderator: bool,
}

/// A model for creating a new user in the database.
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct NewUser {
    /// The user's username.
    pub username: String,

    /// The user's GitHub ID.
    pub github_id: i32,
}

/// A user's token.
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
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = user_tokens)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct UserToken {
    /// The token's ID.
    pub id: i32,

    /// The user's ID.
    pub user_id: i32,

    /// The token's value.
    pub value: String,

    /// The token's expiration date.
    pub expires: NaiveDateTime,
}

/// A model for creating a new user token in the database.
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
    Associations,
    Insertable,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = user_tokens)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct NewUserToken {
    /// The user's ID.
    pub user_id: i32,

    /// The token's value.
    pub value: String,

    /// The token's expiration date.
    pub expires: NaiveDateTime,
}
