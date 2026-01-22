//! Incremental Community Detection Support
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community.LongIfChangedNodePropertyValues`
//!
//! This module provides node property values that only writes changed values for incremental algorithms.

use crate::types::properties::node::{LongNodePropertyValues, NodePropertyValues};
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;

/// Node property values that only writes changed values (for incremental algorithms)
///
/// Translation of: `org.neo4j.gds.algorithms.community.LongIfChangedNodePropertyValues`
///
/// ## Java GDS Source
///
/// ```java
/// public final class LongIfChangedNodePropertyValues implements LongNodePropertyValues, FilteredNodePropertyValuesMarker {
///     private final NodePropertyValues seedProperties;
///     private final NodePropertyValues newProperties;
///
///     public static LongNodePropertyValues of(NodeProperty seedProperty, LongNodePropertyValues newProperties) {
///         // Check if seed property is persistent
///         // Return LongIfChangedNodePropertyValues or newProperties based on property state
///     }
///
///     @Override
///     public long longValue(long nodeId) {
///         var seedValue = seedProperties.longValue(nodeId);
///         var writeValue = newProperties.longValue(nodeId);
///         return (seedValue != writeValue) ? writeValue : Long.MIN_VALUE;
///     }
/// }
/// ```
///
/// ## Purpose
///
/// For incremental community detection algorithms (e.g., iterative Louvain), we only want to write
/// nodes whose community assignment changed. This class filters out unchanged values to reduce
/// I/O and improve performance.
///
/// ## Example
///
/// ```rust,ignore
/// use gds::algo::algorithms::community::LongIfChangedNodePropertyValues;
///
/// let seed_props = /* previous iteration results */;
/// let new_props = /* current iteration results */;
/// let incremental = LongIfChangedNodePropertyValues::new(seed_props, new_props);
///
/// // Only nodes whose community changed will have values written
/// // Unchanged nodes return i64::MIN (sentinel value)
/// ```
#[derive(Debug)]
pub struct LongIfChangedNodePropertyValues {
    /// Previous iteration property values (seed)
    seed_properties: Box<dyn LongNodePropertyValues>,
    /// Current iteration property values (new)
    new_properties: Box<dyn LongNodePropertyValues>,
}

impl LongIfChangedNodePropertyValues {
    /// Create incremental property values
    ///
    /// Translation of: `of()` (lines 36-54)
    ///
    /// ## Parameters
    ///
    /// - `seed_properties`: Previous iteration property values
    /// - `new_properties`: Current iteration property values
    ///
    /// ## Returns
    ///
    /// Node property values that only writes changed values
    ///
    /// ## Note
    ///
    /// In the Java version, this method checks if the seed property is persistent.
    /// If not persistent, it returns the new properties directly. For now, we always
    /// create the incremental wrapper.
    pub fn new(
        seed_properties: Box<dyn LongNodePropertyValues>,
        new_properties: Box<dyn LongNodePropertyValues>,
    ) -> Self {
        Self {
            seed_properties,
            new_properties,
        }
    }
}

impl PropertyValues for LongIfChangedNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.seed_properties
            .element_count()
            .max(self.new_properties.element_count())
    }
}

impl NodePropertyValues for LongIfChangedNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::UnsupportedType {
            actual: ValueType::Long,
            expected: ValueType::Double,
        })
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        let seed_value = if (node_id as usize) < self.seed_properties.element_count() {
            self.seed_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };
        let write_value = if (node_id as usize) < self.new_properties.element_count() {
            self.new_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };

        if seed_value != write_value {
            Ok(write_value)
        } else {
            Ok(i64::MIN) // Sentinel: don't write unchanged values
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
        let seed_value = if (node_id as usize) < self.seed_properties.element_count() {
            self.seed_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };
        let write_value = if (node_id as usize) < self.new_properties.element_count() {
            self.new_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };
        seed_value == i64::MIN || (seed_value != write_value)
    }
}

impl LongNodePropertyValues for LongIfChangedNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        let seed_value = if (node_id as usize) < self.seed_properties.element_count() {
            self.seed_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };
        let write_value = if (node_id as usize) < self.new_properties.element_count() {
            self.new_properties.long_value_unchecked(node_id)
        } else {
            i64::MIN
        };

        if seed_value != write_value {
            write_value
        } else {
            i64::MIN // Sentinel: don't write unchanged values
        }
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
    fn test_incremental_values_changed() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2, 3, 4, 5],
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 6, 4, 7], // Changed: index 2 (3->6), index 4 (5->7)
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.element_count(), 5);
        assert_eq!(incremental.value_type(), ValueType::Long);

        // Unchanged values should return i64::MIN
        assert_eq!(incremental.long_value(0).unwrap(), i64::MIN); // 1 -> 1 (unchanged)
        assert_eq!(incremental.long_value(1).unwrap(), i64::MIN); // 2 -> 2 (unchanged)
        assert_eq!(incremental.long_value(3).unwrap(), i64::MIN); // 4 -> 4 (unchanged)

        // Changed values should return new value
        assert_eq!(incremental.long_value(2).unwrap(), 6); // 3 -> 6 (changed)
        assert_eq!(incremental.long_value(4).unwrap(), 7); // 5 -> 7 (changed)

        // Check has_value
        assert!(!incremental.has_value(0)); // Unchanged
        assert!(!incremental.has_value(1)); // Unchanged
        assert!(incremental.has_value(2)); // Changed
        assert!(!incremental.has_value(3)); // Unchanged
        assert!(incremental.has_value(4)); // Changed
    }

    #[test]
    fn test_incremental_values_no_seed() {
        let seed = Box::new(TestLongProperty {
            values: vec![i64::MIN, i64::MIN, i64::MIN], // No seed values
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // New values
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.element_count(), 3);

        // All values should be written (no seed values)
        assert_eq!(incremental.long_value(0).unwrap(), 1);
        assert_eq!(incremental.long_value(1).unwrap(), 2);
        assert_eq!(incremental.long_value(2).unwrap(), 3);

        assert!(incremental.has_value(0));
        assert!(incremental.has_value(1));
        assert!(incremental.has_value(2));
    }

    #[test]
    fn test_incremental_values_all_unchanged() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2, 3],
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // Same values
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        assert_eq!(incremental.element_count(), 3);

        // All values should be filtered out (unchanged)
        assert_eq!(incremental.long_value(0).unwrap(), i64::MIN);
        assert_eq!(incremental.long_value(1).unwrap(), i64::MIN);
        assert_eq!(incremental.long_value(2).unwrap(), i64::MIN);

        assert!(!incremental.has_value(0));
        assert!(!incremental.has_value(1));
        assert!(!incremental.has_value(2));
    }

    #[test]
    fn test_incremental_values_different_sizes() {
        let seed = Box::new(TestLongProperty {
            values: vec![1, 2], // Smaller
        });
        let new = Box::new(TestLongProperty {
            values: vec![1, 2, 3], // Larger
        });

        let incremental = LongIfChangedNodePropertyValues::new(seed, new);

        // Should use maximum size
        assert_eq!(incremental.element_count(), 3);

        // Existing nodes
        assert_eq!(incremental.long_value(0).unwrap(), i64::MIN); // 1 -> 1 (unchanged)
        assert_eq!(incremental.long_value(1).unwrap(), i64::MIN); // 2 -> 2 (unchanged)

        // New node (no seed value)
        assert_eq!(incremental.long_value(2).unwrap(), 3); // No seed -> 3 (changed)

        assert!(!incremental.has_value(0));
        assert!(!incremental.has_value(1));
        assert!(incremental.has_value(2));
    }
}
