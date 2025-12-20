use super::jaccard;
use super::overlap;
use super::{SimilarityComputer, SimilarityMetric};
use crate::types::properties::node::NodePropertyValues;
use std::sync::{Arc, OnceLock};

#[derive(Debug)]
pub struct LongArrayPropertySimilarityComputer {
    values: Arc<dyn NodePropertyValues>,
    metric: SimilarityMetric,
    cache: Vec<OnceLock<Option<Vec<i64>>>>,
}

impl LongArrayPropertySimilarityComputer {
    pub fn new(values: Arc<dyn NodePropertyValues>, metric: SimilarityMetric) -> Self {
        let node_count = values.node_count();
        let cache = (0..node_count).map(|_| OnceLock::new()).collect();
        Self {
            values,
            metric,
            cache,
        }
    }

    fn sorted(&self, node_id: u64) -> Option<&[i64]> {
        let idx = node_id as usize;
        let slot = self.cache.get(idx)?;
        let opt = slot.get_or_init(|| {
            if !self.values.has_value(node_id) {
                return None;
            }
            let Ok(mut v) = self.values.long_array_value(node_id) else {
                return None;
            };
            v.sort_unstable();
            Some(v)
        });
        opt.as_deref()
    }
}

impl SimilarityComputer for LongArrayPropertySimilarityComputer {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let Some(left) = self.sorted(first_node_id) else {
            return 0.0;
        };
        let Some(right) = self.sorted(second_node_id) else {
            return 0.0;
        };

        match self.metric {
            SimilarityMetric::Jaccard | SimilarityMetric::Default => jaccard::metric(left, right),
            SimilarityMetric::Overlap => overlap::metric(left, right),
            _ => 0.0,
        }
    }

    fn is_symmetric(&self) -> bool {
        true
    }
}
