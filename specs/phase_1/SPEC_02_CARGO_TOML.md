# Spec 02: Cargo.toml - Dependências e Configuração Completa

**Fase**: 1 - Setup & Fundação  
**Componente**: Cargo.toml Completo com Dependências  
**Duração estimada**: 1 hora  
**Dependência**: SPEC_01_CARGO_INIT

---

## Objetivo
Criar o `Cargo.toml` completo com todas as dependências necessárias para o projeto: áudio I/O, DSP, ML/ONNX, CLI, serialização, utilitários.

---

## Cargo.toml Completo

```toml
[package]
name = "stem-separator"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
description = "Audio stem separator using HTDemucs ONNX models"
repository = "https://github.com/user/stem-separator"
authors = ["User <user@example.com>"]
keywords = ["audio", "stem-separation", "demucs", "onnx", "dsp"]
categories = ["audio", "command-line-utilities", "machine-learning"]

[dependencies]
# Audio I/O
hound = "3.5"
lewton = "0.11"
mp3lame-encoder = "0.4"
symphonia = { version = "0.5", features = ["default"] }

# DSP / Signal Processing
daspect = "0.6"
rubato = "0.15"
realfft = "0.5"
num-complex = "0.4"
ndarray = { version = "0.16", features = ["blas"] }

# ML / ONNX
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

# Utilities
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
strip = true

[profile.bench]
debug = true

[workspace]
```

---

## Justificativa das Dependências

### Audio I/O
| Crate | Versão | Uso |
|-------|--------|-----|
| `hound` | 3.5 | WAV read/write (PCM) |
| `lewton` | 0.11 | FLAC decoding |
| `mp3lame-encoder` | 0.4 | MP3 encoding |
| `symphonia` | 0.5 | General audio decoding (fallback) |

### DSP / Signal Processing
| Crate | Versão | Uso |
|-------|--------|-----|
| `daspect` | 0.6 | STFT, ISTFT, spectrograms |
| `rubato` | 0.15 | High-quality resampling (sinc interpolation) |
| `realfft` | 0.5 | Real FFT (r2c/c2r) |
| `num-complex` | 0.4 | Complex numbers (f32) |
| `ndarray` | 0.16 + blas | N-dimensional arrays (spectrograms) |

### ML / ONNX
| Crate | Versão | Uso |
|-------|--------|-----|
| `ort` | 1.17 | ONNX Runtime (load-dynamic) |
| `ndarray` + blas | 0.16 | Tensor operations |

### CLI & Utils
| Crate | Versão | Uso |
|-------|--------|-----|
| `clap` | 4.5 + derive/env | CLI argument parsing |
| `anyhow` | 1.0 | Error handling ergonomics |
| `tracing` + subscriber | 0.1/0.3 | Structured logging |
| `serde` + json + toml | 1.0/1.0/0.8 | Serialization |
| `rayon` | 1.10 | Parallel processing |
| `tempfile` | 3.8 | Temp files/dirs |
| `walkdir` | 2.5 | Directory traversal |
| `indicatif` | 0.17 | Progress bars |

---

## Features

```toml
[features]
default = []           # Mínimo (sem GPU)
cuda = ["ort/cuda"]    # GPU NVIDIA via CUDA
coreml = ["ort/coreml"] # Apple Neural Engine
```

---

## Perfis de Build

```toml
[profile.release]
opt-level = 3      # Máxima otimização
lto = true         # Link-time optimization
codegen-units = 1  # Single codegen unit
strip = true       # Strip symbols

[profile.bench]
debug = true       # Debug info para benchmarks
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-02.1 | Todas as dependências declaradas | `cargo metadata --format-version=1 \| jq '.packages[] .name'` |
| AC-02.2 | Features funcionam | `cargo build --features cuda` compila |
| AC-02.3 | Build release otimizado | `cargo build --release --all-features` |
| AC-02.4 | Sem vulnerabilidades | `cargo audit` passa |

---

## Checklist

- [ ] Substituir Cargo.toml base pelo completo
- [ ] `cargo fetch` baixa todas dependências
- [ ] `cargo check --all-features` passa
- [ ] `cargo build --release --all-features` compila
- [ ] `cargo tree -d` sem duplicatas problemáticas

---

## Notas

- Versões fixadas (sem `^` ou `~`) para builds reprodutíveis
- `ndarray` aparece duas vezes: uma com `blas` para ML, outra sem para DSP (Cargo resolve)
- `ort` com `load-dynamic` carrega libonnxruntime.so/dll em runtime
- `Cargo.lock` deve ser commitado