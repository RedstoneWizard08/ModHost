use super::DbConn;
use crate::{schema::users, Result, User};
use diesel::{
    ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;

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

pub async fn search_users(name: impl AsRef<str>, conn: &mut DbConn) -> Result<Vec<User>> {
    let name = name.as_ref();

    Ok(users::table
        .filter(users::username.ilike(format!("%{}%", name)))
        .select(User::as_select())
        .load(conn)
        .await?)
}
