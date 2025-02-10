//! The actual search functionality.

use crate::{Facet, MeiliProject, MeilisearchService, SearchResults, Sort, SortMode};
use itertools::Itertools;
use modhost_core::Result;

impl MeilisearchService {
    /// Perform a search query.
    pub async fn search(
        &self,
        query_str: impl AsRef<str>,
        facets: Vec<Facet>,
        page: usize,
        per_page: usize,
        sort: Option<(Sort, SortMode)>,
    ) -> Result<SearchResults> {
        let index = self.projects();
        let filter = create_filter_string(facets);
        let mut query = index.search();

        query
            .with_page(page)
            .with_hits_per_page(per_page)
            .with_query(query_str.as_ref())
            .with_filter(&filter);

        if let Some((sort, mode)) = sort {
            if sort != Sort::None {
                let sorter = create_very_dumb_sorter((sort, mode));

                if !sorter[0].is_empty() {
                    query.with_sort(sorter);
                }
            }
        }

        let res = query.execute::<MeiliProject>().await?;
        let total = res.total_hits.unwrap_or_default();
        let per_page = res.hits_per_page.unwrap_or_default();
        let results = res.hits.into_iter().map(|v| v.result).collect_vec();

        Ok(SearchResults {
            page: res.page.unwrap_or_default(),
            pages: (total as f64 / per_page as f64).ceil() as usize,
            hits: results.len(),
            total,
            results: results.into_iter().map(|v| v.into_data()).collect_vec(),
        })
    }
}

// idk how to do this better and im fed up with trying at this point
/// Create a sort string from a sort type and a sort direction.
pub const fn create_very_dumb_sorter((sort, mode): (Sort, SortMode)) -> &'static [&'static str] {
    match (sort, mode) {
        (Sort::Name, SortMode::Ascending) => &["name:asc"],
        (Sort::Name, SortMode::Descending) => &["name:desc"],
        (Sort::Published, SortMode::Ascending) => &["created_at:asc"],
        (Sort::Published, SortMode::Descending) => &["created_at:desc"],
        (Sort::Updated, SortMode::Ascending) => &["updated_at:asc"],
        (Sort::Updated, SortMode::Descending) => &["updated_at:desc"],
        (Sort::Downloads, SortMode::Ascending) => &["downloads:asc"],
        (Sort::Downloads, SortMode::Descending) => &["downloads:desc"],

        _ => &[""],
    }
}

/// Create a filter string from a list of facets.
pub fn create_filter_string(facets: Vec<Facet>) -> String {
    facets
        .into_iter()
        .map(Facet::into_filter_string)
        .join(" AND ")
}
