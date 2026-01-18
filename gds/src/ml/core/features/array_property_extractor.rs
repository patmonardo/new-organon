//! Array property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core ArrayPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{ArrayFeatureExtractor, FeatureExtractor};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

/// Extracts array features from node properties.
pub struct ArrayPropertyExtractor {
    dimension: usize,
    property_key: String,
    node_property_values: Arc<dyn NodePropertyValues>,
    value_type: ValueType,
}

impl ArrayPropertyExtractor {
    pub(crate) fn new(
        dimension: usize,
        property_key: String,
        node_property_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        let value_type = node_property_values.value_type();
        Self {
            dimension,
            property_key,
            node_property_values,
            value_type,
        }
    }

    fn fetch_property_value(&self, node_id: u64) -> Option<Vec<f64>> {
        match self.value_type {
            ValueType::DoubleArray => self.node_property_values.double_array_value(node_id).ok(),
            ValueType::FloatArray => self
                .node_property_values
                .float_array_value(node_id)
                .ok()
                .map(|v| v.into_iter().map(|x| x as f64).collect()),
            ValueType::LongArray => self
                .node_property_values
                .long_array_value(node_id)
                .ok()
                .map(|v| v.into_iter().map(|x| x as f64).collect()),
            _ => None,
        }
    }
}

impl FeatureExtractor for ArrayPropertyExtractor {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl ArrayFeatureExtractor for ArrayPropertyExtractor {
    fn extract(&self, node_id: u64) -> Vec<f64> {
        let property_value = self.fetch_property_value(node_id).unwrap_or_else(|| {
            panic!(
                "Missing node property for property key `{}` on node with id `{}`. Consider using a default value in the property projection.",
                &self.property_key,
                node_id
            )
        });

        if property_value.len() != self.dimension {
            panic!(
                "The property `{}` on node `{}` contains arrays of differing lengths `{}` and `{}`.",
                &self.property_key,
                node_id,
                property_value.len(),
                self.dimension
            );
        }

        if property_value.iter().any(|val| val.is_nan()) {
            panic!(
                "Node with ID `{}` has invalid feature property value NaN for property `{}`",
                node_id, &self.property_key
            );
        }

        property_value
    }
}
