#![warn(missing_docs)]
//! Adds support for [`tokio_tungstenite`] into [`axum`](https://docs.rs/axum).

pub mod fail;
pub mod rejection;
pub mod socket;
pub mod upgrade;
pub mod util;
