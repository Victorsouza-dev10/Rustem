//! Audio postprocessing module
//!
//! Provides post-processing functionality including crossfade, loudness
//! normalization, and dithering.

use ruststft::{IstftBuilder, Spectrogram, Symmetry, Window, WindowFunction};

use crate::{
    audio::io::AudioBuffer,
    separation::Stem,
    utils::errors::{Result, StemError},
};

/// Window types for local window creation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    Bartlett,
    Rectangular,
}

/// Configuration for audio postprocessing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostprocessingConfig {
    /// Crossfade duration in milliseconds
    pub crossfade_ms: u32,
    /// Target output loudness in LUFS (EBU R128)
    pub output_lufs: f32,
    /// Apply dithering when converting to 16-bit
    pub dither: bool,
    /// Apply peak normalization
    pub peak_normalize: bool,
    /// Peak normalization target (dBFS)
    pub peak_normalize_db: f32,
}

impl Default for PostprocessingConfig {
    fn default() -> Self {
        Self {
            crossfade_ms: 50,
            output_lufs: -23.0,
            dither: true,
            peak_normalize: false,
            peak_normalize_db: -1.0,
        }
    }
}

/// Postprocess stem audio buffers
pub fn postprocess(
    stems: &mut std::collections::HashMap<crate::separation::Stem, AudioBuffer>,
    config: &PostprocessingConfig,
) -> Result<()> {
    for (_, audio) in stems.iter_mut() {
        // 1. Apply crossfade between chunks if needed
        if config.crossfade_ms > 0 {
            apply_crossfade(audio, config.crossfade_ms)?;
        }

        // 2. Apply loudness normalization
        normalize_loudness(audio, config.output_lufs)?;

        // 3. Apply peak normalization if enabled
        if config.peak_normalize {
            peak_normalize(audio, config.peak_normalize_db)?;
        }

        // 4. Apply dithering if enabled and converting to 16-bit
        if config.dither {
            apply_dither(audio)?;
        }
    }

    Ok(())
}

/// Apply crossfade to smooth transitions
fn apply_crossfade(audio: &mut AudioBuffer, crossfade_ms: u32) -> Result<()> {
    let samples_per_ms = audio.sample_rate as f32 / 1000.0;
    let crossfade_samples = (crossfade_ms as f32 * samples_per_ms) as usize;

    if crossfade_samples >= audio.frames() / 2 {
        return Ok(()); // Audio too short for crossfade
    }

    // Apply fade-in at start
    for i in 0..crossfade_samples {
        let gain = i as f32 / crossfade_samples as f32;
        let gain = gain * gain; // Equal power crossfade
        for ch in 0..audio.channels {
            let idx = ch as usize + i * audio.channels as usize;
            if idx < audio.samples.len() {
                audio.samples[idx] *= gain;
            }
        }
    }

    // Apply fade-out at end
    let frames = audio.frames();
    for i in 0..crossfade_samples {
        let gain = 1.0 - (i as f32 / crossfade_samples as f32);
        let gain = gain * gain; // Equal power crossfade
        let frame_idx = frames - 1 - i;
        for ch in 0..audio.channels {
            let idx = ch as usize + frame_idx * audio.channels as usize;
            if idx < audio.samples.len() {
                audio.samples[idx] *= gain;
            }
        }
    }

    Ok(())
}

/// Normalize loudness to target LUFS
fn normalize_loudness(audio: &mut AudioBuffer, target_lufs: f32) -> Result<()> {
    let rms = calculate_rms(&audio.samples);
    let current_lufs = 20.0 * rms.max(1e-10).log10() - 0.691;

    let gain_db = target_lufs - current_lufs;
    let gain_linear = 10.0_f32.powf(gain_db / 20.0);

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

    Ok(())
}

/// Apply peak normalization
fn peak_normalize(audio: &mut AudioBuffer, target_db: f32) -> Result<()> {
    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0, f32::max);
    if peak > 0.0 {
        let target_peak = 10.0_f32.powf(target_db / 20.0);
        let gain = target_peak / peak;

        for sample in &mut audio.samples {
            *sample *= gain;
        }
    }
    Ok(())
}

/// Apply TPDF dithering
fn apply_dither(audio: &mut AudioBuffer) -> Result<()> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for sample in &mut audio.samples {
        // TPDF dither: triangular probability density function
        let dither = rng.gen_range(-1.0..1.0) + rng.gen_range(-1.0..1.0);
        *sample += dither * 0.5 / 32768.0; // Scale for 16-bit
    }

    Ok(())
}

/// Calculate RMS of audio samples
fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

/// Reconstruct audio from STFT using ISTFT
pub fn reconstruct_from_stft(
    stft: &Spectrogram<f32>,
    hop_size: usize,
    window_type: WindowType,
) -> Result<Vec<f32>> {
    let frame_len = stft.n_freqs() * 2 - 2;
    let window_fn = window_type_to_rstft(window_type);
    let window = Window::new(window_fn, frame_len, Symmetry::Periodic);
    let mut istft = IstftBuilder::<f32>::default()
        .window(window)
        .hop_size(hop_size)
        .build()
        .map_err(|e| StemError::Dsp(format!("Failed to build ISTFT: {}", e)))?;

    // Convert spectrogram to complex spectrum for ISTFT
    let audio = istft
        .reconstruct(stft)
        .map_err(|e| StemError::Dsp(format!("ISTFT processing failed: {}", e)))?;

    Ok(audio)
}

/// Apply overlap-add reconstruction for chunked processing
pub fn overlap_add_reconstruct(
    chunks: &[Vec<f32>],
    hop_size: usize,
    window_type: WindowType,
) -> Vec<f32> {
    if chunks.is_empty() {
        return Vec::new();
    }

    let chunk_len = chunks[0].len();
    let total_len = (chunks.len() - 1) * hop_size + chunk_len;
    let mut output = vec![0.0; total_len];
    let mut overlap_sum = vec![0.0; total_len];

    // Create window function
    let window = create_window(chunk_len, WindowType::Hann);

    for (i, chunk) in chunks.iter().enumerate() {
        let offset = i * hop_size;
        for (j, &sample) in chunk.iter().enumerate() {
            if offset + j < total_len {
                let idx = offset + j;
                output[idx] += sample * window[j];
                overlap_sum[idx] += window[j];
            }
        }
    }

    // Normalize by overlap sum
    for i in 0..total_len {
        if overlap_sum[i] > 1e-10 {
            output[i] /= overlap_sum[i];
        }
    }

    output
}

/// Create window function
fn create_window(size: usize, window_type: WindowType) -> Vec<f32> {
    let mut window = Vec::with_capacity(size);

    for i in 0..size {
        let x = i as f32 / (size - 1) as f32;
        let value = match window_type {
            WindowType::Hann => 0.5 * (1.0 - (2.0 * std::f32::consts::PI * x).cos()),
            WindowType::Hamming => 0.54 - 0.46 * (2.0 * std::f32::consts::PI * x).cos(),
            WindowType::Blackman => {
                0.42 - 0.5 * (2.0 * std::f32::consts::PI * x).cos()
                    + 0.08 * (4.0 * std::f32::consts::PI * x).cos()
            }
            WindowType::BlackmanHarris => {
                0.35875 - 0.48829 * (2.0 * std::f32::consts::PI * x).cos()
                    + 0.14128 * (4.0 * std::f32::consts::PI * x).cos()
                    - 0.01168 * (6.0 * std::f32::consts::PI * x).cos()
            }
            WindowType::Bartlett => 1.0 - 2.0 * (x - 0.5).abs(),
            WindowType::Rectangular => 1.0,
        };
        window.push(value);
    }

    window
}

/// Convert local WindowType to ruststft WindowFunction
fn window_type_to_rstft(window_type: WindowType) -> WindowFunction {
    match window_type {
        WindowType::Hann => WindowFunction::Hann,
        WindowType::Hamming => WindowFunction::Hamming,
        WindowType::Blackman => WindowFunction::Blackman,
        WindowType::BlackmanHarris => WindowFunction::BlackmanHarris,
        WindowType::Bartlett => WindowFunction::Bartlett,
        WindowType::Rectangular => WindowFunction::Rectangular,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postprocessing_config_default() {
        let config = PostprocessingConfig::default();
        assert_eq!(config.crossfade_ms, 50);
        assert_eq!(config.output_lufs, -23.0);
        assert!(config.dither);
        assert!(!config.peak_normalize);
    }

    #[test]
    fn test_crossfade_short_audio() {
        let mut audio = AudioBuffer::new(vec![1.0; 100], 44100, 1);

        let config = PostprocessingConfig { crossfade_ms: 50, ..Default::default() };

        // Should not panic and handle short audio gracefully
        let result = apply_crossfade(&mut AudioBuffer::new(vec![1.0; 10], 44100, 1), 50);
        assert!(result.is_ok());
    }

    #[test]
    fn test_crossfade_application() {
        let mut audio = AudioBuffer::new(vec![1.0; 10000], 44100, 1);

        let config = PostprocessingConfig { crossfade_ms: 10, ..Default::default() };

        let result = apply_crossfade(&mut audio, 10);
        assert!(result.is_ok());

        // Check that fade-in/out was applied
        assert!(audio.samples[0] < 1.0);
        assert!(audio.samples[9999] < 1.0);
    }

    #[test]
    fn test_peak_normalize() {
        let mut audio = AudioBuffer::new(vec![0.5; 1000], 44100, 1);

        peak_normalize(&mut audio, -1.0);

        let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0, f32::max);
        assert!((peak - 10.0_f32.powf(-1.0 / 20.0)).abs() < 0.01);
    }

    #[test]
    fn test_calculate_rms() {
        let samples = vec![1.0, -1.0, 1.0, -1.0];
        let rms = crate::audio::postprocessing::calculate_rms(&samples);
        assert!((rms - 1.0).abs() < 1e-6);

        let silence = vec![0.0; 100];
        assert_eq!(calculate_rms(&silence), 0.0);
    }

    #[test]
    fn test_create_window() {
        let window = create_window(100, WindowType::Hann);
        assert_eq!(window.len(), 100);

        // Hann window should be 0 at edges, 1 in middle
        assert!((window[0] - 0.0).abs() < 1e-6);
        assert!((window[50] - 1.0).abs() < 0.01);
        assert!((window[99] - 0.0).abs() < 1e-6);
    }
}
