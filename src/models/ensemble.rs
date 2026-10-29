//! Model ensemble logic
//!
//! Provides ensemble strategies for combining multiple model predictions.

use crate::utils::errors::Result;

/// Ensemble strategy for combining model outputs
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EnsembleStrategy {
    /// Simple mean of all model outputs
    Mean,
    /// Median of model outputs (robust to outliers)
    Median,
    /// Weighted mean based on model confidence
    WeightedMean,
    /// Minimum of all outputs (conservative)
    Min,
    /// Maximum of all outputs (aggressive)
    Max,
}

/// Trait for stem separation models
pub trait StemModel: Send + Sync {
    /// Run inference on spectrogram
    fn infer(
        &self,
        spectrogram: &ndarray::Array3<f32>,
    ) -> crate::utils::errors::Result<ndarray::Array4<f32>>;

    /// Get model name
    fn name(&self) -> &str;

    /// Get model weight for weighted ensemble
    fn weight(&self) -> f32 {
        1.0
    }
}

/// Model ensemble for combining multiple stem separation models
pub struct ModelEnsemble {
    models: Vec<Box<dyn StemModel>>,
    strategy: EnsembleStrategy,
}

impl ModelEnsemble {
    /// Create new ensemble with default strategy (Mean)
    pub fn new(models: Vec<Box<dyn StemModel>>) -> Self {
        Self { models, strategy: EnsembleStrategy::Mean }
    }

    /// Create ensemble with specific strategy
    pub fn with_strategy(models: Vec<Box<dyn StemModel>>, strategy: EnsembleStrategy) -> Self {
        Self { models, strategy }
    }

    /// Add a model to the ensemble
    pub fn add_model(&mut self, model: Box<dyn StemModel>) {
        self.models.push(model);
    }

    /// Set ensemble strategy
    pub fn set_strategy(&mut self, strategy: EnsembleStrategy) {
        self.strategy = strategy;
    }

    /// Run inference using ensemble
    pub fn infer(
        &self,
        _spectrogram: &ndarray::Array3<f32>,
    ) -> crate::utils::errors::Result<ndarray::Array4<f32>> {
        // TODO: Implement actual ensemble inference
        Err(crate::utils::errors::StemError::Model(
            "Ensemble inference not yet implemented".to_string(),
        ))
    }

    /// Get number of models in ensemble
    pub fn len(&self) -> usize {
        self.models.len()
    }

    /// Check if ensemble is empty
    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    /// Get ensemble strategy
    pub fn strategy(&self) -> EnsembleStrategy {
        self.strategy
    }

    /// Get model names
    pub fn model_names(&self) -> Vec<String> {
        self.models.iter().map(|m| m.name().to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensemble_creation() {
        let ensemble = ModelEnsemble::new(vec![]);
        assert!(ensemble.is_empty());
    }
}
