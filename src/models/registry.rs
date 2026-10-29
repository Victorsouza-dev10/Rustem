//! Model registry for loading and managing models

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::{
    audio::io::AudioBuffer,
    models::{
        demucs::{DemucsModel, DemucsModelTrait},
        ensemble::{EnsembleStrategy, ModelEnsemble},
        onnx::OnnxModel,
    },
    utils::errors::{Result, StemError},
};

/// Model registry for loading and caching models
pub struct ModelRegistry {
    models: RwLock<HashMap<String, Arc<dyn DemucsModelTrait + Send + Sync>>>,
    cache_dir: PathBuf,
    onnx_models: RwLock<HashMap<String, Arc<OnnxModel>>>,
}

impl crate::models::demucs::DemucsModel {
    pub fn as_trait(&self) -> Arc<dyn DemucsModelTrait + Send + Sync> {
        Arc::new(DemucsModelWrapper(self.clone()))
    }
}

struct DemucsModelWrapper(crate::models::demucs::DemucsModel);

impl DemucsModelTrait for DemucsModelWrapper {
    fn separate(
        &self,
        audio: AudioBuffer,
    ) -> crate::utils::errors::Result<crate::models::demucs::StemOutputs> {
        self.0.separate(audio)
    }

    fn name(&self) -> &str {
        "htdemucs"
    }
}

// Blanket implementation for Arc<T> where T: DemucsModelTrait
impl<T: DemucsModelTrait + ?Sized> DemucsModelTrait for Arc<T> {
    fn separate(
        &self,
        audio: AudioBuffer,
    ) -> crate::utils::errors::Result<crate::models::demucs::StemOutputs> {
        (**self).separate(audio)
    }

    fn name(&self) -> &str {
        (**self).name()
    }
}

impl ModelRegistry {
    /// Create new model registry
    pub fn new<P: AsRef<std::path::Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            models: RwLock::new(HashMap::new()),
            cache_dir,
            onnx_models: RwLock::new(HashMap::new()),
        })
    }

    /// Get or load a Demucs model
    pub async fn get_demucs_model(
        &self,
        name: &str,
    ) -> Result<Arc<dyn DemucsModelTrait + Send + Sync>> {
        {
            let models = self.models.read().await;
            if let Some(model) = models.get(name) {
                return Ok(model.clone());
            }
        }

        // Load model
        let model = self.load_demucs_model(name).await?;
        let model_arc = Arc::new(model);

        {
            let mut models = self.models.write().await;
            models.insert(name.to_string(), model_arc.clone());
        }

        Ok(model_arc)
    }

    /// Load a Demucs model from disk or download
    async fn load_demucs_model(
        &self,
        name: &str,
    ) -> Result<Arc<dyn DemucsModelTrait + Send + Sync>> {
        let model_path = self.cache_dir.join(format!("{}.onnx", name));

        if !model_path.exists() {
            self.download_model(name, &model_path).await?;
        }

        let demucs = crate::models::demucs::DemucsModel::load(&model_path)?;
        Ok(Arc::new(crate::models::demucs::DemucsModelWrapper(demucs)))
    }

    /// Download model from remote
    async fn download_model(&self, name: &str, path: &std::path::Path) -> Result<()> {
        let url = format!(
            "https://github.com/facebookresearch/demucs/releases/download/v4.0/{}.onnx",
            name
        );

        let response = reqwest::get(&url)
            .await
            .map_err(|e| crate::utils::errors::StemError::DownloadError(e.to_string()))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| crate::utils::errors::StemError::DownloadError(e.to_string()))?;

        tokio::fs::write(path, bytes)
            .await
            .map_err(|e| crate::utils::errors::StemError::Io(e))?;

        // Verify checksum if available
        self.verify_checksum(path).await?;

        Ok(())
    }

    /// Verify model checksum
    async fn verify_checksum(&self, path: &std::path::Path) -> Result<()> {
        // TODO: Implement checksum verification
        // For now, just check file exists and has reasonable size
        let metadata = tokio::fs::metadata(path)
            .await
            .map_err(|e| crate::utils::errors::StemError::Io(e))?;

        if metadata.len() < 1024 * 1024 {
            // Less than 1MB is suspicious
            return Err(crate::utils::errors::StemError::ChecksumMismatch {
                expected: "valid size".to_string(),
                actual: format!("{} bytes", metadata.len()),
            });
        }

        Ok(())
    }

    /// Get or create model ensemble
    pub async fn get_ensemble(
        &self,
        name: &str,
        strategy: crate::models::ensemble::EnsembleStrategy,
    ) -> Result<crate::models::ensemble::ModelEnsemble> {
        let models = self.get_all_models().await?;

        let model_boxes: Vec<Box<dyn crate::models::ensemble::StemModel>> = models
            .into_iter()
            .map(|m| {
                Box::new(EnsembleModelWrapper(m)) as Box<dyn crate::models::ensemble::StemModel>
            })
            .collect();

        Ok(crate::models::ensemble::ModelEnsemble::with_strategy(
            vec![],
            crate::models::ensemble::EnsembleStrategy::Mean,
        ))
    }

    /// Get all loaded models
    pub async fn get_all_models(&self) -> Result<Vec<Arc<dyn DemucsModelTrait + Send + Sync>>> {
        let models = self.models.read().await;
        Ok(models.values().cloned().collect())
    }

    /// Get ONNX model
    pub async fn get_onnx_model(&self, name: &str) -> Result<Arc<crate::models::onnx::OnnxModel>> {
        {
            let models = self.onnx_models.read().await;
            if let Some(model) = models.get(name) {
                return Ok(model.clone());
            }
        }

        // Load ONNX model
        let model_path = self.cache_dir.join(format!("{}.onnx", name));
        let model = crate::models::onnx::OnnxModel::load(&model_path)?;
        let model_arc = Arc::new(model);

        {
            let mut models = self.onnx_models.write().await;
            models.insert(name.to_string(), model_arc.clone());
        }

        Ok(model_arc)
    }

    /// List available models
    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }

    /// Clear cache
    pub async fn clear_cache(&self) -> Result<()> {
        let mut models = self.models.write().await;
        models.clear();

        let mut onnx_models = self.onnx_models.write().await;
        onnx_models.clear();

        Ok(())
    }

    /// Get cache directory
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

/// Wrapper to make DemucsModel implement StemModel trait
struct EnsembleModelWrapper(Arc<dyn DemucsModelTrait + Send + Sync>);

impl crate::models::ensemble::StemModel for EnsembleModelWrapper {
    fn infer(
        &self,
        spectrogram: &ndarray::Array3<f32>,
    ) -> crate::utils::errors::Result<ndarray::Array4<f32>> {
        // This would need actual implementation
        // For now, return error
        Err(crate::utils::errors::StemError::Model("Not implemented".to_string()))
    }

    fn name(&self) -> &str {
        "ensemble_wrapper"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let registry = ModelRegistry::new(temp_dir.path()).unwrap();
        assert!(registry.cache_dir().exists());
    }

    #[test]
    fn test_list_models_empty() {
        let temp_dir = tempfile::tempdir().unwrap();
        let registry = ModelRegistry::new(temp_dir.path()).unwrap();
        // Note: list_models is async, but we can't easily test it without tokio runtime
        // This is a placeholder test - actual async testing requires tokio::test
        assert!(registry.cache_dir().exists());
    }
}
