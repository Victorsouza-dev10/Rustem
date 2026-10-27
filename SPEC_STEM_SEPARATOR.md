# Especificação Técnica: Separador de Stems (Stem Separator)

> **Versão**: 1.0  
> **Data**: 2026-10-27  
> **Baseado em**: Specs Separador de Stems.pdf  
> **Linguagem alvo**: Rust

---

## 1. Visão Geral

### 1.1 Objetivo
Sistema de separação de fontes de áudio (Source Separation) que recebe áudio misto e separa em **stems individuais**:
- **Vocals** (Vocais)
- **Drums** (Bateria)
- **Bass** (Baixo)
- **Piano** (Piano)
- **Other** (Outros instrumentos)

### 1.2 Abordagem Técnica
- **Deep Learning**: Modelos híbridos Transformer + CNN (baseados em Demucs / HTDemucs)
- **Inferência**: ONNX Runtime para portabilidade
- **Ensemble**: Múltiplos modelos com voting/averaging
- **Pipeline**: Pré-processamento → Inferência → Pós-processamento

---

## 2. Requisitos Funcionais

### RF-01: Separação de Stems
| Stem | Descrição | Faixa Freq. Principal |
|------|-----------|----------------------|
| Vocals | Voz principal e backing vocals | 80 Hz - 8 kHz |
| Drums | Bateria completa (kick, snare, hi-hat, overheads) | 20 Hz - 16 kHz |
| Bass | Baixo elétrico/acústico, synth bass | 30 Hz - 500 Hz |
| Piano | Piano acústico/elétrico, teclados | 27.5 Hz - 4 kHz |
| Other | Guitarras, sintetizadores, efeitos, ruído | Full range |

### RF-02: Formatos de Entrada Suportados
| Formato | Codec | Sample Rates | Canais |
|---------|-------|--------------|--------|
| WAV | PCM | 44.1, 48, 88.2, 96 kHz | Mono, Stereo |
| FLAC | FLAC | 44.1, 48, 88.2, 96 kHz | Mono, Stereo |
| MP3 | MPEG-1 Layer 3 | 44.1, 48 kHz | Mono, Stereo |

### RF-03: Formatos de Saída
- Mesmo formato do input (preservar) OU formato especificado via flag
- Um arquivo por stem: `vocals.wav`, `drums.wav`, `bass.wav`, `piano.wav`, `other.wav`
- Metadados JSON com métricas de qualidade

---

## 3. Requisitos Não-Funcionais

### 3.1 Métricas de Qualidade (Targets MUSDB18)

| Métrica | Target Mínimo | Target Ideal | Método |
|---------|---------------|--------------|--------|
| **SDR** (Signal-to-Distortion Ratio) | ≥ 7.0 dB | ≥ 8.5 dB | BSS_EVAL v4 |
| **SIR** (Signal-to-Interference Ratio) | ≥ 10.0 dB | ≥ 12.0 dB | BSS_EVAL v4 |
| **SAR** (Signal-to-Artifacts Ratio) | ≥ 8.0 dB | ≥ 10.0 dB | BSS_EVAL v4 |

### 3.2 Performance
| Métrica | Target |
|---------|--------|
| **Real-time Factor (RTF)** | < 1.0x (CPU), < 0.3x (GPU) |
| **Latência por chunk** | < 100 ms |
| **Memória Peak** | < 2 GB (arquivo 10 min) |
| **Throughput** | > 1x real-time em CPU moderna |

### 3.3 Robustez
- Round-trip audio: soma dos stems ≈ original (diferença < -60 dB)
- Duração output = duração input (±1 sample)
- Graceful degradation para áudio corrompido

---

## 4. Arquitetura do Sistema

```
┌─────────────────────────────────────────────────────────────────┐
│                        STEM SEPARATOR                           │
├─────────────────────────────────────────────────────────────────┤
│  INPUT AUDIO ──► [Preprocessing] ──► [Model Ensemble] ──►      │
│  (WAV/FLAC/MP3)   │                    │                    │
│                   ▼                    ▼                    ▼
│            ┌─────────────┐    ┌──────────────┐    ┌────────────┐
│            │ Resample    │    │ Model        │    │ Post-      │
│            │ 44.1/48kHz  │    │ Ensemble     │    │ processing │
│            │ Normalize   │    │ (HTDemucs    │    │            │
│            │ LUFS -23    │    │  x5 models)  │    │ Crossfade  │
│            │ STFT        │    │ Ensemble     │    │ Loudness   │
│            └─────────────┘    └──────────────┘    └────────────┘
│                   │                    │                    │
│                   └────────────────────┴────────────────────┘
│                                    │
│                                    ▼
│                         ┌─────────────────────┐
│                         │  OUTPUT STEMS       │
│                         │ vocals.wav          │
│                         │ drums.wav           │
│                         │ bass.wav            │
│                         │ piano.wav           │
│                         │ other.wav           │
│                         │ metadata.json       │
│                         └─────────────────────┘
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Pipeline de Processamento

### 5.1 Pré-processamento (`audio/preprocessing.rs`)

```rust
pub struct PreprocessingConfig {
    pub target_sample_rate: u32,      // 44100 ou 48000
    pub target_lufs: f32,             // -23.0 (EBU R128)
    pub stft_window_size: usize,      // 4096
    pub stft_hop_size: usize,         // 1024 (75% overlap)
    pub window_type: WindowType,      // Hann
}

pub fn preprocess(audio: AudioBuffer, config: &PreprocessingConfig) 
    -> Result<Spectrogram, Error>
```

**Passos:**
1. **Resampling** → 44.1 kHz ou 48 kHz (rubato, high quality)
2. **Normalização LUFS** → -23 LUFS (EBU R128)
3. **STFT** → Janela Hann 4096, hop 1024 (75% overlap)
3. **Output**: Spectrogram [channels, freq_bins, time_frames]

### 5.2 Model Ensemble (`models/ensemble.rs`)

```rust
pub struct ModelEnsemble {
    models: Vec<Box<dyn StemModel>>,
    strategy: EnsembleStrategy,
}

pub enum EnsembleStrategy {
    Mean,           // Média simples das máscaras
    Median,         // Mediana (robusto a outliers)
    WeightedMean,   // Média ponderada por confiança
}

pub trait StemModel {
    fn forward(&self, spec: &Spectrogram) -> Result<MaskTensor, Error>;
    fn metadata(&self) -> ModelMetadata;
}
```

**Modelos no Ensemble (5 modelos HTDemucs v4):**
| Modelo | Especialização | Peso Default |
|--------|----------------|--------------|
| HTDemucs Vocals | Vocals | 1.0 |
| HTDemucs Drums | Drums | 1.0 |
| HTDemucs Bass | Bass | 1.0 |
| HTDemucs Piano | Piano | 1.0 |
| HTDemucs Other | Other | 1.0 |

### 5.3 Pós-processamento (`audio/postprocessing.rs`)

```rust
pub struct PostprocessingConfig {
    pub crossfade_ms: u32,           // 50ms default
    pub output_lufs: f32,            // -23.0 LUFS
    pub dither: bool,                // true para 16-bit
    pub peak_normalize: bool,        // false default
}

pub fn postprocess(
    stems: StemOutputs, 
    config: &PostprocessingConfig
) -> Result<StemOutputs, Error>
```

**Passos:**
1. **Crossfade** entre chunks (50ms, equal power)
2. **ISTFT** → Reconstrução temporal
3. **Overlap-add** reconstruction
4. **Loudness normalization** → -23 LUFS
4. **Dithering** (TPDF) se output 16-bit
5. **Peak normalization** (opcional, -1 dBFS)

---

## 6. Modelos de Deep Learning

### 6.1 Arquitetura HTDemucs (Hybrid Transformer Demucs)

```
Input: [Batch, 2, Freq, Time]  (Stereo spectrogram)
         │
         ▼
┌─────────────────────────────────────┐
│  Encoder (CNN + Transformer)        │
│  - Conv1d + GLU + Transformer Blocks│
│  - Downsampling progressivo         │
│  - Skip connections                 │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│  Bottleneck (Transformer)           │
│  - Multi-head Self-Attention        │
│  - Feed-forward + LayerNorm         │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│  Decoder (CNN + Transformer)        │
│  - Upsampling + Skip connections    │
│  - Conv1d + GLU + Transformer       │
└─────────────────────────────────────┘
         │
         ▼
Output: [Batch, 5 Stems, Freq, Time]  (Masks: 0.0 - 1.0)
```

**Especificações do Modelo:**
| Parâmetro | Valor |
|-----------|-------|
| Sample Rate | 44.1 kHz |
| STFT Window | 4096 (Hann) |
| Hop Size | 1024 |
| Freq Bins | 2049 |
| Stem Count | 5 |
| Params | ~85M (HTDemucs v4) |
| Formato | ONNX (opset 17) |

### 6.2 Modelos Disponíveis (HTDemucs v4 - Facebook Research)

| Modelo | Arquivo ONNX | Tamanho | Params |
|--------|--------------|---------|--------|
| `htdemucs_vocals.onnx` | vocals | ~340 MB | ~85M |
| `htdemucs_drums.onnx` | drums | ~340 MB | ~85M |
| `htdemucs_bass.onnx` | bass | ~340 MB | ~85M |
| `htdemucs_piano.onnx` | piano | ~340 MB | ~85M |
| `htdemucs_other.onnx` | other | ~340 MB | ~85M |

**Total**: ~1.7 GB (5 modelos × 340 MB)

---

## 7. API Pública

### 7.1 CLI (`cargo run -- ...`)

```bash
# Separação básica
stem-separator separate input.wav -o ./output

# Especificar stems desejados
stem-separator separate input.wav -o ./output --stems vocals,drums,bass

# Formato de saída
stem-separator separate input.wav -o ./output --format flac

# Device
stem-separator separate input.wav -o ./output --device cuda

# Benchmark
stem-separator benchmark --dataset musdb18 --output results.json

# Download modelos
stem-separator model-download --model htdemucs-v4
```

**Argumentos CLI:**
| Flag | Tipo | Default | Descrição |
|------|------|---------|-----------|
| `input` | positional | - | Arquivo de entrada |
| `-o, --output` | string | `./output` | Diretório de saída |
| `--stems` | csv | `all` | Stems a separar |
| `--format` | enum | `wav` | `wav`, `flac`, `mp3` |
| `--device` | enum | `cpu` | `cpu`, `cuda`, `coreml` |
| `--sample-rate` | int | `44100` | Sample rate output |
| `--chunk-duration` | float | `10.0` | Duração chunk (segundos) |

### 7.2 API Rust (`lib.rs`)

```rust
pub struct SeparationConfig {
    pub sample_rate: u32,
    pub chunk_duration_sec: f32,
    pub overlap_ratio: f32,
    pub device: Device,
    pub stems: Vec<Stem>,
    pub output_format: AudioFormat,
    pub output_lufs: f32,
}

pub struct SeparationOutput {
    pub stems: HashMap<Stem, AudioBuffer>,
    pub metadata: SeparationMetadata,
    pub metrics: Option<QualityMetrics>,
}

pub fn separate_audio(
    input: &Path, 
    config: SeparationConfig
) -> Result<SeparationOutput, Error>;

pub fn separate_buffer(
    audio: AudioBuffer, 
    config: SeparationConfig
) -> Result<SeparationOutput, Error>;
```

### 7.3 Tipos Principais

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stem {
    Vocals,
    Drums,
    Bass,
    Piano,
    Other,
}

pub struct AudioBuffer {
    pub samples: Vec<f32>,      // Interleaved: [L, R, L, R, ...]
    pub sample_rate: u32,
    pub channels: u16,          // 1 ou 2
}

pub struct Spectrogram {
    pub data: Array3<Complex<f32>>,  // [channels, freq, time]
    pub sample_rate: u32,
    pub hop_size: usize,
    pub window_size: usize,
}

pub struct StemOutputs {
    pub stems: HashMap<Stem, AudioBuffer>,
}

pub struct SeparationMetadata {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub sample_rate: u32,
    pub duration_sec: f32,
    pub model_version: String,
    pub processing_time_sec: f32,
    pub rtf: f32,  // Real-time factor
}

pub struct QualityMetrics {
    pub sdr: HashMap<Stem, f32>,
    pub sir: HashMap<Stem, f32>,
    pub sar: HashMap<Stem, f32>,
}
```

---

## 8. Estrutura do Projeto (Rust)

```
stem-separator/
├── Cargo.toml
├── Cargo.lock
├── CLAUDE.md
├── README.md
├── .gitignore
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Public API
│   ├── audio/
│   │   ├── mod.rs
│   │   ├── io.rs            # WAV/FLAC/MP3 I/O
│   │   ├── preprocessing.rs # Resample, normalize, STFT
│   │   └── postprocessing.rs# Crossfade, loudness, dither
│   ├── models/
│   │   ├── mod.rs
│   │   ├── onnx.rs          # ONNX Runtime wrapper
│   │   ├── demucs.rs        # HTDemucs model wrapper
│   │   ├── ensemble.rs      # Ensemble logic
│   │   └── registry.rs      # Model loading/caching
│   ├── separation/
│   │   ├── mod.rs
│   │   ├── pipeline.rs      # Main pipeline
│   │   ├── stem.rs          # Stem enum + traits
│   │   └── config.rs        # Config structs
│   ├── metrics/
│   │   ├── mod.rs
│   │   ├── sdr.rs           # SDR (BSS_EVAL v4)
│   │   ├── sir.rs
│   │   └── sar.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── args.rs          # Clap args
│   │   └── commands.rs
│   └── utils/
│       ├── mod.rs
│       ├── errors.rs
│       └── logging.rs
├── models/                  # ONNX models (gitignored)
│   ├── htdemucs_vocals.onnx
│   ├── htdemucs_drums.onnx
│   ├── htdemucs_bass.onnx
│   ├── htdemucs_piano.onnx
│   └── htdemucs_other.onnx
├── tests/
│   ├── integration_tests.rs
│   ├── metrics_tests.rs
│   └── fixtures/
├── benches/
│   └── separation_bench.rs
├── examples/
│   └── basic_separation.rs
└── .github/workflows/ci.yml
```

---

## 9. Dependências (Cargo.toml)

```toml
[package]
name = "stem-separator"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[dependencies]
# Audio I/O
hound = "3.5"
lewton = "0.11"
mp3lame-encoder = "0.4"
symphonia = { version = "0.5", features = ["default"] }

# DSP
daspect = "0.6"
rubato = "0.15"
realfft = "0.5"
num-complex = "0.4"
ndarray = { version = "0.16", features = ["blas"] }

# ML/ONNX
ort = { version = "1.17", features = ["load-dynamic"] }
ndarray = { version = "0.16", features = ["blas"] }

# CLI
clap = { version = "4.5", features = ["derive", "env"] }
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Utils
rayon = "1.10"
tempfile = "3.8"
walkdir = "2.5"
indicatif = "0.17"

[dependencies.ort]
version = "1.17"
features = ["load-dynamic"]

[features]
default = []
cuda = ["ort/cuda"]
coreml = ["ort/coreml"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[profile.bench]
debug = true
```

---

## 10. Modelos Pré-treinados

### Download Automático
```rust
pub async fn download_models(
    cache_dir: &Path, 
    version: &str  // "htdemucs-v4"
) -> Result<Vec<PathBuf>, Error>
```

**URLs de Download (HTDemucs v4 - Facebook Research):**
- Base URL: `https://github.com/facebookresearch/demucs/releases/download/v4.0/`
- Modelos: `htdemucs_vocals.onnx`, `htdemucs_drums.onnx`, etc.
- Checksums SHA256 verificados no código

**Cache Location:**
- Linux: `~/.cache/stem-separator/models/`
- macOS: `~/Library/Caches/stem-separator/models/`
- Windows: `%LOCALAPPDATA%\stem-separator\models\`

---

## 10. Critérios de Aceite (Acceptance Criteria)

| ID | Critério | Target | Verificação |
|----|----------|--------|-------------|
| AC-01 | SDR médio (MUSDB18 test) | ≥ 7.0 dB | `cargo run -- benchmark` |
| AC-02 | SIR médio | ≥ 10.0 dB | `cargo run -- benchmark` |
| AC-03 | SAR médio | ≥ 8.0 dB | `cargo run -- benchmark` |
| AC-04 | Real-time Factor (CPU) | < 1.0x | Benchmark |
| AC-05 | Latência chunk | < 100 ms | Log inference |
| AC-04 | Memória peak | < 2 GB | Monitoramento |
| AC-05 | Formatos suportados | WAV, FLAC, MP3 | Teste manual |
| AC-06 | Stems output | 5 arquivos | Verificar arquivos |
| AC-07 | Round-trip audio | Diferença < -60dB | Teste sintético |
| AC-08 | CLI funciona | `separate` roda | `cargo run -- separate` |
| AC-09 | Benchmark roda | `benchmark` roda | `cargo run -- benchmark` |
| AC-10 | CI passa | 100% testes | GitHub Actions |

---

## 11. Riscos e Mitigações

| Risco | Prob. | Impacto | Mitigação |
|-------|-------|---------|-----------|
| Modelos ONNX indisponíveis | Alta | Crítico | Download automático com múltiplos mirrors |
| Performance CPU lenta | Média | Alto | Feature `cuda` opcional; batch size tuning |
| Licença modelos | Baixa | Crítico | Verificar licença MIT/Apache HTDemucs |
| Qualidade < target | Média | Alto | Ensemble 5 modelos; fine-tuning futuro |
| Dependências imaturas | Baixa | Médio | Fallback pyo3 se necessário |

---

## 12. Roadmap de Implementação

| Fase | Duração | Entregável |
|------|---------|------------|
| **Fase 1**: Setup | 1 semana | Cargo project, CI, logging |
| **Fase 2**: Audio I/O + Preproc | 1-2 sem | I/O, resample, STFT, tests |
| **Fase 3**: Models + ONNX | 1 semana | ONNX wrapper, model loading |
| **Fase 4**: Pipeline | 1-2 sem | Pipeline, chunking, masks |
| **Fase 5**: Post-proc + Output | 1 semana | Crossfade, loudness, output |
| **Fase 6**: CLI + Métricas | 1 semana | CLI, SDR/SIR/SAR, benchmark |
| **Fase 7**: Testes + Validação | 1 semana | Tests, benchmarks, MUSDB18 |

**Total estimado**: 7-8 semanas

---

## 13. Convenções de Código (CLAUDE.md)

```markdown
# Convenções do Projeto stem-separator

- **Edition**: 2021
- **Formatting**: `cargo fmt` (padrão)
- **Linting**: `cargo clippy -D warnings`
- **Error Handling**: `anyhow::Result<T>` + `thiserror` para tipos customizados
- **Async**: `tokio` apenas para I/O (download); DSP síncrono
- **Logging**: `tracing` com spans estruturados
- **Testes**: `#[cfg(test)]` inline + `tests/` integração
- **Docs**: `///` em todos os `pub` items
- **Versionamento**: SemVer, `Cargo.lock` commitado
- **Features**: `default = []`, `cuda`, `coreml`
```

---

## 14. Referências

1. **HTDemucs**: [Hybrid Transformer Demucs](https://github.com/facebookresearch/demucs) - Facebook Research
2. **MUSDB18**: [Multi-track Dataset](https://sigsep.github.io/datasets/musdb.html)
3. **BSS_EVAL v4**: [Evaluation Metrics](https://github.com/sigsep/bsseval)
4. **ONNX Runtime**: [ort crate](https://github.com/pykeio/ort)
5. **EBU R128**: Loudness normalization standard
6. **Demucs v4**: [Paper](https://arxiv.org/abs/2306.06975) - Hybrid Transformer Demucs

---

## 14. Glossário

| Termo | Definição |
|-------|-----------|
| **Stem** | Faixa de áudio isolada (vocals, drums, etc.) |
| **SDR** | Signal-to-Distortion Ratio - qualidade geral |
| **SIR** | Signal-to-Interference Ratio - separação de fontes |
| **SAR** | Signal-to-Artifacts Ratio - ausência de artefatos |
| **RTF** | Real-Time Factor (tempo processamento / duração áudio) |
| **LUFS** | Loudness Units Full Scale (EBU R128) |
| **STFT** | Short-Time Fourier Transform |
| **ISTFT** | Inverse STFT |
| **Overlap-Add** | Método de reconstrução temporal |
| **Crossfade** | Transição suave entre chunks |
| **Ensemble** | Combinação de múltiplos modelos |

---

---

*Documento gerado a partir da especificação técnica original (Specs Separador de Stems.pdf) para implementação em Rust.*