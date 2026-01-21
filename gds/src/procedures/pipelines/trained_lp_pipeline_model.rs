use std::sync::Arc;

use crate::core::model::ModelCatalog;
use crate::procedures::model_catalog::FacadeModelCatalog;

pub struct TrainedLPPipelineModel {
    model_catalog: Arc<FacadeModelCatalog>,
}

impl TrainedLPPipelineModel {
    pub fn new(model_catalog: Arc<FacadeModelCatalog>) -> Self {
        Self { model_catalog }
    }

    pub fn get(
        &self,
        model_name: &str,
        username: &str,
    ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
        self.model_catalog
            .get_untyped(username, model_name)
            .unwrap_or_else(|e| panic!("{e}"))
    }
}
