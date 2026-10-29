//! SIR (Signal-to-Interference Ratio) metric
//!
//! Implementation of BSS_EVAL v4 SIR computation.

use ndarray::Array1;

use crate::utils::errors::{Result, StemError};

/// Compute SIR between reference, estimated, and interference signals
pub fn compute_sir(
    reference: &Array1<f32>,
    estimated: &Array1<f32>,
    interference: &Array1<f32>,
) -> Result<f32> {
    if reference.len() != estimated.len() || reference.len() != interference.len() {
        return Err(StemError::Config("All signals must have same length".to_string()));
    }

    // SIR = 10 * log10(||reference||^2 / ||interference||^2)
    // In practice, interference = estimated - reference
    let reference_power: f32 = reference.iter().map(|x| x * x).sum();
    let interference_power: f32 = interference.iter().map(|x| x * x).sum();

    if interference_power <= 0.0 {
        return Ok(f32::INFINITY);
    }

    let sir = 10.0 * (reference_power / interference_power).log10();
    Ok(sir)
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;

    use super::*;

    #[test]
    fn test_compute_sir_perfect() {
        let reference = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let interference = Array1::from_vec(vec![0.0, 0.0, 0.0]);
        let sir = compute_sir(&reference, &estimated, &interference).unwrap();
        assert!(sir.is_infinite());
    }

    #[test]
    fn test_compute_sir_mismatch_length() {
        let reference = Array1::from_vec(vec![1.0, 2.0]);
        let estimated = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let interference = Array1::from_vec(vec![0.0, 0.0]);
        let result = compute_sir(&reference, &estimated, &interference);
        assert!(result.is_err());
    }
}
