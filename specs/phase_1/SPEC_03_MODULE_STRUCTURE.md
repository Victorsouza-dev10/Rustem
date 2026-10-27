# Spec 03: Module Structure - Estrutura de Módulos

**Fase**: 1 - Setup & Fundação  
**Componente**: Estrutura de Módulos `src/`  
**Duração estimada**: 45 min  
**Dependência**: SPEC_02_CARGO_TOML

---

## Objetivo
Criar a estrutura completa de módulos em `src/` com todos os módulos declarados e arquivos `.rs` base (vazios ou com esqueleto).

---

## Estrutura de Diretórios

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Public API re-exports
├── audio/
│   ├── mod.rs
│   ├── io.rs            # WAV/FLAC/MP3 I/O
│   ├── preprocessing.rs # Resample, normalize, STFT
│   └── postprocessing.rs# Crossfade, loudness, dither
├── models/
│   ├── mod.rs
│   ├── onnx.rs          # ONNX Runtime wrapper
│   ├── demucs.rs        # HTDemucs model wrapper
│   ├── ensemble.rs      # Ensemble logic
│   └── registry.rs      # Model loading/caching
├── separation/
│   ├── mod.rs
│   ├── pipeline.rs      # Main pipeline
│   ├── stem.rs          # Stem enum + traits
│   └── config.rs        # Config structs
├── metrics/
│   ├── mod.rs
│   ├── sdr.rs           # SDR (BSS_EVAL v4)
│   ├── sir.rs
│   └── sar.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs          # Clap args
│   └── commands.rs
└── utils/
    ├── mod.rs
    ├── errors.rs
    └── logging.rs
```

---

## Arquivos a Criar

### Raiz `src/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `main.rs` | Esqueleto CLI (ver SPEC_05) |
| `lib.rs` | Re-exports públicos (ver SPEC_04) |

### `src/audio/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod io; pub mod preprocessing; pub mod postprocessing;` |
| `io.rs` | `//! Audio I/O module` + `pub mod wav; pub mod flac; pub mod mp3;` |
| `preprocessing.rs` | `//! Preprocessing pipeline` + structs vazios |
| `postprocessing.rs` | `//! Postprocessing pipeline` + structs vazios |

### `src/models/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod onnx; pub mod demucs; pub mod ensemble; pub mod registry;` |
| `onnx.rs` | `//! ONNX Runtime wrapper` + struct `OnnxModel` vazio |
| `demucs.rs` | `//! HTDemucs model wrapper` + struct `DemucsModel` vazio |
| `ensemble.rs` | `//! Model ensemble` + trait `StemModel` + enum `EnsembleStrategy` |
| `registry.rs` | `//! Model registry` + struct `ModelRegistry` vazio |

### `src/separation/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod pipeline; pub mod stem; pub mod config;` |
| `pipeline.rs` | `//! Separation pipeline` + struct `SeparationPipeline` vazio |
| `stem.rs` | `//! Stem definitions` + enum `Stem` + trait `StemModel` |
| `config.rs` | `//! Separation config` + struct `SeparationConfig` |

### `src/metrics/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod sdr; pub mod sir; pub mod sar;` |
| `sdr.rs` | `//! SDR metric (BSS_EVAL v4)` + fn `compute_sdr` stub |
| `sir.rs` | `//! SIR metric` + fn `compute_sir` stub |
| `sar.rs` | `//! SAR metric` + fn `compute_sar` stub |

### `src/cli/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod args; pub mod commands;` |
| `args.rs` | `//! CLI arguments` + struct `Args` com clap derive |
| `commands.rs` | `//! CLI commands` + fn `run()` stub |

### `src/utils/`

| Arquivo | Conteúdo Inicial |
|---------|------------------|
| `mod.rs` | `pub mod errors; pub mod logging;` |
| `errors.rs` | `//! Error types` + enum `StemError` + `Result<T>` (ver SPEC_06) |
| `logging.rs` | `//! Logging setup` + fn `init_logging()` (ver SPEC_07) |

---

## Padrão de Arquivo Base

Cada arquivo `.rs` deve começar com:

```rust
//! Module description
//!
//! Brief description of what this module does.

// Imports
use crate::utils::errors::{Result, StemError};

// Public exports
pub mod submodule;
pub use submodule::PublicType;

// Types and functions...
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-03.1 | Todos os diretórios criados | `find src -type d | wc -l` = 7 |
| AC-03.2 | Todos os mod.rs existem | `find src -name "mod.rs" | wc -l` = 7 |
| AC-03.3 | Todos os .rs listados existem | `find src -name "*.rs" | wc -l` = 21 |
| AC-03.4 | Módulos declarados corretamente | `cargo check` sem erros de módulo |

---

## Checklist

- [ ] Criar diretórios: `mkdir -p src/{audio,models,separation,metrics,cli,utils}`
- [ ] Criar `mod.rs` em cada diretório
- [ ] Criar arquivos `.rs` listados acima
- [ ] Cada `mod.rs` declara `pub mod submodule;`
- [ ] `cargo check` passa sem erros de módulo
- [ ] `cargo tree` mostra estrutura correta

---

## Comandos Úteis

```bash
# Criar estrutura
mkdir -p src/{audio,models,separation,metrics,cli,utils}

# Verificar estrutura
find src -type f -name "*.rs" | sort

# Verificar módulos
cargo check 2>&1 | grep -i "error\|warning" || echo "OK"
```

---

## Notas

- Arquivos podem começar vazios com apenas declarações de módulo
- Não implementar lógica ainda (fases posteriores)
- `mod.rs` apenas declara `pub mod` e re-exporta tipos públicos
- `lib.rs` fará re-export final (ver SPEC_04)