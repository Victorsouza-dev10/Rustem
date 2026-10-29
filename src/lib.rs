//! Stem Separator - Audio source separation using HTDemucs ONNX models
//!
//! High-quality audio stem separation using HTDemucs (Hybrid Transformer
//! Demucs) models via ONNX Runtime. Supports separation of vocals, drums, bass,
//! piano, and other instruments from mixed audio.
//!
//! # Example
//! ```
//! use std::path::Path;
//!
//! use stem_separator::{SeparationConfig, Stem, separate_audio};
//!
//! let config = SeparationConfig::default();
//! let output = separate_audio(Path::new("input.wav"), config)?;
//! for (stem, audio) in output.stems {
//!     println!("{:?}: {} samples", stem, audio.samples.len());
//! }
//! ```
//!
//! # Features
//! - `cuda` - Enable CUDA acceleration via ORT
//! - `coreml` - Enable CoreML acceleration (macOS)

// Public modules
pub mod audio;
pub mod cli;
pub mod metrics;
pub mod models;
pub mod separation;
pub mod utils;

// Re-exports - Main API
pub use audio::io::AudioBuffer;
pub use separation::{
    QualityMetrics, SeparationConfig, SeparationMetadata, SeparationOutput, Stem, separate_audio,
    separate_buffer,
};
pub use utils::errors::StemError;

// Version info
/// Versão do crate (lida do Cargo.toml em build time)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Versão do modelo HTDemucs utilizada
pub const MODEL_VERSION: &str = "htdemucs-v4";

/// Sample rate padrão para processamento (Hz)
pub const DEFAULT_SAMPLE_RATE: u32 = 44100;

/// Duração padrão do chunk para processamento (segundos)
pub const DEFAULT_CHUNK_DURATION: f32 = 10.0;

/// Overlap ratio padrão para processamento em chunks (0.0 - 1.0)
pub const DEFAULT_OVERLAP_RATIO: f32 = 0.75;

/// LUFS target padrão para normalização (EBU R128)
pub const DEFAULT_LUFS: f32 = -23.0;

/// Formato de saída padrão
pub const DEFAULT_OUTPUT_FORMAT: &str = "wav";

/// Diretório padrão de cache de modelos
pub const MODEL_CACHE_DIR: &str = "stem-separator";

/// STFT window size (amostras em 44.1kHz)
pub const STFT_WINDOW_SIZE: usize = 4096;

/// STFT hop size (amostras em 44.1kHz) - 75% overlap
pub const STFT_HOP_SIZE: usize = 1024;

/// Número de bins de frequência (window_size / 2 + 1)
pub const STFT_FREQ_BINS: usize = 2049;

/// Número de stems output pelos modelos
pub const NUM_STEMS: usize = 5;

/// Nomes dos stems na ordem de saída do modelo
pub const STEM_NAMES: [&str; 5] = ["vocals", "drums", "bass", "piano", "other"];

#[cfg(test)]
mod tests {
    use crate::{
        DEFAULT_CHUNK_DURATION,
        DEFAULT_LUFS,
        DEFAULT_OVERLAP_RATIO,
        DEFAULT_OUTPUT_FORMAT,
        DEFAULT_SAMPLE_RATE,
        MODEL_CACHE_DIR,
        NUM_STEMS,
        STEM_NAMES,
        STFT_FREQ_BINS,
        STFT_HOP_SIZE,
        STFT_WINDOW_SIZE,
        VERSION,
    };

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_SAMPLE_RATE, 44100);
        assert_eq!(DEFAULT_CHUNK_DURATION, 10.0);
        assert_eq!(DEFAULT_OVERLAP_RATIO, 0.75);
        assert_eq!(DEFAULT_LUFS, -23.0);
        assert_eq!(DEFAULT_OUTPUT_FORMAT, "wav");
        assert_eq!(MODEL_CACHE_DIR, "stem-separator");
        assert_eq!(STFT_WINDOW_SIZE, 4096);
        assert_eq!(STFT_HOP_SIZE, 1024);
        assert_eq!(STFT_FREQ_BINS, 2049);
        assert_eq!(NUM_STEMS, 5);
        assert_eq!(STEM_NAMES.len(), 5);
        assert_eq!(STEM_NAMES[0], "vocals");
        assert_eq!(STEM_NAMES[4], "other");
    }
}
