#![warn(missing_docs)]
//! Common types and utilities for ModHost.

pub(crate) mod error;

#[cfg(feature = "logging")]
pub mod logger;

#[cfg(feature = "utoipa")]
pub mod utoipa;

#[cfg(feature = "sysinfo")]
pub mod info;

pub use error::*;

/// The time the server started up.
#[cfg(feature = "chrono")]
static mut START_TIME: chrono::DateTime<chrono::Utc> = chrono::DateTime::UNIX_EPOCH;

/// Initialize ModHost's core, setting the internal startup time tracker.
/// This should only ever be called once when the server starts.
#[cfg(feature = "chrono")]
pub fn core_init() {
    unsafe {
        START_TIME = chrono::Utc::now();
    }
}

/// Get the instance uptime.
#[cfg(feature = "chrono")]
pub fn uptime_secs() -> u64 {
    chrono::Utc::now()
        .signed_duration_since(unsafe { START_TIME })
        .num_seconds()
        .try_into()
        .unwrap_or_default()
}

#[cfg(all(feature = "utoipa", feature = "sysinfo"))]
utoipa_types![
    info::SysInfo,
    info::CpuInfo,
    info::DiskInfo,
    info::NetworkInfo
];
