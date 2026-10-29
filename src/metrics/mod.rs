//! Metrics module
//!
//! Provides audio quality metrics: SDR, SIR, SAR (BSS_EVAL v4).

pub mod sar;
pub mod sdr;
pub mod sir;

// Re-exports for convenience
pub use sar::compute_sar;
pub use sdr::compute_sdr;
pub use sir::compute_sir;
// QualityMetrics is re-exported from separation module
