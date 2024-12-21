//! Utilities for sanitizing HTML.

/// A trait for something containing HTML that can be sanitized.
pub trait HtmlSanitize {
    /// Sanitize this and return a [`String`].
    fn html_sanitize(&self) -> String;
}

impl HtmlSanitize for String {
    fn html_sanitize(&self) -> String {
        self.replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
    }
}
