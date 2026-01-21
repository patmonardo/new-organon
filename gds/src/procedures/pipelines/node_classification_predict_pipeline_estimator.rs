use crate::mem::{MemoryEstimation, MemoryEstimations};
use crate::ml::models::ClassifierData;
use crate::ml::node_classification::estimate_predict_memory_with_derived_batch_size;

use super::node_classification_predict_pipeline_constants::MIN_BATCH_SIZE;

pub struct NodeClassificationPredictPipelineEstimator;

impl NodeClassificationPredictPipelineEstimator {
    pub fn new() -> Self {
        Self
    }

    pub fn estimate(
        &self,
        model_data: &dyn ClassifierData,
        class_count: usize,
        include_predicted_probabilities: bool,
    ) -> Box<dyn MemoryEstimation> {
        let feature_count = model_data.feature_dimension();

        estimate_predict_memory_with_derived_batch_size(
            model_data.trainer_method(),
            include_predicted_probabilities,
            MIN_BATCH_SIZE,
            feature_count,
            class_count,
            false,
        )
    }

    pub fn estimate_with_fallback(
        &self,
        model_data: Option<&dyn ClassifierData>,
        class_count: usize,
        include_predicted_probabilities: bool,
    ) -> Box<dyn MemoryEstimation> {
        match model_data {
            Some(data) => self.estimate(data, class_count, include_predicted_probabilities),
            None => MemoryEstimations::empty(),
        }
    }
}
