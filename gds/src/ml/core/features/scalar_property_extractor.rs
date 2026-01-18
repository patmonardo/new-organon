//! Scalar property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core ScalarPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{FeatureExtractor, ScalarFeatureExtractor};
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

/// Extracts scalar features from node properties.
///
/// This corresponds to the ScalarPropertyExtractor class in Java GDS.
/// Package-private constructor in Java - use through FeatureExtraction.propertyExtractors().
pub struct ScalarPropertyExtractor {
    property_key: String,
    node_property_values: Arc<dyn NodePropertyValues>,
    value_type: ValueType,
}

impl ScalarPropertyExtractor {
    /// Create a new scalar property extractor.
    ///
    /// Package-private in Java (no pub visibility modifier).
    #[allow(dead_code)]
    pub(crate) fn new(
        property_key: String,
        node_property_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        let value_type = node_property_values.value_type();
        Self {
            property_key,
            node_property_values,
            value_type,
        }
    }
}

impl FeatureExtractor for ScalarPropertyExtractor {
    fn dimension(&self) -> usize {
        1
    }
}

impl ScalarFeatureExtractor for ScalarPropertyExtractor {
    fn extract(&self, node_id: u64) -> f64 {
        let value = match self.value_type {
            ValueType::Double => self
                .node_property_values
                .double_value(node_id)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed reading property `{}` for node `{}`: {e}",
                        self.property_key, node_id
                    )
                }),
            ValueType::Long => self
                .node_property_values
                .long_value(node_id)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed reading property `{}` for node `{}`: {e}",
                        self.property_key, node_id
                    )
                }) as f64,
            other => panic!(
                "ScalarPropertyExtractor expected scalar Double/Long but property `{}` is {:?}",
                self.property_key, other
            ),
        };

        if value.is_nan() {
            panic!(
                "Node with ID `{}` has invalid feature property value `NaN` for property `{}`",
                node_id, self.property_key
            );
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_is_one() {
        struct MockScalarExtractor;
        impl FeatureExtractor for MockScalarExtractor {
            fn dimension(&self) -> usize {
                1
            }
        }
        let extractor = MockScalarExtractor;
        assert_eq!(FeatureExtractor::dimension(&extractor), 1);
    }
}
