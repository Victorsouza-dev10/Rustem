# Spec 07: logging.rs - Logging Setup

**Fase**: 1 - Setup & Fundação  
**Componente**: Logging Setup (`src/utils/logging.rs`)  
**Duração estimada**: 20 min  
**Dependência**: SPEC_03_MODULE_STRUCTURE, SPEC_06_ERRORS_RS

---

## Objetivo
Configurar logging estruturado com `tracing` e `tracing-subscriber` com suporte a níveis via variável de ambiente.

---

## Código do logging.rs

```rust
//! Logging setup for stem-separator
//!
//! Structured logging using `tracing` and `tracing-subscriber`.
//! Supports log levels via RUST_LOG environment variable.

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Initialize structured logging
///
/// Reads log level from `RUST_LOG` environment variable.
/// Defaults to `info` for general, `debug` for stem_separator crate.
///
/// # Environment Variables
/// - `RUST_LOG`: Standard Rust log filter (e.g., "debug,stem_separator=trace")
/// - `STEM_SEPARATOR_LOG`: Alternative, crate-specific filter
///
/// # Examples
/// ```bash
/// # Default logging
/// RUST_LOG=info cargo run -- separate input.wav -o out/
///
/// # Debug stem_separator only
/// RUST_LOG=info,stem_separator=debug cargo run -- separate input.wav -o out/
///
/// # Trace everything
/// RUST_LOG=trace cargo run -- separate input.wav -o out/
/// ```
pub fn init_logging() {
    // Build env filter from RUST_LOG or default
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info,stem_separator=debug")
        });

    // Build formatter
    let fmt_layer = fmt::layer()
        .with_target(false)           // Don't show module path
        .with_thread_ids(false)       // No thread IDs
        .with_thread_names(false)     // No thread names
        .with_file(false)             // No file names
        .with_line_number(false)      // No line numbers
        .with_level(true)             // Show level (ERROR, WARN, INFO, DEBUG, TRACE)
        .compact();                   // Compact single-line format

    // Initialize global subscriber
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,stem_separator=debug"))
        )
        .init();
}

/// Initialize logging with custom filter
///
/// Useful for tests or programmatic configuration.
pub fn init_logging_with_filter(filter: &str) {
    let env_filter = EnvFilter::new(filter);
    
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .compact();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .init();
}

/// Initialize JSON logging (for log aggregation)
pub fn init_json_logging() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,stem_separator=debug"));

    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(json_layer)
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,stem_separator=debug")))
        .init();
}

/// Convenience macro for creating spans
#[macro_export]
macro_rules! span {
    ($name:expr) => {
        tracing::info_span!($name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info_span!($name, $($field)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_default() {
        // Should not panic
        init_logging_with_filter("info");
    }

    #[test]
    fn test_init_logging_with_filter() {
        init_logging_with_filter("trace");
    }

    #[test]
    fn test_span_macro() {
        let _span = span!("test_span");
        let _span2 = span!("test_span", field = "value");
    }
}
```

---

## Uso

### Inicialização no main.rs

```rust
fn main() -> Result<()> {
    stem_separator::utils::logging::init_logging();
    commands::run()
}
```

### Variáveis de Ambiente

```bash
# Default (info + stem_separator debug)
RUST_LOG=info cargo run -- separate input.wav -o out/

# Debug stem_separator
RUST_LOG=info,stem_separator=debug cargo run -- separate input.wav -o out/

# Trace tudo
RUST_LOG=trace cargo run -- separate input.wav -o out/

# Apenas warnings
RUST_LOG=warn cargo run -- separate input.wav -o out/

# Filtro custom
STEM_SEPARATOR_LOG=debug cargo run -- separate input.wav -o out/
```

### Formato de Saída

```
2024-01-15T10:30:45.123 INFO  stem_separator::audio::io: Reading audio file: input.wav
2024-01-15T10:30:45.234 DEBUG stem_separator::audio::preprocessing: Resampling 48000 -> 44100 Hz
2024-01-15T10:30:45.567 INFO  stem_separator::separation::pipeline: Processing 5 stems
2024-01-15T10:30:50.123 INFO  stem_separator::separation::pipeline: Separation completed in 4.8s (RTF: 0.16)
```

---

## Modos de Logging

### Desenvolvimento (Padrão)
```rust
init_logging()  // Compact, colorido, human-readable
```

### Testes
```rust
init_logging_with_filter("trace")
```

### Produção / Log Aggregation (JSON)
```rust
init_json_logging()  // JSON lines para log aggregation (ELK, Datadog, etc.)
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-07.1 | Compila | `cargo check` |
| AC-07.2 | `init_logging()` não panica | `init_logging()` roda sem panic |
| AC-07.3 | `RUST_LOG` funciona | `RUST_LOG=trace cargo run` mostra trace |
| AC-07.4 | Formato compacto | Logs em uma linha |
| AC-07.4 | JSON logging | `init_json_logging()` produz JSON lines |

---

## Checklist

- [ ] `src/utils/logging.rs` criado
- [ ] `init_logging()` implementada
- [ ] `init_logging_with_filter()` implementada
- [ ] `init_json_logging()` implementada
- [ ] Macro `span!` definida
- [ ] Doc comments com exemplos
- [ ] Testes unitários
- [ ] `cargo check` passa
- [ ] `cargo test utils::logging` passa

---

## Notas

- `tracing-subscriber` com `EnvFilter` para filtros dinâmicos
- `fmt::layer().compact()` para output single-line
- `RUST_LOG` padrão: `info,stem_separator=debug`
- `init_logging()` deve ser chamada **antes** de qualquer `tracing::info!` etc.
- JSON mode para produção (ELK, Datadog, Loki, etc.)
- Não usar `tracing::subscriber::set_global_default` diretamente - usar `registry().init()`