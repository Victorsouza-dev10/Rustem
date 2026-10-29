//! SAR (Signal-to-Artifacts Ratio) metric
//!
//! Implementation of BSS_EVAL v4 SAR computation.

use ndarray::Array1;

use crate::utils::errors::{Result, StemError};

/// Compute SAR between reference, estimated, and artifact signals
pub fn compute_sar(
    reference: &Array1<f32>,
    estimated: &Array1<f32>,
    artifact: &Array1<f32>,
) -> Result<f32> {
    if reference.len() != estimated.len() || reference.len() != artifact.len() {
        return Err(StemError::Config("All signals must have same length".to_string()));
    }

    // SAR = 10 * log10(||estimated||^2 / ||artifact||^2)
    // Where artifact = estimated - reference (simplified)
    let estimated_power: f32 = estimated.iter().map(|x| x * x).sum();
    let artifact_power: f32 = artifact.iter().map(|x| x * x).sum();

    if artifact_power <= 0.0 {
        return Ok(f32::INFINITY);
    }

    let sar = 10.0 * (estimated_power / artifact_power).log10();
    Ok(sar)
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;

    use super::*;

    #[test]
    fn test_compute_sar_perfect() {
        let reference = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let artifact = Array1::from_vec(vec![0.0, 0.0, 0.0]);
        let sar = compute_sar(&reference, &estimated, &artifact).unwrap();
        assert!(sar.is_infinite());
    }

    #[test]
    fn test_compute_sar_mismatch_length() {
        let reference = Array1::from_vec(vec![1.0, 2.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let artifact = Array1::from_vec(vec![0.0, 0.0]);
        let result = compute_sar(&reference, &estimated, &artifact);
        assert!(result.is_err());
    }
}
