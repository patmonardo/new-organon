//! PageRank Distribution Computation
//!
//! **Translation Source**:
//! - `org.neo4j.gds.algorithms.centrality.PageRankDistribution`
//! - `org.neo4j.gds.algorithms.centrality.PageRankDistributionComputer`
//!
//! This module provides distribution computation for PageRank and other centrality algorithms.

use super::result::CentralityAlgorithmResult;
use crate::algo::core::statistics::{StatisticsConfig, StatisticsEngine};
use std::collections::HashMap;

/// PageRank distribution result
///
/// Translation of: `org.neo4j.gds.algorithms.centrality.PageRankDistribution`
///
/// ## Java GDS Source
///
/// ```java
/// public class PageRankDistribution {
///     public final Map<String, Object> centralitySummary;
///     public final long postProcessingMillis;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PageRankDistribution {
    /// Centrality statistics summary
    pub centrality_summary: HashMap<String, f64>,
    /// Optional error message (e.g., histogram not available)
    pub error: Option<String>,
    /// Post-processing time in milliseconds
    pub post_processing_millis: u64,
}

impl PageRankDistribution {
    /// Create a new PageRank distribution result
    ///
    /// Translation of: Constructor (lines 29-32)
    pub fn new(
        centrality_summary: HashMap<String, f64>,
        error: Option<String>,
        post_processing_millis: u64,
    ) -> Self {
        Self {
            centrality_summary,
            error,
            post_processing_millis,
        }
    }
}

/// Compute PageRank distribution statistics
///
/// Translation of: `org.neo4j.gds.algorithms.centrality.PageRankDistributionComputer`
///
/// ## Java GDS Source
///
/// ```java
/// public static PageRankDistribution computeDistribution(
///     PageRankResult result,
///     RankConfig configuration,
///     boolean shouldComputeCentralityDistribution
/// ) {
///     // Compute centrality statistics using CentralityStatistics.centralityStatistics()
///     // Handle LOG scaler case with error message
///     // Return PageRankDistribution with summary and timing
/// }
/// ```
pub struct PageRankDistributionComputer;

impl PageRankDistributionComputer {
    /// Compute distribution statistics for centrality results
    ///
    /// Translation of: `computeDistribution()` (lines 38-66)
    ///
    /// ## Parameters
    ///
    /// - `result`: Centrality algorithm result
    /// - `use_log_scaler`: Whether LOG scaler was used (prevents histogram computation)
    /// - `should_compute_distribution`: Whether to compute distribution statistics
    /// - `concurrency`: Number of threads for parallel computation
    ///
    /// ## Returns
    ///
    /// `PageRankDistribution` with statistics summary and processing time
    pub fn compute_distribution<R>(
        result: &R,
        use_log_scaler: bool,
        should_compute_distribution: bool,
        concurrency: usize,
    ) -> PageRankDistribution
    where
        R: CentralityAlgorithmResult,
    {
        let mut centrality_summary = HashMap::new();
        let mut error: Option<String> = None;
        let mut post_processing_millis = 0;

        if should_compute_distribution {
            if use_log_scaler {
                // LOG scaler prevents histogram computation
                // Java stores a String in the summary map; we keep summary numeric
                // and carry the message separately.
                error =
                    Some("Unable to create histogram when using scaler of type LOG".to_string());
            } else {
                let start = std::time::Instant::now();

                // Compute statistics using our enhanced core
                // Translation of: CentralityStatistics.centralityStatistics() (lines 54-60)
                let config = StatisticsConfig {
                    compute_histogram: true,
                    concurrency,
                    ..Default::default()
                };

                let score_fn = result.centrality_score_provider();
                let node_count = result.node_property_values().node_count();

                // Create a thread-safe closure by cloning the scores
                let scores: Vec<f64> = (0..node_count).map(score_fn).collect();

                if let Ok((summary, _histogram)) = StatisticsEngine::compute_statistics(
                    node_count,
                    move |node_id| scores[node_id],
                    config,
                ) {
                    // Build centrality summary map
                    // Translation of: CentralityStatistics.centralitySummary() (line 62)
                    centrality_summary.insert("min".to_string(), summary.min);
                    centrality_summary.insert("max".to_string(), summary.max);
                    centrality_summary.insert("mean".to_string(), summary.mean);
                    centrality_summary.insert("p50".to_string(), summary.percentiles.p50);
                    centrality_summary.insert("p75".to_string(), summary.percentiles.p75);
                    centrality_summary.insert("p90".to_string(), summary.percentiles.p90);
                    centrality_summary.insert("p95".to_string(), summary.percentiles.p95);
                    centrality_summary.insert("p99".to_string(), summary.percentiles.p99);
                    centrality_summary.insert("p999".to_string(), summary.percentiles.p999);
                }

                post_processing_millis = start.elapsed().as_millis() as u64;
            }
        }

        PageRankDistribution::new(centrality_summary, error, post_processing_millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::algorithms::centrality::result::CentralityAlgorithmResult;
    use crate::types::properties::node::NodePropertyValues;
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
    fn test_distribution_computation() {
        let result = TestCentralityResult {
            scores: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            node_count: 5,
        };

        let distribution = PageRankDistributionComputer::compute_distribution(
            &result, false, // use_log_scaler
            true,  // should_compute_distribution
            1,     // concurrency
        );

        // Check that statistics were computed
        assert!(distribution.centrality_summary.contains_key("min"));
        assert!(distribution.centrality_summary.contains_key("max"));
        assert!(distribution.centrality_summary.contains_key("mean"));
        assert!(distribution.centrality_summary.contains_key("p50"));
        assert!(distribution.centrality_summary.contains_key("p75"));
        assert!(distribution.centrality_summary.contains_key("p90"));
        assert!(distribution.centrality_summary.contains_key("p95"));
        assert!(distribution.centrality_summary.contains_key("p99"));
        assert!(distribution.centrality_summary.contains_key("p999"));

        // Check values
        assert_eq!(distribution.centrality_summary["min"], 0.1);
        assert_eq!(distribution.centrality_summary["max"], 0.5);
        assert_eq!(distribution.centrality_summary["mean"], 0.3);
        assert!(distribution.error.is_none());
    }

    #[test]
    fn test_log_scaler_error() {
        let result = TestCentralityResult {
            scores: vec![0.1, 0.2, 0.3],
            node_count: 3,
        };

        let distribution = PageRankDistributionComputer::compute_distribution(
            &result, true, // use_log_scaler
            true, // should_compute_distribution
            1,    // concurrency
        );

        // Should have error message instead of statistics
        assert!(distribution.centrality_summary.is_empty());
        assert!(distribution.error.is_some());
    }

    #[test]
    fn test_no_distribution_computation() {
        let result = TestCentralityResult {
            scores: vec![0.1, 0.2, 0.3],
            node_count: 3,
        };

        let distribution = PageRankDistributionComputer::compute_distribution(
            &result, false, // use_log_scaler
            false, // should_compute_distribution
            1,     // concurrency
        );

        // Should have empty summary
        assert!(distribution.centrality_summary.is_empty());
        assert!(distribution.error.is_none());
        assert_eq!(distribution.post_processing_millis, 0);
    }
}
