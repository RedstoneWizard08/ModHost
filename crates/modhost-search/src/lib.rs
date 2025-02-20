#![warn(missing_docs)]
//! ModHost's search system, based on Meilisearch using the [`meilisearch_sdk`].

#[macro_use]
extern crate serde;

#[macro_use]
extern crate utoipa;

mod facets;
mod index;
mod models;
mod search;
mod service;
mod setup;

pub use facets::*;
pub use models::*;
pub use search::*;
pub use service::*;

pub use meilisearch_sdk::indexes::Index;

modhost_core::utoipa_types![Sort, SortMode, SearchResults, Facet,];
