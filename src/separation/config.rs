//! Separation configuration

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Device for inference
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Device {
    /// CPU inference
    Cpu,
    /// CUDA GPU inference
    Cuda,
    /// CoreML (Apple Neural Engine)
    CoreML,
}

impl Default for Device {
    fn default() -> Self {
        Device::Cpu
    }
}

impl std::str::FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cpu" => Ok(Device::Cpu),
            "cuda" | "gpu" => Ok(Device::Cuda),
            "coreml" | "ml" => Ok(Device::CoreML),
            _ => Err(format!("Unknown device: {}", s)),
        }
    }
}

/// Audio output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AudioFormat {
    Wav,
    Flac,
    Mp3,
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::Wav
    }
}

impl std::str::FromStr for AudioFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wav" => Ok(AudioFormat::Wav),
            "flac" => Ok(AudioFormat::Flac),
            "mp3" => Ok(AudioFormat::Mp3),
            _ => Err(format!("Unknown audio format: {}", s)),
        }
    }
}

/// Configuration for stem separation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeparationConfig {
    /// Sample rate for processing (Hz)
    pub sample_rate: u32,
    /// Chunk duration in seconds
    pub chunk_duration_sec: f32,
    /// Overlap ratio between chunks (0.0 - 1.0)
    pub overlap_ratio: f32,
    /// Device for inference
    pub device: Device,
    /// Stems to separate (default: all)
    pub stems: Vec<crate::separation::stem::Stem>,
    /// Output format
    pub output_format: AudioFormat,
    /// Output directory
    pub output_dir: Option<String>,
    /// Target loudness in LUFS
    pub output_lufs: f32,
    /// Enable GPU acceleration
    pub use_gpu: bool,
    /// Number of threads for inference
    pub num_threads: Option<usize>,
    /// Model version to use
    pub model_version: String,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for SeparationConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            chunk_duration_sec: 10.0,
            overlap_ratio: 0.75,
            device: Device::default(),
            stems: crate::separation::stem::Stem::all().to_vec(),
            output_format: AudioFormat::default(),
            output_dir: None,
            output_lufs: -23.0,
            use_gpu: false,
            num_threads: None,
            model_version: "htdemucs-v4".to_string(),
            verbose: false,
        }
    }
}

impl SeparationConfig {
    /// Create config for CPU inference
    pub fn cpu() -> Self {
        Self { device: Device::Cpu, ..Default::default() }
    }

    /// Create config for GPU inference
    pub fn cuda() -> Self {
        Self { device: Device::Cuda, use_gpu: true, ..Default::default() }
    }

    /// Create config for CoreML (Apple Silicon)
    pub fn coreml() -> Self {
        Self { device: Device::CoreML, ..Default::default() }
    }

    /// Set specific stems to separate
    pub fn with_stems(mut self, stems: Vec<crate::separation::stem::Stem>) -> Self {
        self.stems = stems;
        self
    }

    /// Set output directory
    pub fn with_output_dir<P: Into<std::path::PathBuf>>(mut self, dir: P) -> Self {
        self.output_dir = Some(dir.into().to_string_lossy().to_string());
        self
    }

    /// Set sample rate
    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    /// Set chunk duration
    pub fn with_chunk_duration(mut self, duration: f32) -> Self {
        self.chunk_duration_sec = duration;
        self
    }

    /// Set overlap ratio
    pub fn with_overlap_ratio(mut self, ratio: f32) -> Self {
        self.overlap_ratio = ratio.clamp(0.0, 0.99);
        self
    }

    /// Set output format
    pub fn with_output_format(mut self, format: AudioFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Set model version
    pub fn with_model_version(mut self, version: String) -> Self {
        self.model_version = version;
        self
    }

    /// Enable verbose logging
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.sample_rate == 0 {
            return Err("Sample rate must be > 0".to_string());
        }

        if self.chunk_duration_sec <= 0.0 {
            return Err("Chunk duration must be > 0".to_string());
        }

        if self.overlap_ratio >= 1.0 || self.overlap_ratio < 0.0 {
            return Err("Overlap ratio must be in [0, 1)".to_string());
        }

        if self.stems.is_empty() {
            return Err("At least one stem must be selected".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SeparationConfig::default();
        assert_eq!(config.sample_rate, 44100);
        assert_eq!(config.chunk_duration_sec, 10.0);
        assert_eq!(config.overlap_ratio, 0.75);
        assert_eq!(config.stems.len(), 5);
    }

    #[test]
    fn test_config_validation() {
        let mut config = SeparationConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid = SeparationConfig::default();
        invalid.sample_rate = 0;
        assert!(invalid.validate().is_err());

        invalid = SeparationConfig::default();
        invalid.overlap_ratio = 1.5;
        assert!(invalid.validate().is_err());

        invalid = SeparationConfig::default();
        invalid.stems.clear();
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = SeparationConfig::default()
            .with_stems(vec![
                crate::separation::stem::Stem::Vocals,
                crate::separation::stem::Stem::Drums,
            ])
            .with_output_dir("/tmp/output")
            .with_sample_rate(48000)
            .with_chunk_duration(5.0)
            .with_overlap_ratio(0.5)
            .with_output_format(crate::separation::config::AudioFormat::Flac)
            .verbose();

        assert_eq!(config.stems.len(), 2);
        assert_eq!(config.output_dir, Some("/tmp/output".to_string()));
        assert_eq!(config.sample_rate, 48000);
        assert_eq!(config.chunk_duration_sec, 5.0);
        assert_eq!(config.overlap_ratio, 0.5);
        assert!(config.verbose);
    }
}
