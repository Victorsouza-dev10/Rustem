# Spec 08: CLAUDE.md - Convenções do Projeto

**Fase**: 1 - Setup & Fundação  
**Componente**: Convenções do Projeto (`CLAUDE.md`)  
**Duração estimada**: 30 min  
**Dependência**: SPEC_03_MODULE_STRUCTURE

---

## Objetivo
Criar o arquivo `CLAUDE.md` com todas as convenções de código, estilo, versionamento e práticas do projeto.

---

## Conteúdo do CLAUDE.md

```markdown
# Convenções do Projeto stem-separator

## Rust

- **Edition**: 2021
- **Formatting**: `cargo fmt` (padrão rustfmt)
- **Linting**: `cargo clippy -D warnings` (zero warnings)
- **MSRV**: 1.70+

## Error Handling

- Tipos de erro customizados com `thiserror` em `utils/errors.rs`
- `anyhow::Result<T>` para propagação
- `#[error("...")]` em todos os variants

## Async

- **Apenas** para I/O assíncrono (download de modelos)
- **DSP/Inferência**: síncrono (CPU-bound, usa rayon)
- Runtime: `tokio` apenas para download de modelos

## Logging

- `tracing` com spans estruturados
- Níveis: `error`, `warn`, `info`, `debug`, `trace`
- `init_logging()` em `main.rs` antes de tudo

## Testes

- Unitários: `#[cfg(test)]` inline no mesmo arquivo
- Integração: `tests/` (arquivos `*_test.rs`)
- Fixtures em `tests/fixtures/`
- `cargo test` deve passar 100%

## Documentação

- `///` doc comments em **todos** os `pub` items
- Exemplos compiláveis nos doc comments (`/// ```)
- `cargo doc --no-deps` sem warnings

## Versionamento

- SemVer (MAJOR.MINOR.PATCH)
- `Cargo.lock` commitado
- Tags git: `v0.1.0`, `v0.2.0`, etc.

## Features

- `default = []` (mínimo)
- `cuda` = ["ort/cuda"] - GPU NVIDIA
- `coreml` = ["ort/coreml"] - Apple Neural Engine
- Features são **aditivas** (não mutuamente exclusivas)

## Code Style

- `cargo fmt` antes de commit
- `cargo clippy -D warnings` passa limpo
- Imports agrupados: std, external, local
- `use` agrupado por crate
- Nomes: `snake_case` (vars/fn), `PascalCase` (types), `SCREAMING_SNAKE_CASE` (consts)
- Um `mod.rs` por diretório, re-exporta filhos
- **Não** usar `unwrap()`/`expect()` em código de produção - usar `?` e `?`

## Estrutura de Módulos

```
src/
├── main.rs
├── lib.rs
├── audio/
├── models/
├── separation/
├── metrics/
├── cli/
└── utils/
```

## Imports

```rust
// Ordem: std -> external -> local
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use ndarray::Array2;
use ort::Session;

use crate::audio::AudioBuffer;
use crate::models::DemucsModel;
use crate::utils::errors::{Result, StemError};
```

## Naming Conventions

| Tipo | Convenção | Exemplo |
|------|-----------|---------|
| Functions/Variables | snake_case | `process_audio`, `sample_rate` |
| Types/Structs/Enums | PascalCase | `AudioBuffer`, `SeparationConfig` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_SAMPLE_RATE` |
| Modules | snake_case | `audio`, `models` |
| Files | snake_case | `preprocessing.rs` |
| Traits | PascalCase | `StemModel` |
| Macros | snake_case | `bail!` |

## Error Handling Patterns

```rust
// ✅ Bom - propagação com contexto
fn load_model(path: &Path) -> Result<Model> {
    let model = Model::load(path)
        .context("loading model")?;
    Ok(model)
}

// ✅ Bom - early return com bail!
fn validate_config(cfg: &Config) -> Result<()> {
    if cfg.sample_rate == 0 {
        bail!("sample_rate must be > 0");
    }
    Ok(())
}

// ❌ Ruim - unwrap em produção
let model = Model::load(path).unwrap(); // NUNCA

// ❌ Ruim - expect em produção
let audio = read_audio(path).expect("failed"); // NUNCA
```

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Given
        let input = create_test_input();
        
        // When
        let result = function_under_test(input);
        
        // Then
        assert_eq!(result, expected);
    }
}
```

## Documentation

```rust
/// Short description (one line)
///
/// Longer description if needed. Explains what the function does,
/// its parameters, return value, and any side effects.
///
/// # Arguments
/// * `input` - Description of input parameter
/// * `config` - Configuration options
///
/// # Returns
/// Returns `Result<Output, StemError>` with the processed output.
///
/// # Errors
/// Returns `StemError::Io` if file cannot be read.
/// Returns `StemError::Model` if model inference fails.
///
/// # Example
/// ```
/// use stem_separator::{process_audio, SeparationConfig};
/// use std::path::Path;
///
/// let config = SeparationConfig::default();
/// let output = process_audio(Path::new("input.wav"), config)?;
/// ```
pub fn process_audio(input: &Path, config: SeparationConfig) -> Result<Output> {
    // implementation
}
```

## Git Workflow

- Branch naming: `feature/description`, `fix/description`, `chore/description`
- Commit messages: Conventional Commits
  - `feat: add new feature`
  - `fix: fix bug description`
  - `chore: update dependencies`
  - `docs: update documentation`
  - `refactor: refactor module`
  - `test: add tests`
  - `perf: improve performance`
- PRs requerem: CI verde, review aprovado, docs atualizadas

## Versionamento

- SemVer: MAJOR.MINOR.PATCH
- `Cargo.lock` commitado
- Tags: `v0.1.0`, `v0.2.0`, etc.
- CHANGELOG.md mantido

## Performance

- `#[inline]` apenas em hot paths comprovados
- `#[inline(always)]` apenas com benchmarks
- `#[cold]` em error paths
- `#[inline(never)]` em cold paths
- `#[target_feature]` apenas com `#[cfg(target_feature)]`

## Security

- Não logar dados sensíveis (paths ok, tokens não)
- Validar inputs de usuário
- Verificar checksums de modelos baixados
- Não executar código não confiável

---

## Referências

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Style Guide](https://rust-lang.github.io/style-guide/)
- [API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)