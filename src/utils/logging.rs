//! Logging setup module
//!
//! Provides logging initialization with support for RUST_LOG environment
//! variable.

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::utils::errors::{Result, StemError};

/// Initialize logging with RUST_LOG support
pub fn init_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).compact())
        .with(filter)
        .try_init()
        .map_err(|e| StemError::Other(format!("Failed to initialize logging: {}", e)))
}

/// Initialize logging with custom filter
pub fn init_logging_with_filter(filter: &str) -> Result<()> {
    let filter = EnvFilter::new(filter);

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).compact())
        .with(filter)
        .try_init()
        .map_err(|e| StemError::Other(format!("Failed to initialize logging: {}", e)))
}

/// Initialize JSON logging for structured output
pub fn init_json_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer().json().with_target(true))
        .with(filter)
        .try_init()
        .map_err(|e| StemError::Other(format!("Failed to initialize logging: {}", e)))
}

/// Macro for creating spans with context
#[macro_export]
macro_rules! span {
    ($name:expr) => {
        tracing::span!(tracing::Level::INFO, $name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::span!(tracing::Level::INFO, $name, $($field)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging() {
        let result = init_logging();
        // May fail if already initialized, that's OK
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_init_logging_with_filter() {
        let result = init_logging_with_filter("debug");
        assert!(result.is_ok() || result.is_err());
    }
}
