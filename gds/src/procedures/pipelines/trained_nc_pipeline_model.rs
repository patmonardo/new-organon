use std::sync::Arc;

use crate::core::model::{Model, ModelCatalog, ModelCatalogCustomInfo, ModelConfig, ModelData};
use crate::procedures::model_catalog::ModelCatalogFacade;
use crate::projection::eval::pipeline::NodeClassificationModelResult;
use crate::projection::eval::pipeline::NodeClassificationPipelineTrainConfig;
use crate::projection::eval::pipeline::NodeClassificationToModelConverter;
use crate::projection::eval::pipeline::NodeClassificationTrainResult;
use crate::projection::eval::pipeline::NodeClassificationTrainingPipeline;
use crate::types::schema::GraphSchema;

pub struct TrainedNCPipelineModel {
    model_catalog: Arc<ModelCatalogFacade>,
}

impl TrainedNCPipelineModel {
    pub fn new(model_catalog: Arc<ModelCatalogFacade>) -> Self {
        Self { model_catalog }
    }

    pub fn store<D, C, I>(&self, model: Model<D, C, I>, _store_to_disk: bool)
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: ModelCatalogCustomInfo + 'static,
    {
        self.model_catalog
            .set(model)
            .unwrap_or_else(|e| panic!("{e}"));
    }

    pub fn to_model(
        &self,
        pipeline: &NodeClassificationTrainingPipeline,
        config: &NodeClassificationPipelineTrainConfig,
        result: NodeClassificationTrainResult,
    ) -> NodeClassificationModelResult {
        let converter = NodeClassificationToModelConverter::new(pipeline.clone(), config.clone());
        converter.to_model(result, &GraphSchema::empty())
    }
}
