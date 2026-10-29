//! Audio preprocessing module
//!
//! Provides audio preprocessing functionality including resampling,
//! normalization, and STFT.

use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType,
    WindowFunction as RubatoWindowFunction,
};
use ruststft::{IstftBuilder, Spectrogram, StftBuilder, Symmetry, Window, WindowFunction};

use crate::{
    audio::io::AudioBuffer,
    utils::errors::{Result, StemError},
};

/// Configuration for audio preprocessing
#[derive(Debug, Clone)]
pub struct PreprocessingConfig {
    /// Target sample rate (Hz)
    pub target_sample_rate: u32,
    /// Target loudness in LUFS (EBU R128)
    pub target_lufs: f32,
    /// STFT window size
    pub stft_window_size: usize,
    /// STFT hop size
    pub stft_hop_size: usize,
    /// Window type for STFT
    pub window_type: WindowFunction,
}

impl Default for PreprocessingConfig {
    fn default() -> Self {
        Self {
            target_sample_rate: 44100,
            target_lufs: -23.0,
            stft_window_size: 4096,
            stft_hop_size: 1024,
            window_type: WindowFunction::Hann,
        }
    }
}

/// Preprocessed audio with STFT representation
#[derive(Debug, Clone)]
pub struct PreprocessedAudio {
    /// Time-domain audio (mono)
    pub audio: AudioBuffer,
    /// STFT representation [channels, freq_bins, time_frames]
    pub stft: Spectrogram<f32>,
    /// Preprocessing configuration used
    pub config: PreprocessingConfig,
}

/// Preprocess audio buffer
pub fn preprocess(audio: AudioBuffer, config: &PreprocessingConfig) -> Result<PreprocessedAudio> {
    // 1. Resample if needed
    let audio = if audio.sample_rate != config.target_sample_rate {
        resample_audio(audio, config.target_sample_rate)?
    } else {
        audio
    };

    // 2. Normalize loudness to target LUFS
    let audio = normalize_loudness(audio, config.target_lufs)?;

    // 3. Convert to mono if stereo (for model input)
    let audio = if audio.channels > 1 {
        audio.to_mono()
    } else {
        audio
    };

    // 4. Compute STFT
    let stft = compute_stft(&audio, &config)?;

    Ok(PreprocessedAudio { audio, stft, config: config.clone() })
}

/// Resample audio to target sample rate using rubato
fn resample_audio(audio: AudioBuffer, target_sample_rate: u32) -> Result<AudioBuffer> {
    if audio.sample_rate == 0 {
        return Err(StemError::Config("Sample rate cannot be zero".to_string()));
    }

    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: RubatoWindowFunction::BlackmanHarris2,
    };

    let ratio = audio.sample_rate as f64 / target_sample_rate as f64;
    let mut resampler = SincFixedIn::<f32>::new(ratio, 1.0, params, audio.frames(), 1)
        .map_err(|e| StemError::Resampling(e.to_string()))?;

    let input = audio.samples;
    let output = resampler
        .process(&[input], None)
        .map_err(|e| StemError::Resampling(e.to_string()))?;

    Ok(AudioBuffer::new(output[0].clone(), target_sample_rate, 1))
}

/// Normalize audio loudness to target LUFS (EBU R128)
fn normalize_loudness(mut audio: AudioBuffer, target_lufs: f32) -> Result<AudioBuffer> {
    // Calculate current loudness (simplified - using RMS as approximation)
    let rms = calculate_rms(&audio.samples);
    let current_lufs = 20.0 * rms.max(1e-10).log10() - 0.691; // Approximate LUFS from RMS

    let gain_db = target_lufs - current_lufs;
    let gain_linear = 10.0_f32.powf(gain_db / 20.0);

    // Apply gain
    for sample in &mut audio.samples {
        *sample *= gain_linear;
    }

    // Prevent clipping
    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0, f32::max);
    if peak > 1.0 {
        for sample in &mut audio.samples {
            *sample /= peak;
        }
    }

    Ok(audio)
}

/// Calculate RMS of audio samples
fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

/// Compute STFT using ruststft
fn compute_stft(audio: &AudioBuffer, config: &PreprocessingConfig) -> Result<Spectrogram<f32>> {
    let samples = &audio.samples;

    // Build STFT processor
    let window = Window::new(config.window_type, config.stft_window_size, Symmetry::Periodic);
    let mut stft = StftBuilder::<f32>::default()
        .window(window)
        .hop_size(config.stft_hop_size)
        .build()
        .map_err(|e| StemError::Dsp(format!("Failed to build STFT: {}", e)))?;

    // Process audio
    let spectrogram = stft.spectrogram(samples);

    Ok(spectrogram)
}

/// Compute ISTFT (inverse STFT) using ruststft
pub fn compute_istft(
    spectrogram: &Spectrogram<f32>,
    hop_size: usize,
    window_type: WindowFunction,
) -> Result<Vec<f32>> {
    let frame_len = spectrogram.n_freqs() * 2 - 2;
    let window = Window::new(window_type, frame_len, Symmetry::Periodic);
    let mut istft = IstftBuilder::<f32>::default()
        .window(window)
        .hop_size(hop_size)
        .build()
        .map_err(|e| StemError::Dsp(format!("Failed to build ISTFT: {}", e)))?;

    // Convert spectrogram to complex spectrum for ISTFT
    // Spectrogram stores data frame-major: n_frames columns of n_freqs complex
    // bins
    let audio = istft
        .reconstruct(spectrogram)
        .map_err(|e| StemError::Dsp(format!("ISTFT processing failed: {}", e)))?;

    Ok(audio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessing_config_default() {
        let config = PreprocessingConfig::default();
        assert_eq!(config.target_sample_rate, 44100);
        assert_eq!(config.target_lufs, -23.0);
        assert_eq!(config.stft_window_size, 4096);
        assert_eq!(config.stft_hop_size, 1024);
    }

    #[test]
    fn test_calculate_rms() {
        let samples = vec![1.0, -1.0, 1.0, -1.0];
        let rms = calculate_rms(&samples);
        assert!((rms - 1.0).abs() < 1e-6);

        let silence = vec![0.0; 100];
        assert_eq!(calculate_rms(&silence), 0.0);
    }

    #[test]
    fn test_normalize_loudness() {
        let samples = vec![0.5; 44100]; // 1 second of 0.5 amplitude
        let audio = AudioBuffer::new(samples, 44100, 1);
        let config = PreprocessingConfig::default();

        let normalized = normalize_loudness(audio, config.target_lufs).unwrap();

        // Check that peak is not > 1.0
        let peak = normalized
            .samples
            .iter()
            .map(|s| s.abs())
            .fold(0.0, f32::max);
        assert!(peak <= 1.0 + 1e-6);
    }
}
