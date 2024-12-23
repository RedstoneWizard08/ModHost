//! ModHost's error type.

mod err;

#[cfg(feature = "axum")]
mod util;

pub use err::*;

#[cfg(feature = "axum")]
pub use util::*;

/// A wrapper for the [`core::result::Result`] type that
/// uses [`AppError`] as the default error type.
pub type Result<T, E = AppError> = core::result::Result<T, E>;
