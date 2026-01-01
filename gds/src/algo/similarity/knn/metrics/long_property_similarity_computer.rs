use super::SimilarityComputer;
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

#[derive(Debug)]
pub struct LongPropertySimilarityComputer {
    values: Arc<dyn NodePropertyValues>,
}

impl LongPropertySimilarityComputer {
    pub fn new(values: Arc<dyn NodePropertyValues>) -> Self {
        Self { values }
    }
}

impl SimilarityComputer for LongPropertySimilarityComputer {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let Ok(left) = self.values.long_value(first_node_id) else {
            return 0.0;
        };
        let Ok(right) = self.values.long_value(second_node_id) else {
            return 0.0;
        };

        let mut abs = (left - right).abs();
        if abs == i64::MIN {
            abs = i64::MAX;
        }
        1.0 / (1.0 + abs as f64)
    }

    fn is_symmetric(&self) -> bool {
        true
    }
}
