use super::SimilarityComputer;
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

#[derive(Debug)]
pub struct DoublePropertySimilarityComputer {
    values: Arc<dyn NodePropertyValues>,
}

impl DoublePropertySimilarityComputer {
    pub fn new(values: Arc<dyn NodePropertyValues>) -> Self {
        Self { values }
    }
}

impl SimilarityComputer for DoublePropertySimilarityComputer {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let Ok(left) = self.values.double_value(first_node_id) else {
            return 0.0;
        };
        let Ok(right) = self.values.double_value(second_node_id) else {
            return 0.0;
        };
        1.0 / (1.0 + (left - right).abs())
    }

    fn is_symmetric(&self) -> bool {
        true
    }
}
