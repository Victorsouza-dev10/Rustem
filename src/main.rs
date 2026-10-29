//! Stem Separator - Audio Stem Separation CLI
//!
//! High-quality audio stem separation using HTDemucs models via ONNX Runtime.

use Rustem::{cli::commands, utils::logging::init_logging};
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize structured logging first
    init_logging();

    // Delegate to CLI commands
    commands::run()
}
