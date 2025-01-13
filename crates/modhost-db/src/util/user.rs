//! Utilities for users.

use crate::{schema::users, DbConn, User};
use modhost_core::Result;
use diesel::{
    ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

/// Get a user by their ID, GitHub ID, or their username.
pub async fn get_user(id: impl AsRef<str>, conn: &mut DbConn) -> Result<User> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        if let Some(user) = users::table
            .find(id)
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?
        {
            return Ok(user);
        }

        if let Some(user) = users::table
            .filter(users::github_id.eq(id))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?
        {
            return Ok(user);
        }
    }

    Ok(users::table
        .filter(users::username.eq(id))
        .select(User::as_select())
        .first(conn)
        .await?)
}

/// Search for users with the specified string in their username.
pub async fn search_users(name: impl AsRef<str>, conn: &mut DbConn) -> Result<Vec<User>> {
    let name = name.as_ref();

    Ok(users::table
        .filter(users::username.ilike(format!("%{}%", name)))
        .select(User::as_select())
        .load(conn)
        .await?)
}
