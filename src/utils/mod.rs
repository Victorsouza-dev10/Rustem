//! Utilities module
//!
//! Provides error handling, logging, and common utilities.

pub mod errors;
pub mod logging;

// Re-exports for convenience
pub use errors::{Result, ResultExt, StemError};
pub use logging::{init_json_logging, init_logging, init_logging_with_filter};
