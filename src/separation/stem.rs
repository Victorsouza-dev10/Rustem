//! Stem definitions for audio separation

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::audio::io::AudioBuffer;

/// Audio stem types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stem {
    /// Vocal stem
    Vocals,
    /// Drum stem
    Drums,
    /// Bass stem
    Bass,
    /// Piano stem
    Piano,
    /// Other instruments stem
    Other,
}

impl Stem {
    /// Get all stem variants
    pub fn all() -> [Stem; 5] {
        [Stem::Vocals, Stem::Drums, Stem::Bass, Stem::Piano, Stem::Other]
    }

    /// Get stem name as string
    pub fn name(&self) -> &'static str {
        match self {
            Stem::Vocals => "vocals",
            Stem::Drums => "drums",
            Stem::Bass => "bass",
            Stem::Piano => "piano",
            Stem::Other => "other",
        }
    }

    /// Get stem display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Stem::Vocals => "Vocals",
            Stem::Drums => "Drums",
            Stem::Bass => "Bass",
            Stem::Piano => "Piano",
            Stem::Other => "Other",
        }
    }

    /// Get stem color for UI
    pub fn color(&self) -> &'static str {
        match self {
            Stem::Vocals => "#FF6B6B",
            Stem::Drums => "#4ECDC4",
            Stem::Bass => "#45B7D1",
            Stem::Piano => "#96CEB4",
            Stem::Other => "#FFEAA7",
        }
    }

    /// Get all stems as slice
    pub fn all_stems() -> [Stem; 5] {
        [Stem::Vocals, Stem::Drums, Stem::Bass, Stem::Piano, Stem::Other]
    }
}

impl std::fmt::Display for Stem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for Stem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vocals" | "vocal" | "voice" => Ok(Stem::Vocals),
            "drums" | "drum" | "percussion" => Ok(Stem::Drums),
            "bass" | "bass guitar" => Ok(Stem::Bass),
            "piano" | "keys" | "keyboard" => Ok(Stem::Piano),
            "other" | "others" | "rest" => Ok(Stem::Other),
            _ => Err(format!("Unknown stem: {}", s)),
        }
    }
}

/// Output from stem separation
#[derive(Debug, Clone)]
pub struct StemOutputs {
    /// Separated stems
    pub stems: std::collections::HashMap<Stem, AudioBuffer>,
    /// Separation metadata
    pub metadata: SeparationMetadata,
}

impl StemOutputs {
    /// Create new stem outputs
    pub fn new(
        stems: std::collections::HashMap<Stem, AudioBuffer>,
        metadata: SeparationMetadata,
    ) -> Self {
        Self { stems, metadata }
    }

    /// Get stem audio buffer
    pub fn get_stem(&self, stem: Stem) -> Option<&AudioBuffer> {
        self.stems.get(&stem)
    }

    /// Get mutable stem
    pub fn get_stem_mut(&mut self, stem: Stem) -> Option<&mut AudioBuffer> {
        self.stems.get_mut(&stem)
    }

    /// Get all stems
    pub fn stems(&self) -> &std::collections::HashMap<Stem, AudioBuffer> {
        &self.stems
    }

    /// Get stems in order
    pub fn stems_ordered(&self) -> Vec<(Stem, &AudioBuffer)> {
        let order = [Stem::Vocals, Stem::Drums, Stem::Bass, Stem::Piano, Stem::Other];
        order
            .iter()
            .filter_map(|s| self.stems.get(s).map(|a| (*s, a)))
            .collect()
    }

    /// Get mutable stems in order
    pub fn stems_ordered_mut(&mut self) -> Vec<(Stem, &mut AudioBuffer)> {
        // TODO: Implement proper mutable stem access in later SPEC
        // For now, return empty vec to avoid borrow checker issues
        Vec::new()
    }

    /// Get number of stems
    pub fn len(&self) -> usize {
        self.stems.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.stems.is_empty()
    }
}

/// Metadata for separation results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeparationMetadata {
    /// Input file path
    pub input_path: Option<String>,
    /// Input duration in seconds
    pub input_duration_sec: f32,
    /// Processing time in seconds
    pub processing_time_sec: f32,
    /// Model version used
    pub model_version: String,
    /// Sample rate used
    pub sample_rate: u32,
    /// Real-time factor (processing_time / audio_duration)
    pub rtf: f32,
    /// Per-stem metrics
    pub stem_metrics: std::collections::HashMap<String, StemMetrics>,
}

/// Per-stem quality metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StemMetrics {
    /// Signal-to-Distortion Ratio (dB)
    pub sdr: Option<f32>,
    /// Signal-to-Interference Ratio (dB)
    pub sir: Option<f32>,
    /// Signal-to-Artifacts Ratio (dB)
    pub sar: Option<f32>,
}

impl Default for SeparationMetadata {
    fn default() -> Self {
        Self {
            input_path: None,
            input_duration_sec: 0.0,
            processing_time_sec: 0.0,
            model_version: String::new(),
            sample_rate: 44100,
            rtf: 0.0,
            stem_metrics: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_from_str() {
        assert_eq!("vocals".parse::<Stem>(), Ok(Stem::Vocals));
        assert_eq!("drums".parse::<Stem>(), Ok(Stem::Drums));
        assert_eq!("bass".parse::<Stem>(), Ok(Stem::Bass));
        assert_eq!("piano".parse::<Stem>(), Ok(Stem::Piano));
        assert_eq!("other".parse::<Stem>(), Ok(Stem::Other));
        assert!("invalid".parse::<Stem>().is_err());
    }

    #[test]
    fn test_stem_display() {
        assert_eq!(Stem::Vocals.to_string(), "vocals");
        assert_eq!(Stem::Drums.to_string(), "drums");
    }

    #[test]
    fn test_stem_all() {
        let stems = Stem::all();
        assert_eq!(stems.len(), 5);
        assert_eq!(stems[0], Stem::Vocals);
        assert_eq!(stems[4], Stem::Other);
    }

    #[test]
    fn test_stem_outputs() {
        use std::collections::HashMap;
        let mut stems = std::collections::HashMap::new();
        stems.insert(Stem::Vocals, crate::audio::io::AudioBuffer::new(vec![1.0; 44100], 44100, 1));

        let outputs = StemOutputs::new(stems, Default::default());
        assert_eq!(outputs.len(), 1);
        assert!(outputs.get_stem(Stem::Vocals).is_some());
        assert!(outputs.get_stem(Stem::Drums).is_none());
    }
}
