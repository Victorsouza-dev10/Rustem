# Spec 06: errors.rs - Error Handling

**Fase**: 1 - Setup & Fundação  
**Componente**: Error Handling (`src/utils/errors.rs`)  
**Duração estimada**: 30 min  
**Dependência**: SPEC_03_MODULE_STRUCTURE

---

## Objetivo
Criar os tipos de erro customizados usando `thiserror` para erro handling ergonômico e tipado.

---

## Código do errors.rs

```rust
//! Error types for stem-separator
//!
//! Custom error types using `thiserror` for ergonomic error handling.

use thiserror::Error;

/// Main error type for stem-separator operations
#[derive(Debug, Error)]
pub enum StemError {
    /// Audio I/O error (WAV/FLAC/MP3 read/write)
    #[error("Audio I/O error: {0}")]
    AudioIo(#[from] hound::Error),

    /// FLAC decoding/encoding error
    #[error("FLAC error: {0}")]
    Flac(#[from] lewton::error::Error),

    /// MP3 encoding error
    #[error("MP3 encoding error: {0}")]
    Mp3Encode(String),

    /// ONNX Runtime error
    #[error("ONNX Runtime error: {0}")]
    Onnx(#[from] ort::Error),

    /// DSP/Signal processing error
    #[error("DSP error: {0}")]
    Dsp(String),

    /// Model loading/inference error
    #[error("Model error: {0}")]
    Model(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// I/O error (file system)
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Model file not found
    #[error("Model not found: {path}")]
    ModelNotFound { path: String },

    /// Model version mismatch
    #[error("Model version mismatch: expected {expected}, found {found}")]
    ModelVersionMismatch { expected: String, found: String },

    /// Invalid audio format
    #[error("Unsupported audio format: {format}")]
    UnsupportedFormat { format: String },

    /// Sample rate mismatch
    #[error("Sample rate mismatch: expected {expected}, got {actual}")]
    SampleRateMismatch { expected: u32, actual: u32 },

    /// Channel count mismatch
    #[error("Channel count mismatch: expected {expected}, got {actual}")]
    ChannelMismatch { expected: u16, actual: u16 },

    /// Model download error
    #[error("Model download failed: {0}")]
    DownloadError(String),

    /// Model checksum verification failed
    #[error("Model checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    /// GPU/Device error
    #[error("Device error: {0}")]
    Device(String),

    /// Generic error with context
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Convenience type alias for Results
pub type Result<T> = std::result::Result<T, StemError>;

/// Extension trait for adding context to errors
pub trait ResultExt<T> {
    /// Add context to an error
    fn context(self, context: &str) -> Result<T>;
    
    /// Add context with a closure (lazy evaluation)
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| StemError::WithContext {
            context: context.to_string(),
            source: Box::new(e),
        })
    }

    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| StemError::WithContext {
            context: f(),
            source: Box::new(e),
        })
    }
}

/// Macro for creating errors with context
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::utils::errors::StemError::InvalidArgument(
            format!($($arg)*)
        ))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = StemError::InvalidArgument("test error".to_string());
        assert_eq!(format!("{}", err), "Invalid argument: test error");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let stem_err: StemError = io_err.into();
        assert!(matches!(stem_err, StemError::Io(_)));
    }

    #[test]
    fn test_result_ext_context() {
        let result: std::result::Result<(), std::io::Error> = 
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));
        
        let result: crate::utils::errors::Result<()> = result.context("reading file");
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, StemError::WithContext { .. }));
    }

    #[test]
    fn test_error_chain() {
        let err = StemError::Model("load failed".to_string())
            .context("loading model");
        
        let err_str = format!("{}", err);
        assert!(err_str.contains("loading model"));
        assert!(err_str.contains("load failed"));
    }
}
```

---

## Uso Comum

```rust
// Retornar erro simples
fn load_model(path: &Path) -> Result<Model> {
    if !path.exists() {
        bail!("Model file not found: {}", path.display());
    }
    Ok(Model::load(path)?)
}

// Adicionar contexto
fn process_audio(path: &Path) -> Result<AudioBuffer> {
    let audio = read_audio(path)
        .context("reading audio file")?;
    
    let processed = process(audio)
        .context("processing audio")?;
    
    Ok(processed)
}

// Propagar erro com contexto
fn process_batch(files: &[PathBuf]) -> Result<Vec<AudioBuffer>> {
    files.iter()
        .map(|p| process_audio(p).with_context(|| format!("processing {:?}", p)))
        .collect()
}
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-06.1 | Compila | `cargo check` |
| AC-06.2 | `From` implementations funcionam | `let err: StemError = io_err.into();` |
| AC-06.2 | `context()` adiciona contexto | `err.context("msg")` adiciona prefixo |
| AC-06.3 | `bail!` macro funciona | `bail!("error")` retorna Err |
| AC-06.4 | Testes passam | `cargo test utils::errors` passa |
| AC-06.5 | `Display` implementado | `format!("{}", err)` mostra mensagem |

---

## Checklist

- [ ] `src/utils/errors.rs` criado
- [ ] `StemError` enum com todos os variants
- [ ] `Result<T>` type alias
- [ ] `ResultExt` trait com `context()` e `with_context()`
- [ ] `bail!` macro
- [ ] `#[from]` implementations para erros externos
- [ ] `#[test]` module com testes
- [ ] `cargo check` passa
- [ ] `cargo test utils::errors` passa

---

## Notas

- Usa `thiserror` para `#[error]` e `#[from]`
- `#[from]` para conversão automática de erros externos
- `StemError::WithContext` para error chaining
- `ResultExt` trait permite `.context()` em qualquer `Result<T, E>`
- Macro `bail!` para early return ergonômico
- Todos os variants têm `#[error("...")]` com formatação