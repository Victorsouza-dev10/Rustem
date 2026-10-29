//! Audio I/O module
//!
//! Provides reading and writing of audio files in various formats.

use std::path::Path;
use std::fs;

use crate::utils::errors::{Result, StemError};

/// Supported audio formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AudioFormat {
    Wav,
    Flac,
    Mp3,
}

impl AudioFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "wav" | "wave" => Some(AudioFormat::Wav),
            "flac" => Some(AudioFormat::Flac),
            "mp3" => Some(AudioFormat::Mp3),
            _ => None,
        }
    }

    /// Detect format from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

/// Audio buffer with samples and metadata
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    /// Interleaved samples: [L, R, L, R, ...] for stereo
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioBuffer {
    /// Create new audio buffer
    pub fn new(samples: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self { samples, sample_rate, channels }
    }

    /// Get duration in seconds
    pub fn duration(&self) -> f32 {
        self.samples.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }

    /// Get number of frames
    pub fn frames(&self) -> usize {
        self.samples.len() / self.channels as usize
    }

    /// Convert to mono by averaging channels
    pub fn to_mono(&self) -> AudioBuffer {
        if self.channels == 1 {
            return self.clone();
        }

        let frames = self.frames();
        let mut mono_samples = Vec::with_capacity(frames);

        for i in 0..frames {
            let mut sum = 0.0;
            for ch in 0..self.channels {
                sum += self.samples[i * self.channels as usize + ch as usize];
            }
            mono_samples.push(sum / self.channels as f32);
        }

        AudioBuffer::new(mono_samples, self.sample_rate, 1)
    }

    /// Convert to stereo by duplicating mono channel
    pub fn to_stereo(&self) -> AudioBuffer {
        if self.channels == 2 {
            return self.clone();
        }

        let frames = self.frames();
        let mut stereo_samples = Vec::with_capacity(frames * 2);

        for i in 0..frames {
            let sample = self.samples[i];
            stereo_samples.push(sample);
            stereo_samples.push(sample);
        }

        AudioBuffer::new(stereo_samples, self.sample_rate, 2)
    }

    /// Ensure buffer has specific number of channels
    pub fn ensure_channels(&self, target_channels: u16) -> AudioBuffer {
        match (self.channels, target_channels) {
            (1, 2) => self.to_stereo(),
            (2, 1) => self.to_mono(),
            (c, t) if c == t => self.clone(),
            _ => {
                // For other conversions, convert to mono then to target
                self.to_mono().ensure_channels(target_channels)
            }
        }
    }
}

/// Read audio file from path
pub fn read_audio(path: &Path) -> Result<AudioBuffer> {
    let format = AudioFormat::from_path(path).ok_or_else(|| {
        StemError::UnsupportedFormat {
            format: path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string(),
        }
    })?;

    match format {
        AudioFormat::Wav => read_wav(path),
        AudioFormat::Flac => read_flac(path),
        AudioFormat::Mp3 => read_mp3(path),
    }
}

/// Read WAV file using hound
fn read_wav(path: &Path) -> Result<AudioBuffer> {
    let mut reader = hound::WavReader::open(path).map_err(|e| StemError::AudioIo(e))?;

    let spec = reader.spec();
    let channels = spec.channels as u16;
    let sample_rate = spec.sample_rate;

    let samples: Result<Vec<f32>> = reader
        .samples::<f32>()
        .collect::<std::result::Result<Vec<f32>, _>>()
        .map_err(|e| StemError::AudioIo(e));

    let samples = samples?;
    Ok(AudioBuffer::new(samples, sample_rate, channels))
}

/// Read FLAC file using lewton
fn read_flac(path: &Path) -> Result<AudioBuffer> {
    let file = std::fs::File::open(path).map_err(StemError::Io)?;
    let mut reader =
        lewton::inside_ogg::OggStreamReader::new(file).map_err(StemError::Flac)?;

    let channels = reader.ident_hdr.audio_channels as u16;
    let sample_rate = reader.ident_hdr.audio_sample_rate;

    let mut samples = Vec::new();

    loop {
        let packet = reader
            .read_dec_packet_itl()
            .map_err(StemError::Flac)?;

        match packet {
            Some(packet_samples) => {
                // Convert i16 to f32 (-1.0 to 1.0)
                for sample in packet_samples {
                    samples.push(sample as f32 / 32768.0);
                }
            }
            None => break,
        }
    }

    Ok(AudioBuffer::new(samples, sample_rate, channels))
}

/// Read MP3 file using symphonia
fn read_mp3(path: &Path) -> Result<AudioBuffer> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::codecs::DecoderOptions;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;

    // Open the file
    let file = fs::File::open(path).map_err(StemError::Io)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Create probe hint
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    // Probe the format
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .map_err(|e| StemError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

    // Get the default track
    let track = probed.format.default_track()
        .ok_or_else(|| StemError::UnsupportedFormat { format: "No audio track found in MP3".to_string() })?;

    // Create decoder
    let decoder_opts = DecoderOptions::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .map_err(|e| StemError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

    // Get sample rate and channels from codec params
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);

    // Decode all frames
    let mut samples = Vec::new();
    let mut sample_buf = None;

    loop {
        let packet = match probed.format.next_packet() {
            Ok(packet) => packet,
            Err(e) => return Err(StemError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))),
        };

        if packet.data.is_empty() {
            break;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Initialize sample buffer if needed
                if sample_buf.is_none() {
                    let spec = *decoded.spec();
                    sample_buf = Some(SampleBuffer::new(decoded.frames() as u64, spec));
                }

                if let Some(buf) = sample_buf.as_mut() {
                    buf.copy_interleaved_ref(decoded);
                    // Convert to f32 and normalize to [-1.0, 1.0]
                    for s in buf.samples() {
                        samples.push(*s);
                    }
                }
            }
            Err(e) => {
                // Skip decode errors
                eprintln!("Warning: MP3 decode error: {}", e);
            }
        }
    }

    Ok(AudioBuffer::new(samples, sample_rate, channels))
}

/// Validate audio buffer before writing
fn validate_audio_buffer(audio: &AudioBuffer) -> Result<()> {
    // Check samples not empty
    if audio.samples.is_empty() {
        return Err(StemError::InvalidArgument("Audio buffer has no samples".to_string()));
    }

    // Check sample rate
    if audio.sample_rate == 0 || audio.sample_rate > 192000 {
        return Err(StemError::InvalidArgument(format!("Invalid sample rate: {}", audio.sample_rate)));
    }

    // Check channels
    if audio.channels != 1 && audio.channels != 2 {
        return Err(StemError::InvalidArgument(format!("Unsupported channel count: {}", audio.channels)));
    }

    // Check samples are finite (no NaN/inf)
    for &sample in &audio.samples {
        if !sample.is_finite() {
            return Err(StemError::InvalidArgument("Audio buffer contains non-finite samples (NaN or inf)".to_string()));
        }
    }

    // Check duration > 0
    if audio.duration() <= 0.0 {
        return Err(StemError::InvalidArgument("Audio buffer has zero or negative duration".to_string()));
    }

    Ok(())
}

/// Write audio buffer to file
pub fn write_audio(path: &Path, audio: &AudioBuffer, format: AudioFormat) -> Result<()> {
    // Validate audio buffer before writing
    validate_audio_buffer(audio)?;

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(StemError::Io)?;
    }

    match format {
        AudioFormat::Wav => write_wav(path, audio),
        AudioFormat::Flac => write_flac(path, audio),
        AudioFormat::Mp3 => write_mp3(path, audio),
    }
}

/// Write WAV file using hound
fn write_wav(path: &Path, audio: &AudioBuffer) -> Result<()> {
    let spec = hound::WavSpec {
        channels: audio.channels,
        sample_rate: audio.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, spec).map_err(|e| StemError::AudioIo(e))?;

    for sample in &audio.samples {
        writer
            .write_sample(*sample)
            .map_err(|e| StemError::AudioIo(e))?;
    }

    writer.finalize().map_err(|e| StemError::AudioIo(e))
}

/// Write FLAC file - placeholder (FLAC encoder not yet implemented)
fn write_flac(path: &Path, audio: &AudioBuffer) -> Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(StemError::Io)?;
    }

    // Validate audio buffer
    validate_audio_buffer(audio)?;

    // TODO: Implement FLAC encoding when flac crate API is confirmed
    Err(StemError::UnsupportedFormat {
        format: "FLAC writing not yet implemented".to_string()
    })
}

/// Write MP3 file - placeholder (MP3 encoder API needs adjustment)
fn write_mp3(path: &Path, audio: &AudioBuffer) -> Result<()> {
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(StemError::Io)?;
    }

    // Validate audio buffer
    validate_audio_buffer(audio)?;

    // TODO: Implement MP3 encoding when mp3lame_encoder API is confirmed
    Err(StemError::UnsupportedFormat {
        format: "MP3 writing not yet implemented".to_string()
    })
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_audio_buffer_creation() {
        let samples = vec![0.0, 1.0, -1.0, 0.5];
        let buffer = AudioBuffer::new(samples.clone(), 44100, 2);

        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.channels, 2);
        assert_eq!(buffer.samples, samples);
    }

    #[test]
    fn test_audio_buffer_mono_stereo() {
        // Create mono with 1 frame (1 sample, 1 channel)
        let mono = AudioBuffer::new(vec![1.0], 44100, 1);
        let stereo = mono.to_stereo();

        assert_eq!(stereo.channels, 2);
        assert_eq!(stereo.frames(), 1);
        assert_eq!(stereo.samples[0], 1.0);
        assert_eq!(stereo.samples[1], 1.0);
    }

    #[test]
    fn test_audio_format_detection() {
        assert_eq!(AudioFormat::from_extension("wav"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("WAV"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("flac"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_extension("mp3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_extension("txt"), None);
    }

    #[test]
    fn test_read_wav_stereo() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Wav).unwrap();

        let read_audio = read_audio(temp.path()).unwrap();
        assert_eq!(read_audio.sample_rate, 44100);
        assert_eq!(read_audio.channels, 2);
        assert_eq!(read_audio.frames(), 44100);
    }

    #[test]
    fn test_read_wav_mono() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100], 44100, 1);
        write_audio(temp.path(), &audio, AudioFormat::Wav).unwrap();

        let read_audio = read_audio(temp.path()).unwrap();
        assert_eq!(read_audio.sample_rate, 44100);
        assert_eq!(read_audio.channels, 1);
        assert_eq!(read_audio.frames(), 44100);
    }

    #[test]
    fn test_read_flac() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Flac).unwrap();

        let read_audio = read_audio(temp.path()).unwrap();
        assert_eq!(read_audio.sample_rate, 44100);
        assert_eq!(read_audio.channels, 2);
    }

    #[test]
    fn test_read_mp3() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Mp3).unwrap();

        let read_audio = read_audio(temp.path()).unwrap();
        assert_eq!(read_audio.sample_rate, 44100);
        assert_eq!(read_audio.channels, 2);
    }

    #[test]
    fn test_write_wav() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Wav).unwrap();

        // Verify file exists
        assert!(temp.path().exists());
    }

    #[test]
    fn test_write_flac() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Flac).unwrap();

        assert!(temp.path().exists());
    }

    #[test]
    fn test_write_mp3() {
        let temp = NamedTempFile::new().unwrap();
        let audio = AudioBuffer::new(vec![0.5; 44100 * 2], 44100, 2);
        write_audio(temp.path(), &audio, AudioFormat::Mp3).unwrap();

        assert!(temp.path().exists());
    }

    #[test]
    fn test_roundtrip_wav() {
        let original = AudioBuffer::new(vec![0.1, -0.2, 0.3, -0.4], 44100, 2);
        let temp = NamedTempFile::new().unwrap();

        write_audio(temp.path(), &original, AudioFormat::Wav).unwrap();
        let roundtrip = read_audio(temp.path()).unwrap();

        assert_eq!(roundtrip.sample_rate, original.sample_rate);
        assert_eq!(roundtrip.channels, original.channels);
        assert_eq!(roundtrip.frames(), original.frames());

        // Check samples match (allowing small floating point differences)
        for (orig, rt) in original.samples.iter().zip(roundtrip.samples.iter()) {
            assert!((orig - rt).abs() < 1e-5);
        }
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(AudioFormat::from_extension("wav"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("WAV"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("wave"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("flac"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_extension("FLAC"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_extension("mp3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_extension("MP3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_extension("txt"), None);
        assert_eq!(AudioFormat::from_extension(""), None);
    }

    #[test]
    fn test_invalid_format_error() {
        let temp = NamedTempFile::new().unwrap();
        let invalid_path = temp.path().with_extension("xyz");
        std::fs::write(&invalid_path, b"dummy").unwrap();

        let result = read_audio(&invalid_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            StemError::UnsupportedFormat { format } => {
                assert_eq!(format, "xyz");
            }
            _ => panic!("Expected UnsupportedFormat error"),
        }
    }

    #[test]
    fn test_audio_buffer_mono_stereo() {
        let mono = AudioBuffer::new(vec![1.0], 44100, 1);
        let stereo = mono.to_stereo();

        assert_eq!(stereo.channels, 2);
        assert_eq!(stereo.frames(), 1);
        assert_eq!(stereo.samples[0], 1.0);
        assert_eq!(stereo.samples[1], 1.0);
    }

    #[test]
    fn test_validate_audio_buffer_empty() {
        let audio = AudioBuffer::new(vec![], 44100, 2);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_audio_buffer_invalid_sample_rate() {
        let audio = AudioBuffer::new(vec![0.5; 100], 0, 2);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());

        let audio = AudioBuffer::new(vec![0.5; 100], 200000, 2);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_audio_buffer_invalid_channels() {
        let audio = AudioBuffer::new(vec![0.5; 100], 44100, 3);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());

        let audio = AudioBuffer::new(vec![0.5; 100], 44100, 0);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_audio_buffer_nan_samples() {
        let audio = AudioBuffer::new(vec![0.5, f32::NAN, 0.5], 44100, 1);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_audio_buffer_inf_samples() {
        let audio = AudioBuffer::new(vec![0.5, f32::INFINITY, 0.5], 44100, 1);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_audio_buffer_zero_duration() {
        let audio = AudioBuffer::new(vec![], 44100, 2);
        let result = validate_audio_buffer(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_audio_creates_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("subdir").join("output.wav");
        let audio = AudioBuffer::new(vec![0.5; 44100], 44100, 1);

        write_audio(&output_path, &audio, AudioFormat::Wav).unwrap();
        assert!(output_path.exists());
    }
}
