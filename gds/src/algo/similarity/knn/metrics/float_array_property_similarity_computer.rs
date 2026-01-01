use super::cosine;
use super::euclidean;
use super::pearson;
use super::{SimilarityComputer, SimilarityMetric};
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

#[derive(Debug)]
pub struct FloatArrayPropertySimilarityComputer {
    values: Arc<dyn NodePropertyValues>,
    metric: SimilarityMetric,
}

impl FloatArrayPropertySimilarityComputer {
    pub fn new(values: Arc<dyn NodePropertyValues>, metric: SimilarityMetric) -> Self {
        Self { values, metric }
    }
}

impl SimilarityComputer for FloatArrayPropertySimilarityComputer {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let Ok(left) = self.values.float_array_value(first_node_id) else {
            return 0.0;
        };
        let Ok(right) = self.values.float_array_value(second_node_id) else {
            return 0.0;
        };

        match self.metric {
            SimilarityMetric::Cosine | SimilarityMetric::Default => {
                cosine::float_metric(&left, &right)
            }
            SimilarityMetric::Euclidean => euclidean::float_metric(&left, &right),
            SimilarityMetric::Pearson => pearson::float_metric(&left, &right),
            _ => 0.0,
        }
    }

    fn is_symmetric(&self) -> bool {
        true
    }
}
