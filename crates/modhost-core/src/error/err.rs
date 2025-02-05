//! The error type.

#[cfg(feature = "axum")]
use super::AxumError;
use thiserror::Error;

/// ModHost's error type, which uses [`thiserror`], wrapping many crates' error
/// types, and providing some extra for custom responses.
#[derive(Debug, Error)]
pub enum AppError {
    /// An error with the async database pool occured.
    #[error(transparent)]
    #[cfg(feature = "diesel-async")]
    Pool(#[from] diesel_async::pooled_connection::deadpool::PoolError),

    /// An error with the synchronous database pool occured.
    #[deprecated]
    #[error(transparent)]
    #[cfg(feature = "diesel")]
    SyncPool(#[from] diesel::r2d2::PoolError),

    /// An error with a GitHub API client occured.
    #[error(transparent)]
    #[cfg(feature = "octocrab")]
    GitHub(#[from] octocrab::Error),

    /// An error parsing a URL occured.
    #[error(transparent)]
    #[cfg(feature = "url")]
    Url(#[from] url::ParseError),

    /// An error with the database occured.
    #[error(transparent)]
    #[cfg(feature = "diesel")]
    Database(#[from] diesel::result::Error),

    /// An error with [`axum`] occured.
    #[error(transparent)]
    #[cfg(feature = "axum")]
    Axum(#[from] axum::Error),

    /// An error with [`axum::http`] occured.
    #[error(transparent)]
    #[cfg(feature = "axum")]
    AxumHttp(#[from] axum::http::Error),

    /// An error converting a header from [`reqwest`] to a string occured.
    #[error(transparent)]
    #[cfg(feature = "reqwest")]
    Header(#[from] reqwest::header::ToStrError),

    /// An error parsing a header value occured.
    #[error(transparent)]
    #[cfg(feature = "axum")]
    HeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    /// An error with [`serde_json`] occured.
    #[error(transparent)]
    #[cfg(feature = "serde-json")]
    Json(#[from] serde_json::Error),

    /// An error with [`serde_yaml`] occured.
    #[error(transparent)]
    #[cfg(feature = "serde-yaml")]
    Yaml(#[from] serde_yaml::Error),

    /// An error serializing toml occured.
    #[error(transparent)]
    #[cfg(feature = "toml")]
    TomlSer(#[from] toml::ser::Error),

    /// An error deserializing toml occured.
    #[error(transparent)]
    #[cfg(feature = "toml")]
    TomlDe(#[from] toml::de::Error),

    /// An error involving environment variables occured.
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    /// An error with [`dotenvy`] occured.
    #[error(transparent)]
    #[cfg(feature = "dotenvy")]
    Dotenv(#[from] dotenvy::Error),

    /// An error created using the [`anyhow`] crate occured.
    #[error(transparent)]
    #[cfg(feature = "anyhow")]
    Anyhow(#[from] anyhow::Error),

    /// An IO error occured.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An error parsing a [`std::net::SocketAddr`] occured.
    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),

    /// An error parsing multipart form data occured.
    #[error(transparent)]
    #[cfg(feature = "axum")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    /// An error joining threads occured.
    #[error(transparent)]
    #[cfg(feature = "tokio")]
    Join(#[from] tokio::task::JoinError),

    /// An error with [`reqwest`] occured.
    #[error(transparent)]
    #[cfg(feature = "reqwest")]
    Http(#[from] reqwest::Error),

    /// An error configuring [`jsglue`] occured.
    #[error(transparent)]
    #[cfg(feature = "glue")]
    Glue(#[from] jsglue::config::GlueConfigBuilderError),

    /// An error initializing the database occured.
    #[error(transparent)]
    #[cfg(feature = "diesel-async")]
    DbInit(#[from] diesel_async::pooled_connection::deadpool::BuildError),

    /// An error validating semver occured.
    #[error(transparent)]
    #[cfg(feature = "semver")]
    SemVer(#[from] semver::Error),

    /// A configuration parsing error occured.
    #[error(transparent)]
    #[cfg(feature = "config")]
    Config(#[from] config::ConfigError),

    /// An error with S3 occured.
    #[error(transparent)]
    #[cfg(feature = "s3")]
    S3(#[from] s3::error::S3Error),

    /// An error creating S3 credentials occured.
    #[error(transparent)]
    #[cfg(feature = "s3")]
    S3Creds(#[from] s3::creds::error::CredentialsError),

    /// An error with zip files occured.
    #[error(transparent)]
    #[cfg(feature = "zip")]
    Zip(#[from] zip::result::ZipError),

    /// An error with persisting temporary files occured.
    #[error(transparent)]
    #[cfg(feature = "tempfile")]
    TempFile(#[from] tempfile::PersistError),

    /// An error parsing an integer occured.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// An error parsing a date occured.
    #[error(transparent)]
    #[cfg(feature = "chrono")]
    ParseDate(#[from] chrono::ParseError),

    /// An error with Meilisearch occured.
    #[error(transparent)]
    #[cfg(feature = "meilisearch")]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),

    /// An error with badge generation occured.
    #[error(transparent)]
    #[cfg(feature = "rsbadges")]
    Badge(#[from] rsbadges::BadgeError),

    /// An error with OpenTelemetry logging.
    #[error(transparent)]
    #[cfg(feature = "logging")]
    OpenTelemetryLog(#[from] opentelemetry_sdk::logs::LogError),

    /// An error with OpenTelemetry.
    #[error(transparent)]
    #[cfg(feature = "logging")]
    OpenTelemetry(#[from] opentelemetry_sdk::error::Error),

    /// An error with OpenTelemetry metrics.
    #[error(transparent)]
    #[cfg(feature = "logging")]
    OpenTelemetryMetrics(#[from] opentelemetry_sdk::metrics::MetricError),

    /// An error with the OpenTelemetry exporter.
    #[error(transparent)]
    #[cfg(feature = "logging")]
    OpenTelemetryExporter(#[from] opentelemetry_otlp::Error),

    /// An error with OpenTelemetry tracing.
    #[error(transparent)]
    #[cfg(feature = "logging")]
    OpenTelemetryTrace(#[from] opentelemetry::trace::TraceError),

    /// A token was missing.
    #[error("Missing required token header or cookie!")]
    MissingToken,

    /// A user could not be found.
    #[error("Unknown user!")]
    UnknownUser,

    /// A resource could not be found.
    #[error("Resource not found!")]
    NotFound,

    /// The name of a form data field could not be found!
    #[error("Missing field name!")]
    MissingFieldName,

    /// A user doesn't have access to a resource.
    #[error("You do not have access to that resource!")]
    NoAccess,

    /// Invalid facet data.
    #[error("Invalid facet data for type {0}: {1}")]
    InvalidFacetData(String, String),

    /// Unknown facet type.
    #[error("Unknown facet type: {0}")]
    UnknownFacetType(String),

    /// Invalid image file
    #[error("Invalid image file!")]
    InvalidImageFile,

    /// A required field was missing!
    #[error("Missing field: {0}")]
    MissingField(String),

    /// How did we get here?
    #[error("An unknown error occured.")]
    Unknown,

    /// No versions published.
    #[error("The project doesn't have any versions published!")]
    NoVersions,

    /// Couldn't find the right logo for a badge.
    #[error("Failed to find logo: {0}")]
    NoLogo(String),
}

#[cfg(feature = "axum")]
impl super::HasCode for AppError {
    fn code(&self) -> u16 {
        match self {
            Self::Multipart(_)
            | Self::ParseInt(_)
            | Self::MissingField(_)
            | Self::MissingFieldName
            | Self::InvalidFacetData(_, _)
            | Self::UnknownFacetType(_)
            | Self::InvalidImageFile
            | Self::NoLogo(_) => 400,

            Self::MissingToken => 401,
            Self::NoAccess => 403,
            Self::NotFound | Self::UnknownUser | Self::NoVersions => 404,
            _ => 500,
        }
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.as_response()
    }
}

#[cfg(feature = "axum")]
///  A trait to fix an error to use our error type.
pub trait FixError<T> {
    /// Fix the error!
    fn fix_err(self) -> Result<T, axum::response::Response>;
}

#[cfg(feature = "axum")]
impl<T, E: Into<AppError>> FixError<T> for Result<T, E> {
    fn fix_err(self) -> Result<T, axum::response::Response> {
        self.map_err(|v| v.into().as_response())
    }
}
