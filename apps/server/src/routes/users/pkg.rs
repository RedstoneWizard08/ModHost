//! Routes concerning user packages.

use crate::{auth::get_user_from_req, state::AppState, Result};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use db::{
    get_user, package_authors, packages, users, Package, PackageAuthor, PackageData,
    PackageVisibility, User,
};
use diesel::{BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// The amount of time for a user's packages cache to automatically expire.
const CACHE_EXPIRY_MS: i64 = 15 * 60 * 1000; // 15 minutes = 15m * 60s * 1000ms

lazy_static! {
    /// A cache of user packages.
    /// This is cached because retrieving this info is a slow process.
    ///
    /// [!TODO] The method of retreiving this info will soon be changed to a single select
    /// query with some array manipulation, so this will be unnecessary when that happens.
    static ref USER_PACKAGES_CACHE: Arc<Mutex<HashMap<i32, (i64, Vec<PackageData>)>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// Manually clear a user's packages cache.
pub async fn clear_user_cache(id: i32) {
    USER_PACKAGES_CACHE.lock().await.remove(&id);
}

/// Get User Packages
///
/// Get a user's packages.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/packages",
    tag = "Users",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Found packages!", body = Vec<PackageData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! The user may not exist!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<PackageData>>> {
    let mut conn = state.pool.get().await?;
    let user = get_user(id, &mut conn).await?;
    let mut lock = USER_PACKAGES_CACHE.lock().await;

    if let Some((expires, data)) = lock.get(&user.id) {
        let now = Utc::now().timestamp_millis();

        if *expires > now {
            return match get_user_from_req(&jar, &headers, &mut conn).await {
                Ok(user) => Ok(Json(
                    data.clone()
                        .into_iter()
                        .filter(|v| {
                            v.visibility == PackageVisibility::Public
                                || v.authors.iter().any(|v| v.github_id == user.github_id)
                                || user.admin
                        })
                        .collect(),
                )),

                Err(_) => Ok(Json(
                    data.clone()
                        .into_iter()
                        .filter(|v| v.visibility == PackageVisibility::Public)
                        .collect(),
                )),
            };
        }
    }

    // TODO: Do this as a single query
    let pkgs = package_authors::table
        .filter(package_authors::user_id.eq(user.id))
        .inner_join(packages::table)
        .select((PackageAuthor::as_select(), Package::as_select()))
        .load(&mut conn)
        .await?;

    let pkgs = pkgs.iter().map(|(_, pkg)| pkg).collect::<Vec<_>>();

    let users: Vec<(PackageAuthor, User)> = PackageAuthor::belonging_to(&pkgs)
        .inner_join(users::table)
        .select((PackageAuthor::as_select(), User::as_select()))
        .load(&mut conn)
        .await
        .unwrap();

    let res = users
        .grouped_by(&pkgs)
        .into_iter()
        .zip(pkgs)
        .map(|(users, pkg)| {
            pkg.clone()
                .with_authors(users.iter().map(|(_, user)| user.clone()).collect())
        })
        .collect::<Vec<_>>();

    lock.insert(
        user.id,
        (Utc::now().timestamp_millis() + CACHE_EXPIRY_MS, res.clone()),
    );

    match get_user_from_req(&jar, &headers, &mut conn).await {
        Ok(user) => Ok(Json(
            res.iter()
                .filter(|v| {
                    v.visibility == PackageVisibility::Public
                        || v.authors.iter().any(|v| v.github_id == user.github_id)
                        || user.admin
                })
                .cloned()
                .collect(),
        )),

        Err(_) => Ok(Json(
            res.iter()
                .filter(|v| v.visibility == PackageVisibility::Public)
                .cloned()
                .collect(),
        )),
    }
}
