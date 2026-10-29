//! Models module
//!
//! Provides ONNX model loading, Demucs model wrapper, ensemble logic, and model
//! registry.

pub mod demucs;
pub mod ensemble;
pub mod onnx;
pub mod registry;

// Re-exports for convenience
pub use demucs::DemucsModel;
pub use ensemble::{EnsembleStrategy, ModelEnsemble};
pub use onnx::OnnxModel;
pub use registry::ModelRegistry;
