//! Community Detection Companion Utilities
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community.CommunityCompanion`
//!
//! This module provides utility functions for community detection algorithms.

use super::consecutive_values::ConsecutiveLongNodePropertyValues;
use crate::types::properties::node::{LongNodePropertyValues, NodePropertyValues};
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;
use std::collections::HashMap;

#[derive(Debug)]
struct CommunitySizeFilter {
    inner: Box<dyn LongNodePropertyValues>,
    min_community_size: usize,
    community_sizes: HashMap<i64, usize>,
}

impl CommunitySizeFilter {
    fn new(inner: Box<dyn LongNodePropertyValues>, min_community_size: usize) -> Self {
        let mut community_sizes: HashMap<i64, usize> = HashMap::new();
        for node_id in 0..inner.node_count() {
            if inner.has_value(node_id as u64) {
                let cid = inner.long_value_unchecked(node_id as u64);
                if cid != i64::MIN {
                    *community_sizes.entry(cid).or_insert(0) += 1;
                }
            }
        }

        Self {
            inner,
            min_community_size,
            community_sizes,
        }
    }
}

impl PropertyValues for CommunitySizeFilter {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.inner.node_count()
    }
}

impl NodePropertyValues for CommunitySizeFilter {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::Double,
        })
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        if !self.inner.has_value(node_id) {
            return Ok(i64::MIN);
        }

        let cid = self.inner.long_value_unchecked(node_id);
        if cid == i64::MIN {
            return Ok(i64::MIN);
        }

        match self.community_sizes.get(&cid) {
            Some(size) if *size >= self.min_community_size => Ok(cid),
            _ => Ok(i64::MIN),
        }
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::DoubleArray,
        })
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::FloatArray,
        })
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::LongArray,
        })
    }

    fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::Unknown,
        })
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        // This would be complex to compute, so we'll skip for now
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        if !self.inner.has_value(node_id) {
            return false;
        }

        let cid = self.inner.long_value_unchecked(node_id);
        if cid == i64::MIN {
            return false;
        }

        self.community_sizes
            .get(&cid)
            .is_some_and(|size| *size >= self.min_community_size)
    }
}

impl LongNodePropertyValues for CommunitySizeFilter {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.long_value(node_id).unwrap_or(i64::MIN)
    }
}

/// Community algorithm companion utilities
///
/// Translation of: `org.neo4j.gds.algorithms.community.CommunityCompanion`
///
/// ## Java GDS Source
///
/// ```java
/// public final class CommunityCompanion {
///     public static NodePropertyValues nodePropertyValues(
///         boolean consecutiveIds,
///         LongNodePropertyValues nodeProperties
///     ) { /* ... */ }
///
///     public static NodePropertyValues nodePropertyValues(
///         boolean incremental,
///         String resultProperty,
///         String seedProperty,
///         boolean consecutiveIds,
///         LongNodePropertyValues nodeProperties,
///         Optional<Long> minCommunitySize,
///         Concurrency concurrency,
///         Supplier<NodeProperty> seedPropertySupplier
///     ) { /* ... */ }
/// }
/// ```
pub struct CommunityCompanion;

impl CommunityCompanion {
    /// Create node property values, optionally with consecutive IDs
    ///
    /// Translation of: `nodePropertyValues(boolean consecutiveIds, LongNodePropertyValues nodeProperties)`
    /// (lines 44-54)
    ///
    /// ## Parameters
    ///
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    ///
    /// ## Returns
    ///
    /// Node property values with optional consecutive ID mapping
    pub fn node_property_values(
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
    ) -> Box<dyn LongNodePropertyValues> {
        if consecutive_ids {
            Box::new(ConsecutiveLongNodePropertyValues::new(node_properties))
        } else {
            node_properties
        }
    }

    /// Create node property values with incremental detection support
    ///
    /// Translation of: `nodePropertyValues()` with incremental parameters (lines 56-98)
    ///
    /// ## Parameters
    ///
    /// - `incremental`: Whether this is an incremental algorithm
    /// - `result_property`: Name of result property
    /// - `seed_property`: Name of seed property
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    /// - `min_community_size`: Minimum community size filter (optional)
    /// - `concurrency`: Number of threads for parallel processing
    ///
    /// ## Returns
    ///
    /// Node property values with optional incremental filtering and consecutive ID mapping
    pub fn node_property_values_with_incremental(
        incremental: bool,
        result_property: &str,
        seed_property: &str,
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
        min_community_size: Option<usize>,
        _concurrency: usize,
    ) -> Box<dyn LongNodePropertyValues> {
        // Apply minimum community size filter if specified
        // Translation of: applySizeFilter() (lines 86-88)
        let filtered: Box<dyn LongNodePropertyValues> = match min_community_size {
            Some(min_size) => Box::new(CommunitySizeFilter::new(node_properties, min_size)),
            None => node_properties,
        };

        // Apply incremental filtering if needed
        // Translation of: lines 68-70
        let incremental_filtered = if incremental && result_property == seed_property {
            // Note: LongIfChangedNodePropertyValues is deferred until we have a seed property supplier.
            // For now, return filtered properties
            filtered
        } else {
            filtered
        };

        // Apply consecutive ID mapping
        Self::node_property_values(consecutive_ids, incremental_filtered)
    }

    /// Create node property values with minimum community size filter
    ///
    /// Translation of: `nodePropertyValues()` with min community size (lines 100-111)
    ///
    /// ## Parameters
    ///
    /// - `consecutive_ids`: Whether to remap community IDs to consecutive integers
    /// - `node_properties`: Original community property values
    /// - `min_community_size`: Minimum community size filter (optional)
    /// - `concurrency`: Number of threads for parallel processing
    ///
    /// ## Returns
    ///
    /// Node property values with optional community size filtering and consecutive ID mapping
    pub fn node_property_values_with_filter(
        consecutive_ids: bool,
        node_properties: Box<dyn LongNodePropertyValues>,
        min_community_size: Option<usize>,
        _concurrency: usize,
    ) -> Box<dyn LongNodePropertyValues> {
        let filtered: Box<dyn LongNodePropertyValues> = match min_community_size {
            Some(min_size) => Box::new(CommunitySizeFilter::new(node_properties, min_size)),
            None => node_properties,
        };

        Self::node_property_values(consecutive_ids, filtered)
    }

    /// Extract seeding node property values
    ///
    /// Translation of: `extractSeedingNodePropertyValues()` (lines 197-211)
    ///
    /// ## Parameters
    ///
    /// - `property_name`: Name of the seeding property
    ///
    /// ## Returns
    ///
    /// Node property values for seeding, or error if property not found or wrong type
    ///
    /// ## Deferred
    ///
    /// This method requires `NodePropertyContainer` which we haven't translated yet.
    /// Will be implemented when we translate the core API.
    pub fn extract_seeding_property(
        property_name: &str,
        // Note: Accept NodePropertyContainer when available.
    ) -> Result<Box<dyn NodePropertyValues>, String> {
        // Stub for now - will be implemented when we have full property system
        Err(format!(
            "Not yet implemented - requires NodePropertyContainer for property '{}'",
            property_name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::{PropertyValues, PropertyValuesResult};
    use crate::types::ValueType;

    // Test implementation
    #[derive(Debug)]
    struct TestLongProperty {
        values: Vec<i64>,
    }

    impl PropertyValues for TestLongProperty {
        fn value_type(&self) -> ValueType {
            ValueType::Long
        }

        fn element_count(&self) -> usize {
            self.values.len()
        }
    }

    impl NodePropertyValues for TestLongProperty {
        fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
            Err(PropertyValuesError::UnsupportedType {
                actual: ValueType::Long,
                expected: ValueType::Double,
            })
        }

        fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
            Ok(self.values[node_id as usize])
        }

        fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
            Err(PropertyValuesError::UnsupportedType {
                actual: ValueType::Long,
                expected: ValueType::DoubleArray,
            })
        }

        fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
            Err(PropertyValuesError::UnsupportedType {
                actual: ValueType::Long,
                expected: ValueType::FloatArray,
            })
        }

        fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
            Err(PropertyValuesError::UnsupportedType {
                actual: ValueType::Long,
                expected: ValueType::LongArray,
            })
        }

        fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
            Err(PropertyValuesError::UnsupportedType {
                actual: ValueType::Long,
                expected: ValueType::Unknown,
            })
        }

        fn dimension(&self) -> Option<usize> {
            Some(1)
        }

        fn get_max_long_property_value(&self) -> Option<i64> {
            self.values.iter().max().copied()
        }

        fn get_max_double_property_value(&self) -> Option<f64> {
            None
        }

        fn has_value(&self, node_id: u64) -> bool {
            self.values[node_id as usize] != i64::MIN
        }
    }

    impl LongNodePropertyValues for TestLongProperty {
        fn long_value_unchecked(&self, node_id: u64) -> i64 {
            self.values[node_id as usize]
        }
    }

    #[test]
    fn test_node_property_values_basic() {
        let props = Box::new(TestLongProperty {
            values: vec![1, 2, 3, 4, 5],
        });

        // Test without consecutive IDs
        let result = CommunityCompanion::node_property_values(false, props);
        assert_eq!(result.element_count(), 5);
        assert_eq!(result.long_value(0).unwrap(), 1);
        assert_eq!(result.long_value(4).unwrap(), 5);
    }

    #[test]
    fn test_node_property_values_with_consecutive_ids() {
        let props = Box::new(TestLongProperty {
            values: vec![10, 20, 10, 30, 20], // Non-consecutive IDs
        });

        // Test with consecutive IDs
        let result = CommunityCompanion::node_property_values(true, props);
        assert_eq!(result.element_count(), 5);

        // Should remap 10->0, 20->1, 30->2
        assert_eq!(result.long_value(0).unwrap(), 0); // 10 -> 0
        assert_eq!(result.long_value(1).unwrap(), 1); // 20 -> 1
        assert_eq!(result.long_value(2).unwrap(), 0); // 10 -> 0
        assert_eq!(result.long_value(3).unwrap(), 2); // 30 -> 2
        assert_eq!(result.long_value(4).unwrap(), 1); // 20 -> 1
    }

    #[test]
    fn test_node_property_values_with_filter() {
        let props = Box::new(TestLongProperty {
            values: vec![1, 1, 2, 3, 3],
        });

        // Test with minimum community size filter
        let result = CommunityCompanion::node_property_values_with_filter(
            false, // consecutive_ids
            props,
            Some(3), // min_community_size
            1,       // concurrency
        );

        assert_eq!(result.element_count(), 5);
        // sizes: 1->2, 2->1, 3->2; min=3 means everything filtered
        for node_id in 0..5 {
            assert!(!result.has_value(node_id as u64));
            assert_eq!(result.long_value(node_id as u64).unwrap(), i64::MIN);
        }
    }

    #[test]
    fn test_extract_seeding_property_stub() {
        let result = CommunityCompanion::extract_seeding_property("test_property");
        assert!(result.is_err());
        // Just verify we get an error - don't try to unwrap it due to Debug trait requirements
    }
}
