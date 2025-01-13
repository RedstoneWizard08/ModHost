//! Utilities for bootstrapping a logger.

use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

/// Converts a [`log::LevelFilter`] to a [`LevelFilter`].
pub fn from_log_level(level: log::LevelFilter) -> LevelFilter {
    match level {
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Trace => LevelFilter::TRACE,
        log::LevelFilter::Warn => LevelFilter::WARN,
    }
}

/// Initializes the file logger.
pub fn init_logger(verbosity: LevelFilter) {
    let mut filter = EnvFilter::from_default_env().add_directive(verbosity.into());

    filter = filter.add_directive("tokio_postgres::connection=warn".parse().unwrap());
    filter = filter.add_directive("tokio_postgres::query=warn".parse().unwrap());
    filter = filter.add_directive("diesel_async_migrations=warn".parse().unwrap());
    filter = filter.add_directive("tokio_util::codec::framed_impl=warn".parse().unwrap());
    filter = filter.add_directive("tokio_tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("want=warn".parse().unwrap());
    filter = filter.add_directive("tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("arboard=warn".parse().unwrap());

    let layer = fmt::layer()
        .pretty()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .without_time();

    registry().with(filter).with(layer).init();
}
