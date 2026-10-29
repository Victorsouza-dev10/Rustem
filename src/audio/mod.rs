//! Audio processing module
//!
//! Provides audio I/O, preprocessing, and postprocessing functionality.

pub mod io;
pub mod postprocessing;
pub mod preprocessing;

// Re-exports for convenience
pub use io::{AudioFormat, read_audio, write_audio};
pub use postprocessing::{PostprocessingConfig, postprocess};
pub use preprocessing::{PreprocessingConfig, preprocess};
