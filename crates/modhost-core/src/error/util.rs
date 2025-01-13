//! Error handling utilities.
#![cfg(feature = "axum")]

use axum::{body::Body, response::Response};
use std::fmt::Display;

/// A trait for an object that has an associated error code.
pub trait HasCode {
    /// Get the error code.
    fn code(&self) -> u16;
}

/// An error that can be turned into an axum [`Response`].
pub trait AxumError: Display
where
    Self: Sized + HasCode,
{
    /// Get the [`Response`].
    fn as_response(self) -> Response {
        match Response::builder()
            .status(self.code())
            .body(Body::new(format!("{}", self)))
        {
            Ok(it) => it,
            Err(err) => Response::new(Body::new(format!("Could not create a response: {}", err))),
        }
    }
}

impl<T: Display + HasCode> AxumError for T {}
