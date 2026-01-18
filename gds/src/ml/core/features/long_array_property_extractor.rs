//! Long array property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core LongArrayPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{ArrayFeatureExtractor, FeatureExtractor};
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

/// Array feature extractor backed by long-valued node properties.
pub struct LongArrayPropertyExtractor {
    dimension: usize,
    property_key: String,
    node_property_values: Arc<dyn NodePropertyValues>,
}

impl LongArrayPropertyExtractor {
    pub(crate) fn new(
        dimension: usize,
        property_key: String,
        node_property_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        Self {
            dimension,
            property_key,
            node_property_values,
        }
    }
}

impl FeatureExtractor for LongArrayPropertyExtractor {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl ArrayFeatureExtractor for LongArrayPropertyExtractor {
    fn extract(&self, node_id: u64) -> Vec<f64> {
        let values = self
            .node_property_values
            .long_array_value(node_id)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed reading property `{}` for node `{}`: {e}",
                    self.property_key, node_id
                )
            });

        if values.len() != self.dimension {
            panic!(
                "The property `{}` on node `{}` contains arrays of differing lengths `{}` and `{}`.",
                &self.property_key,
                node_id,
                values.len(),
                self.dimension
            );
        }

        values.into_iter().map(|v| v as f64).collect()
    }
}
