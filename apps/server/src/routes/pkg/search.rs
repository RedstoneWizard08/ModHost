//! The package search route.

use crate::{auth::get_user_from_req, state::AppState, Result};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use db::PackageVisibility;
use search::{Facet, SearchResults, Sort, SortMode};

/// The absolute maximum items per-page for pagination.
/// The value from a query will be clamped with this.
pub const MAX_PER_PAGE: usize = 200;

/// Parameters to the search route.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct SearchQuery {
    /// The query string.
    pub q: Option<String>,

    /// The current page. Defaults to 1.
    pub page: Option<usize>,

    /// How many items per page. Defaults to 25.
    pub per_page: Option<usize>,

    /// The sort mode. Defaults to None.
    pub sort: Option<Sort>,

    /// The sort direction. Defaults to None.
    pub dir: Option<SortMode>,

    /// Search filters. Defaults to an empty array.
    /// Note that this will actually get deserialized to `Vec<(String, Vec<String>)>`.
    pub filters: Option<String>,
}

/// Search Packages
///
/// Search packages by a query string
#[utoipa::path(
    get,
    path = "/api/v1/packages/search",
    tag = "Packages",
    params(
        ("q" = Option<String>, Query, description = "The query string"),
        ("page" = Option<i64>, Query, description = "The current page (zero-based indexed) - defaults to 0"),
        ("per_page" = Option<i64>, Query, description = "How many items per page - defaults to 25"),
    ),
    responses(
        (status = 200, description = "Method returned ok", body = SearchResults),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn search_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(SearchQuery {
        q,
        page,
        per_page,
        sort,
        dir,
        filters,
    }): Query<SearchQuery>,
) -> Result<Json<SearchResults>> {
    let mut conn = state.pool.get().await?;
    let page = page.unwrap_or(1).max(1);
    let per_page = per_page.unwrap_or(25).min(MAX_PER_PAGE).max(1);
    let filters =
        serde_json::from_str::<Vec<(String, Vec<String>)>>(&filters.unwrap_or("[]".into()))?;
    let mut facets = Vec::new();

    match get_user_from_req(&jar, &headers, &mut conn).await {
        Ok(user) => {
            if !user.admin {
                facets.push(Facet::Manual(format!(
                    "{} OR {}",
                    Facet::Visibility(PackageVisibility::Public).into_filter_string(),
                    Facet::Author(user.id).into_filter_string()
                )))
            }
        }

        Err(_) => facets.push(Facet::Visibility(PackageVisibility::Public)),
    }

    for item in filters {
        facets.push(Facet::parse(item)?);
    }

    let mut real_sort = None;

    if let Some(sort) = sort {
        if let Some(dir) = dir {
            real_sort = Some((sort, dir));
        } else {
            real_sort = Some((sort, Default::default()));
        }
    } else if let Some(dir) = dir {
        real_sort = Some((Default::default(), dir));
    }

    Ok(Json(
        state
            .search
            .search(q.unwrap_or_default(), facets, page, per_page, real_sort)
            .await?,
    ))
}
