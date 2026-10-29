//! CLI module
//!
//! Command-line interface for stem separation.

pub mod args;
pub mod commands;

// Re-exports for convenience
pub use args::Args;
pub use commands::run;
