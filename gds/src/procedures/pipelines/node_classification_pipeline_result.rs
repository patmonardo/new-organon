use std::sync::Arc;

use crate::collections::{HugeLongArray, HugeObjectArray};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::node_classification::NodeClassificationPredictResult;

pub struct NodeClassificationPipelineResult {
    predicted_classes: Arc<HugeLongArray>,
    predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
}

impl NodeClassificationPipelineResult {
    pub fn of(
        node_classification_result: &NodeClassificationPredictResult,
        class_id_map: &LocalIdMap,
    ) -> Self {
        let internal_predictions = node_classification_result.predicted_classes();
        let mut predictions = HugeLongArray::new(internal_predictions.size());

        for index in 0..internal_predictions.size() {
            let internal_class_id = internal_predictions.get(index) as usize;
            let original_class_id = class_id_map.to_original(internal_class_id) as i64;
            predictions.set(index, original_class_id);
        }

        Self {
            predicted_classes: Arc::new(predictions),
            predicted_probabilities: node_classification_result
                .predicted_probabilities()
                .map(Arc::clone),
        }
    }

    pub fn predicted_classes(&self) -> &Arc<HugeLongArray> {
        &self.predicted_classes
    }

    pub fn predicted_probabilities(&self) -> Option<&Arc<HugeObjectArray<Vec<f64>>>> {
        self.predicted_probabilities.as_ref()
    }
}
