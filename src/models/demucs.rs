//! HTDemucs model wrapper
//!
//! Wrapper for HTDemucs (Hybrid Transformer Demucs) model for stem separation.

use std::{collections::HashMap, path::Path};

use ndarray::{Array3, Array4, Axis};
use ruststft::Spectrogram;

use crate::{
    audio::{
        io::AudioBuffer,
        postprocessing::reconstruct_from_stft,
        preprocessing::{PreprocessedAudio, PreprocessingConfig, preprocess},
    },
    models::onnx::{OnnxModel, create_input_tensor, extract_output_tensor},
    separation::{Stem, stem::SeparationMetadata},
    utils::errors::{Result, StemError},
};

/// HTDemucs model wrapper
#[derive(Clone)]
pub struct DemucsModel {
    model: crate::models::onnx::OnnxModel,
    sample_rate: u32,
    config: PreprocessingConfig,
}

impl DemucsModel {
    /// Load Demucs model from ONNX file
    pub fn load<P: AsRef<std::path::Path>>(model_path: P) -> Result<Self> {
        let model = crate::models::onnx::OnnxModel::load(model_path)?;

        Ok(Self { model, sample_rate: 44100, config: Default::default() })
    }

    /// Load with custom configuration
    pub fn load_with_config<P: AsRef<std::path::Path>>(
        model_path: P,
        config: crate::audio::preprocessing::PreprocessingConfig,
    ) -> Result<Self> {
        let model = crate::models::onnx::OnnxModel::load(model_path)?;

        Ok(Self { model, sample_rate: config.target_sample_rate, config })
    }

    /// Separate audio into stems
    pub fn separate(&self, audio: AudioBuffer) -> Result<StemOutputs> {
        // TODO: Implement actual separation logic in later SPEC
        // For now, return empty stems
        let metadata = SeparationMetadata {
            input_path: None,
            input_duration_sec: audio.duration(),
            processing_time_sec: 0.0,
            model_version: "htdemucs-v4".to_string(),
            sample_rate: audio.sample_rate,
            rtf: 0.0,
            stem_metrics: std::collections::HashMap::new(),
        };

        let stems = std::collections::HashMap::new();

        Ok(StemOutputs { stems, metadata })
    }
}

/// Trait for Demucs models to enable trait objects
pub trait DemucsModelTrait: Send + Sync {
    fn separate(
        &self,
        audio: crate::audio::io::AudioBuffer,
    ) -> crate::utils::errors::Result<StemOutputs>;
    fn name(&self) -> &str;
}

/// Wrapper to make DemucsModel implement DemucsModelTrait
pub struct DemucsModelWrapper(pub DemucsModel);

impl DemucsModelTrait for DemucsModelWrapper {
    fn separate(
        &self,
        audio: crate::audio::io::AudioBuffer,
    ) -> crate::utils::errors::Result<StemOutputs> {
        self.0.separate(audio)
    }

    fn name(&self) -> &str {
        "htdemucs"
    }
}

/// Output from stem separation
#[derive(Debug, Clone)]
pub struct StemOutputs {
    pub stems: std::collections::HashMap<Stem, AudioBuffer>,
    pub metadata: SeparationMetadata,
}

impl StemOutputs {
    /// Get stem audio buffer
    pub fn get_stem(&self, stem: Stem) -> Option<&AudioBuffer> {
        self.stems.get(&stem)
    }

    /// Get all stems
    pub fn stems(&self) -> &std::collections::HashMap<Stem, AudioBuffer> {
        &self.stems
    }

    /// Get stems as vector in order
    pub fn stems_ordered(&self) -> Vec<(Stem, &AudioBuffer)> {
        let order = [Stem::Vocals, Stem::Drums, Stem::Bass, Stem::Piano, Stem::Other];
        order
            .iter()
            .filter_map(|s| self.stems.get(s).map(|a| (*s, a)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_outputs() {
        let mut stems = std::collections::HashMap::new();
        stems.insert(Stem::Vocals, AudioBuffer::new(vec![1.0; 44100], 44100, 1));

        let outputs = StemOutputs {
            stems,
            metadata: SeparationMetadata {
                input_path: None,
                input_duration_sec: 10.0,
                processing_time_sec: 5.0,
                model_version: "htdemucs-v4".to_string(),
                sample_rate: 44100,
                rtf: 0.0,
                stem_metrics: std::collections::HashMap::new(),
            },
        };

        assert!(outputs.get_stem(Stem::Vocals).is_some());
        assert!(outputs.get_stem(Stem::Drums).is_none());

        let ordered = outputs.stems_ordered();
        assert_eq!(ordered.len(), 1);
    }
}
