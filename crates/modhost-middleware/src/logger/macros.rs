//! Logging middleware utility macros

/// Log something with midlog.
#[macro_export]
macro_rules! midlog_log {
    ($prefix: expr, $route: expr, $status: expr, $time: expr) => {
        let time = colored::Colorize::bright_blue(format!("({} ms)", $time).as_str());

        tracing::event!(
            target: "modhost::logging",
            tracing::Level::INFO,
            "{} {} {} {}",
            colored::Colorize::cyan($prefix),
            colored::Colorize::magenta(format!("{}", $route).as_str()),
            $status,
            time,
        );
    };
}
