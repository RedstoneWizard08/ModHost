#![warn(missing_docs)]
//! A crate providing a lot of the basic types ModHost uses.

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate utoipa;

pub mod github;
pub mod glue;
pub mod macros;
pub mod models;
pub mod state;
pub mod worker;
