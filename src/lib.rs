//! Rustem - Separador de áudio usando modelos HTDemucs ONNX

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}