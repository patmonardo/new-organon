//! Centrality Algorithm Result Types
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult`
//!
//! This module provides result types and traits for centrality algorithms.

use crate::algo::algorithms::{
    ExecutionMetadata, ResultBuilder, ResultBuilderError,
};
use crate::algo::algorithms::scaling::Scaler;
use crate::algo::algorithms::statistics::{
    Histogram, StatisticalSummary, StatisticsConfig, StatisticsEngine,
};
use crate::types::properties::node::NodePropertyValues;
use std::time::Duration;

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

/// Centrality algorithm result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CentralityResult {
    /// Node centrality scores
    pub scores: Vec<f64>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of scores
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
    /// Post-processing time in milliseconds
    pub post_processing_millis: i64,
}

/// Provides result building for centrality algorithms with histogram computation,
/// scaling support, and post-processing timing.
pub struct CentralityResultBuilder {
    centrality_function: Option<Box<dyn Fn(u64) -> f64 + Send + Sync>>,
    scaler: Option<Box<dyn Scaler + Send + Sync>>,
    scores: Option<Vec<f64>>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
    post_processing_millis: i64,
}

impl CentralityResultBuilder {
    /// Create a new centrality result builder
    pub fn new() -> Self {
        Self {
            centrality_function: None,
            scaler: None,
            scores: None,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
            post_processing_millis: -1,
        }
    }

    /// Set the centrality function for histogram computation
    pub fn with_centrality_function<F>(mut self, centrality_function: F) -> Self
    where
        F: Fn(u64) -> f64 + Send + Sync + 'static,
    {
        self.centrality_function = Some(Box::new(centrality_function));
        self
    }

    /// Set the scaler for post-processing centrality values
    pub fn with_scaler<S: Scaler + Send + Sync + 'static>(mut self, scaler: S) -> Self {
        self.scaler = Some(Box::new(scaler));
        self
    }

    /// Set pre-computed scores (alternative to centrality function)
    pub fn with_scores(mut self, scores: Vec<f64>) -> Self {
        self.scores = Some(scores);
        self
    }

    /// Enable or disable statistics computation
    pub fn with_statistics(mut self, compute: bool) -> Self {
        self.compute_statistics = compute;
        self
    }

    /// Enable or disable histogram computation
    pub fn with_histogram(mut self, compute: bool) -> Self {
        self.compute_histogram = compute;
        self
    }

    /// Set execution metadata
    pub fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl ResultBuilder<CentralityResult> for CentralityResultBuilder {
    fn build(mut self) -> Result<CentralityResult, ResultBuilderError> {
        use std::time::Instant;

        let post_processing_start = Instant::now();
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;

        // Get or compute scores
        let scores = if let Some(scores) = self.scores {
            scores
        } else if let Some(ref _centrality_function) = self.centrality_function {
            // We need node count to compute scores - this should be provided via metadata
            // For now, return an error if we don't have scores
            // Note: computing scores via centrality_function is deferred until node count is available here.
            return Err(ResultBuilderError::MissingData(
                "Either scores or node count must be provided".to_string(),
            ));
        } else {
            return Err(ResultBuilderError::MissingData(
                "No centrality data provided".to_string(),
            ));
        };

        // Apply scaler if provided
        let final_scores = if let Some(ref scaler) = self.scaler {
            scores
                .iter()
                .enumerate()
                .map(|(node_id, &score)| scaler.scale_property(node_id as u64, &|_| score))
                .collect()
        } else {
            scores
        };

        // Compute statistics if requested and not already provided
        if self.compute_statistics && statistics.is_none() {
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };

            let (stats, hist) =
                StatisticsEngine::compute_statistics_from_values(final_scores.clone(), config)?;

            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }

        // Compute histogram directly from centrality function if available and requested
        if self.compute_histogram && histogram.is_none() {
            if let Some(ref _centrality_function) = self.centrality_function {
                // This would use the Java-style parallel histogram computation
                // For now, we'll use the existing StatisticsEngine
                // Note: histogram computation via centrality_function is deferred.
                let config = StatisticsConfig {
                    compute_histogram: true,
                    ..Default::default()
                };

                let (_, hist) =
                    StatisticsEngine::compute_statistics_from_values(final_scores.clone(), config)?;
                histogram = hist;
            }
        }

        self.post_processing_millis = post_processing_start.elapsed().as_millis() as i64;

        let metadata = self
            .metadata
            .unwrap_or_else(|| ExecutionMetadata::new(Duration::from_secs(0)));

        Ok(CentralityResult {
            scores: final_scores,
            statistics,
            histogram,
            metadata,
            post_processing_millis: self.post_processing_millis,
        })
    }

    fn with_statistics(mut self, stats: StatisticalSummary) -> Self {
        self.statistics = Some(stats);
        self
    }

    fn with_histogram(mut self, hist: Option<Histogram>) -> Self {
        self.histogram = hist;
        self
    }

    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for CentralityResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centrality_result_builder() {
        let scores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let metadata = ExecutionMetadata::new(Duration::from_secs(1));

        let result = CentralityResultBuilder::new()
            .with_scores(scores.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();

        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_some());
        assert!(result.histogram.is_some());
        assert_eq!(result.metadata.execution_time, Duration::from_secs(1));
    }

    #[test]
    fn test_result_builder_without_statistics() {
        let scores = vec![1.0, 2.0, 3.0];

        let result = CentralityResultBuilder::new()
            .with_scores(scores.clone())
            .with_statistics(false)
            .with_histogram(false)
            .build()
            .unwrap();

        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_none());
        assert!(result.histogram.is_none());
    }
}
