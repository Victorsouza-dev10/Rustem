# Spec 04: lib.rs - API Pública

**Fase**: 1 - Setup & Fundação  
**Componente**: Public API (`src/lib.rs`)  
**Duração estimada**: 30 min  
**Dependência**: SPEC_03_MODULE_STRUCTURE

---

## Objetivo
Criar o `src/lib.rs` com re-exports públicos de toda a API pública do crate.

---

## Código do lib.rs

```rust
//! Stem Separator - Audio source separation using HTDemucs ONNX models
//!
//! High-quality audio stem separation using HTDemucs (Hybrid Transformer Demucs)
//! models via ONNX Runtime. Supports separation of vocals, drums, bass, piano,
//! and other instruments from mixed audio.
//!
//! # Example
//! ```
//! use stem_separator::{separate_audio, SeparationConfig, Stem};
//! use std::path::Path;
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
pub mod models;
pub mod separation;
pub mod metrics;
pub mod cli;
pub mod utils;

// Re-exports - Main API
pub use separation::{
    separate_audio,
    separate_buffer,
    SeparationConfig,
    SeparationOutput,
    Stem,
    AudioBuffer,
    SeparationOutput,
    QualityMetrics,
    SeparationMetadata,
};

pub use audio::AudioBuffer;
pub use utils::errors::StemError;
pub use separation::stem::Stem;
pub use separation::config::SeparationConfig;
pub use separation::SeparationOutput;
pub use separation::QualityMetrics;
pub use separation::SeparationMetadata;
pub use audio::AudioBuffer;
pub use utils::errors::StemError;

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MODEL_VERSION: &str = "htdemucs-v4";

/// Default sample rate for processing (Hz)
pub const DEFAULT_SAMPLE_RATE: u32 = 44100;

/// Default chunk duration for processing (seconds)
pub const DEFAULT_CHUNK_DURATION: f32 = 10.0;

/// Default overlap ratio for chunk processing (0.0 - 1.0)
pub const DEFAULT_OVERLAP_RATIO: f32 = 0.75;

/// Default LUFS target for normalization (EBU R128)
pub const DEFAULT_LUFS: f32 = -23.0;

/// Default output format
pub const DEFAULT_OUTPUT_FORMAT: &str = "wav";

/// Model version used
pub const MODEL_VERSION: &str = "htdemucs-v4";

/// Default model cache directory name
pub const MODEL_CACHE_DIR: &str = "stem-separator";

/// STFT window size (samples at 44.1kHz)
pub const STFT_WINDOW_SIZE: usize = 4096;

/// STFT hop size (samples at 44.1kHz) - 75% overlap
pub const STFT_HOP_SIZE: usize = 1024;

/// Number of frequency bins (window_size / 2 + 1)
pub const STFT_FREQ_BINS: usize = 2049;

/// Number of stems output by models
pub const NUM_STEMS: usize = 5;

/// Stem names in model output order
pub const STEM_NAMES: [&str; 5] = [
    "vocals",
    "drums",
    "bass",
    "piano",
    "other",
];
```

---

## Re-exports Organizados

### Por Módulo

```rust
// audio/
pub use audio::{AudioBuffer, read_audio, write_audio, resample};

// models/
pub use models::{OnnxModel, DemucsModel, ModelEnsemble, ModelRegistry};

// separation/
pub use separation::{
    separate_audio, separate_buffer,
    SeparationConfig, SeparationOutput, SeparationMetadata,
    Stem, AudioBuffer, QualityMetrics,
    SeparationPipeline,
};

// metrics/
pub use metrics::{compute_sdr, compute_sir, compute_sar, QualityMetrics};

// cli/
pub use cli::{run, Args};

// utils/
pub use utils::errors::{StemError, Result};
pub use utils::logging::init_logging;
```

---

## Versionamento

```rust
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MODEL_VERSION: &str = "htdemucs-v4";
```

---

## Constantes Públicas

| Constante | Valor | Descrição |
|-----------|-------|-----------|
| `VERSION` | `env!("CARGO_PKG_VERSION")` | Versão do crate |
| `MODEL_VERSION` | `"htdemucs-v4"` | Versão do modelo |
| `DEFAULT_SAMPLE_RATE` | `44100` | Sample rate padrão (Hz) |
| `DEFAULT_CHUNK_DURATION` | `10.0` | Duração chunk (segundos) |
| `DEFAULT_OVERLAP_RATIO` | `0.75` | Overlap ratio (0.0-1.0) |
| `DEFAULT_LUFS` | `-23.0` | LUFS target (EBU R128) |
| `DEFAULT_OUTPUT_FORMAT` | `"wav"` | Formato saída padrão |
| `MODEL_VERSION` | `"htdemucs-v4"` | Versão modelo |
| `MODEL_CACHE_DIR` | `"stem-separator"` | Dir cache modelos |
| `STFT_WINDOW_SIZE` | `4096` | Window size STFT |
| `STFT_HOP_SIZE` | `1024` | Hop size STFT |
| `STFT_FREQ_BINS` | `2049` | Freq bins (window/2+1) |
| `NUM_STEMS` | `5` | Número de stems |
| `STEM_NAMES` | `[&str; 5]` | Nomes dos stems |

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-04.1 | `lib.rs` compila | `cargo check` |
| AC-04.2 | Re-exports funcionam | `use stem_separator::SeparationConfig;` compila |
| AC-04.3 | Constantes acessíveis | `stem_separator::DEFAULT_SAMPLE_RATE` compila |
| AC-04.4 | Docs geradas | `cargo doc --no-deps` sem warnings |
| AC-04.5 | Version acessível | `stem_separator::VERSION` retorna versão |

---

## Checklist

- [ ] `src/lib.rs` criado com re-exports
- [ ] Constantes públicas definidas
- [ ] Doc comments em todos os re-exports públicos
- [ ] `cargo check` passa
- [ ] `cargo doc --no-deps` sem warnings
- [ ] `use stem_separator::SeparationConfig;` compila em exemplo

---

## Notas

- `lib.rs` é a **única** entrada pública do crate
- Re-exporta **apenas** o que é API pública estável
- Internos (`_private`, `impl` details) **não** exportados
- Doc comments em tudo que é `pub`
- `env!("CARGO_PKG_VERSION")` lê do `Cargo.toml` em build time