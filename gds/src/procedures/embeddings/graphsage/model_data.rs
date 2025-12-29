//! Java: `ModelData`.

use std::sync::Arc;

use super::feature_function::FeatureFunction;
use super::layer::Layer;

#[derive(Clone)]
pub struct ModelData {
    layers: Vec<Arc<dyn Layer>>,
    feature_function: Arc<dyn FeatureFunction>,
}

impl ModelData {
    pub fn of(layers: Vec<Arc<dyn Layer>>, feature_function: Arc<dyn FeatureFunction>) -> Self {
        Self {
            layers,
            feature_function,
        }
    }

    pub fn layers(&self) -> &[Arc<dyn Layer>] {
        &self.layers
    }

    pub fn feature_function(&self) -> Arc<dyn FeatureFunction> {
        Arc::clone(&self.feature_function)
    }
}
