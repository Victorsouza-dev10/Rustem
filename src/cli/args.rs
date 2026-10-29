//! CLI arguments module
//!
//! Command-line argument definitions using clap.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::utils::errors::{Result, StemError};

/// Main CLI arguments
#[derive(Debug, Parser)]
#[command(
    name = "Rustem",
    version,
    about = "Audio stem separator using HTDemucs ONNX models"
)]
pub struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Command,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

/// Available commands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Separate audio file into stems
    Separate {
        /// Input audio file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output directory
        #[arg(short, long, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Model to use
        #[arg(short, long, value_name = "MODEL", default_value = "htdemucs-v4")]
        model: String,

        /// Device to use (cpu, cuda, coreml)
        #[arg(long, value_name = "DEVICE", default_value = "cpu")]
        device: String,

        /// Output format (wav, flac, mp3)
        #[arg(long, value_name = "FORMAT", default_value = "wav")]
        format: String,

        /// Number of stems to separate (2, 4, 5)
        #[arg(long, value_name = "N", default_value = "5")]
        stems: usize,
    },

    /// List available models
    Models,

    /// Download model
    Download {
        /// Model name to download
        #[arg(value_name = "MODEL")]
        model: String,
    },

    /// Show version
    Version,
}

impl Args {
    /// Parse arguments from command line
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::try_parse_from(["rustem", "separate", "input.wav"]);
        assert!(args.is_ok());
    }

    #[test]
    fn test_command_factory() {
        Args::command().debug_assert();
    }
}
