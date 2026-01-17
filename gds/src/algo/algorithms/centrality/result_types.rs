//! Centrality Algorithm Result Types
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult`
//!
//! This module provides result types and traits for centrality algorithms.

use crate::types::properties::node::NodePropertyValues;

/// Result trait for centrality algorithms
///
/// Translation of: `org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult`
///
/// ## Java GDS Source
///
/// ```java
/// public interface CentralityAlgorithmResult {
///     NodePropertyValues nodePropertyValues();
///     LongToDoubleFunction centralityScoreProvider();
/// }
/// ```
///
/// ## Usage
///
/// ```rust,ignore
/// use gds::algo::algorithms::centrality::CentralityAlgorithmResult;
///
/// struct PageRankResult {
///     scores: Vec<f64>,
/// }
///
/// impl CentralityAlgorithmResult for PageRankResult {
///     fn node_property_values(&self) -> &dyn NodePropertyValues {
///         // Return property values accessor
///     }
///
///     fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64> {
///         let scores = self.scores.clone();
///         Box::new(move |node_id| scores[node_id])
///     }
/// }
/// ```
pub trait CentralityAlgorithmResult {
    /// Get node property values accessor
    ///
    /// Translation of: `NodePropertyValues nodePropertyValues()`
    fn node_property_values(&self) -> &dyn NodePropertyValues;

    /// Get centrality score provider function
    ///
    /// Translation of: `LongToDoubleFunction centralityScoreProvider()`
    fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
    use crate::types::ValueType;
    use std::fmt;

    // Test implementation
    struct TestCentralityResult {
        scores: Vec<f64>,
        node_count: usize,
    }

    impl fmt::Debug for TestCentralityResult {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TestCentralityResult")
                .field("node_count", &self.node_count)
                .finish()
        }
    }

    impl PropertyValues for TestCentralityResult {
        fn value_type(&self) -> ValueType {
            ValueType::Double
        }

        fn element_count(&self) -> usize {
            self.node_count
        }
    }

    impl NodePropertyValues for TestCentralityResult {
        fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
            self.scores
                .get(node_id as usize)
                .copied()
                .ok_or(PropertyValuesError::InvalidNodeId(node_id))
        }

        fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
            Err(PropertyValuesError::unsupported_type(
                ValueType::Double,
                ValueType::Long,
            ))
        }

        fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
            Err(PropertyValuesError::unsupported_operation(
                "double_array_value not supported",
            ))
        }

        fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
            Err(PropertyValuesError::unsupported_operation(
                "float_array_value not supported",
            ))
        }

        fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
            Err(PropertyValuesError::unsupported_operation(
                "long_array_value not supported",
            ))
        }

        fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
            Err(PropertyValuesError::unsupported_operation(
                "get_object not supported",
            ))
        }

        fn dimension(&self) -> Option<usize> {
            Some(1)
        }

        fn get_max_long_property_value(&self) -> Option<i64> {
            None
        }

        fn get_max_double_property_value(&self) -> Option<f64> {
            self.scores
                .iter()
                .copied()
                .filter(|v| v.is_finite())
                .reduce(f64::max)
        }

        fn has_value(&self, node_id: u64) -> bool {
            (node_id as usize) < self.node_count
        }
    }

    impl CentralityAlgorithmResult for TestCentralityResult {
        fn node_property_values(&self) -> &dyn NodePropertyValues {
            self
        }

        fn centrality_score_provider(&self) -> Box<dyn Fn(usize) -> f64> {
            let scores = self.scores.clone();
            Box::new(move |node_id| scores[node_id])
        }
    }

    #[test]
    fn test_centrality_result_trait() {
        let result = TestCentralityResult {
            scores: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            node_count: 5,
        };

        assert_eq!(result.node_property_values().node_count(), 5);
        assert_eq!(
            result.node_property_values().value_type(),
            ValueType::Double
        );

        let score_fn = result.centrality_score_provider();
        assert_eq!(score_fn(0), 0.1);
        assert_eq!(score_fn(2), 0.3);
        assert_eq!(score_fn(4), 0.5);
    }
}
