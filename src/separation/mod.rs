//! Separation module
//!
//! Provides the main separation pipeline, stem definitions, and configuration.

pub mod config;
pub mod pipeline;
pub mod stem;

// Re-exports for convenience
pub use config::{AudioFormat, Device, SeparationConfig};
pub use pipeline::{
    QualityMetrics, SeparationOutput, SeparationPipeline, separate_audio, separate_buffer,
};
// Also re-export SeparationMetadata from stem
pub use stem::SeparationMetadata;
pub use stem::{SeparationMetadata as StemSeparationMetadata, Stem, StemMetrics, StemOutputs};
