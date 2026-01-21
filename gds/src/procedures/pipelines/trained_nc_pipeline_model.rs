use std::sync::Arc;

use crate::core::model::{Model, ModelCatalog, ModelCatalogCustomInfo, ModelConfig, ModelData};
use crate::procedures::model_catalog::FacadeModelCatalog;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_model_result::NodeClassificationModelResult;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_to_model_converter::NodeClassificationToModelConverter;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_train_result::NodeClassificationTrainResult;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_training_pipeline::NodeClassificationTrainingPipeline;

pub struct TrainedNCPipelineModel {
    model_catalog: Arc<FacadeModelCatalog>,
}

impl TrainedNCPipelineModel {
    pub fn new(model_catalog: Arc<FacadeModelCatalog>) -> Self {
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
        converter.to_model(result, &crate::types::schema::GraphSchema::empty())
    }
}
