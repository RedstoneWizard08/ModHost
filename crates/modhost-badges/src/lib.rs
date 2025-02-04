#![warn(missing_docs)]
//! ModHost's badge generator and routes.

#[macro_use]
extern crate serde;

#[macro_use]
extern crate utoipa;

pub mod generator;
pub mod logo;
pub mod models;
pub mod routes;

modhost_core::utoipa_types![models::BadgeStyle, models::BadgeOptions,];
