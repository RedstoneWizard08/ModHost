//! Utilities for tokens.

use crate::{user_tokens, users, DbConn, DbPool, NewUserToken, User, UserToken};
use modhost_core::Result;
use chrono::{DateTime, Utc};
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use random_string::{charsets::ALPHANUMERIC, generate};

/// The time until a token expires in milliseconds.
/// Calculation: 1 * SECS_PER_WEEK * MILLIS_PER_SEC
#[allow(clippy::identity_op)]
pub const TOKEN_EXPIRE_TIME: i64 = 1 * 604800 * 1000;

/// The length of a token string.
pub const TOKEN_LENGTH: usize = 64;

/// Generate a token to insert into the database.
pub fn generate_token(user_id: i32) -> NewUserToken {
    NewUserToken {
        user_id,
        value: generate(TOKEN_LENGTH, ALPHANUMERIC),
        expires: DateTime::from_timestamp_millis(Utc::now().timestamp_millis() + TOKEN_EXPIRE_TIME)
            .unwrap()
            .naive_utc(),
    }
}

/// Create a token and insert it into the database.
pub async fn create_token(user_id: i32, pool: &DbPool) -> Result<UserToken> {
    Ok(insert_into(user_tokens::table)
        .values(&generate_token(user_id))
        .returning(UserToken::as_returning())
        .get_result(&mut pool.get().await?)
        .await?)
}

/// Get the user the provided token belongs to.
pub async fn get_user_for_token(token: impl AsRef<str>, conn: &mut DbConn) -> Result<Option<User>> {
    let token = user_tokens::table
        .filter(user_tokens::value.eq(token.as_ref().to_string()))
        .select(UserToken::as_select())
        .first(conn)
        .await
        .optional()?;

    if let Some(token) = token {
        Ok(users::table
            .filter(users::id.eq(token.user_id))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()?)
    } else {
        Ok(None)
    }
}
