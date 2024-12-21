//! The error type.

use std::num::ParseIntError;
use super::{AxumError, HasCode};
use axum::response::{IntoResponse, Response};
use diesel::r2d2::PoolError as SyncPoolError;
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};
use jsglue::config::GlueConfigBuilderError;
use thiserror::Error;

/// ModHost's error type, which uses [`thiserror`], wrapping many crates' error
/// types, and providing some extra for custom responses.
#[derive(Debug, Error)]
pub enum AppError {
    /// An error with the database pool occured.
    #[error(transparent)]
    Pool(#[from] PoolError),

    /// An error with the synchronous database pool occured.
    #[error(transparent)]
    SyncPool(#[from] SyncPoolError),

    /// An error with a GitHub API client occured.
    #[error(transparent)]
    GitHub(#[from] octocrab::Error),

    /// An error parsing a URL occured.
    #[error(transparent)]
    Url(#[from] url::ParseError),

    /// An error with the database occured.
    #[error(transparent)]
    Database(#[from] diesel::result::Error),

    /// An error with [`axum`] occured.
    #[error(transparent)]
    Axum(#[from] axum::Error),

    /// An error with [`axum::http`] occured.
    #[error(transparent)]
    AxumHttp(#[from] axum::http::Error),

    /// An error converting a header from [`reqwest`] to a string occured.
    #[error(transparent)]
    Header(#[from] reqwest::header::ToStrError),

    /// An error parsing a header value occured.
    #[error(transparent)]
    HeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    /// An error with [`serde_json`] occured.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// An error with [`serde_yaml`] occured.
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),

    /// An error serializing toml occured.
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    /// An error deserializing toml occured.
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    /// An error involving environment variables occured.
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    /// An error with [`dotenvy`] occured.
    #[error(transparent)]
    Dotenv(#[from] dotenvy::Error),

    /// An error created using the [`anyhow`] crate occured.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    /// An IO error occured.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An error parsing a [`std::net::SocketAddr`] occured.
    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),

    /// An error parsing multipart form data occured.
    #[error(transparent)]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    /// An error joining threads occured.
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    /// An error with [`reqwest`] occured.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// An error configuring [`jsglue`] occured.
    #[error(transparent)]
    Glue(#[from] GlueConfigBuilderError),

    /// An error initializing the database occured.
    #[error(transparent)]
    DbInit(#[from] BuildError),

    /// An error validating semver occured.
    #[error(transparent)]
    SemVer(#[from] semver::Error),

    /// A configuration parsing error occured.
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    /// An error with S3 occured.
    #[error(transparent)]
    S3(#[from] s3::error::S3Error),

    /// An error creating S3 credentials occured.
    #[error(transparent)]
    S3Creds(#[from] s3::creds::error::CredentialsError),

    /// An error with zip files occured.
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    /// An error with persisting temporary files occured.
    #[error(transparent)]
    TempFile(#[from] tempfile::PersistError),

    /// An error parsing an integer occured.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    /// An error parsing a date occured.
    #[error(transparent)]
    ParseDate(#[from] chrono::ParseError),

    /// An error with Meilisearch occured.
    #[error(transparent)]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),

    /// A token was missing.
    #[error("Missing required token header or cookie!")]
    MissingToken,

    /// A user could not be found.
    #[error("Unknown user!")]
    UnknownUser,

    /// A resource could not be found.
    #[error("Resource not found!")]
    NotFound,
}

impl HasCode for AppError {
    fn code(&self) -> u16 {
        match self {
            Self::Multipart(_) | Self::ParseInt(_) => 400,
            Self::MissingToken => 401,
            Self::NotFound | Self::UnknownUser => 404,
            _ => 500,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.as_response()
    }
}

///  A trait to fix an error to use our error type.
pub trait FixError<T> {
    /// Fix the error!
    fn fix_err(self) -> Result<T, Response>;
}

impl<T, E: Into<AppError>> FixError<T> for Result<T, E> {
    fn fix_err(self) -> Result<T, Response> {
        self.map_err(|v| v.into().as_response())
    }
}
