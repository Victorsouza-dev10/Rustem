//! SDR (Signal-to-Distortion Ratio) metric
//!
//! Implementation of BSS_EVAL v4 SDR computation.

use ndarray::Array1;

use crate::utils::errors::{Result, StemError};

/// Compute SDR between reference and estimated signals
pub fn compute_sdr(reference: &Array1<f32>, estimated: &Array1<f32>) -> Result<f32> {
    if reference.len() != estimated.len() {
        return Err(StemError::Config(
            "Reference and estimated signals must have same length".to_string(),
        ));
    }

    // SDR = 10 * log10(||reference||^2 / ||reference - estimated||^2)
    let reference_power: f32 = reference.iter().map(|x| x * x).sum();
    let error_power: f32 = reference
        .iter()
        .zip(estimated.iter())
        .map(|(r, e)| (r - e) * (r - e))
        .sum();

    if error_power <= 0.0 {
        return Ok(f32::INFINITY);
    }

    let sdr = 10.0 * (reference_power / error_power).log10();
    Ok(sdr)
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;

    use super::*;

    #[test]
    fn test_compute_sdr_perfect() {
        let reference = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let sdr = compute_sdr(&reference, &estimated).unwrap();
        assert!(sdr.is_infinite());
    }

    #[test]
    fn test_compute_sdr_mismatch_length() {
        let reference = Array1::from_vec(vec![1.0, 2.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let result = compute_sdr(&reference, &estimated);
        assert!(result.is_err());
    }
}
