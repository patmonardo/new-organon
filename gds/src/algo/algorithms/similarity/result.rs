//! Similarity algorithm result types and builders
//!
//! **Translation Source**: `org.neo4j.gds.result.AbstractSimilarityResultBuilder`

use crate::algo::algorithms::{
    ExecutionMetadata, ResultBuilder, ResultBuilderError,
};
use crate::algo::algorithms::statistics::{
    Histogram, StatisticalSummary, StatisticsConfig, StatisticsEngine,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Similarity algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    /// Similarity scores
    pub scores: Vec<f64>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of scores
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
}

/// Similarity result builder
///
/// **Translation**: `AbstractSimilarityResultBuilder`
///
/// Provides result building for similarity algorithms with histogram computation.
pub struct SimilarityResultBuilder {
    scores: Vec<f64>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
}

impl SimilarityResultBuilder {
    /// Create a new similarity result builder
    pub fn new(scores: Vec<f64>) -> Self {
        Self {
            scores,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
        }
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

impl ResultBuilder<SimilarityResult> for SimilarityResultBuilder {
    fn build(self) -> Result<SimilarityResult, ResultBuilderError> {
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;

        // Compute statistics if requested and not already provided
        if self.compute_statistics && statistics.is_none() {
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };

            let (stats, hist) =
                StatisticsEngine::compute_statistics_from_values(self.scores.clone(), config)?;

            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }

        let metadata = self
            .metadata
            .unwrap_or_else(|| ExecutionMetadata::new(Duration::from_secs(0)));

        Ok(SimilarityResult {
            scores: self.scores,
            statistics,
            histogram,
            metadata,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_result_builder() {
        let scores = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let metadata = ExecutionMetadata::new(Duration::from_secs(3));

        let result = SimilarityResultBuilder::new(scores.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();

        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_some());
        assert!(result.histogram.is_some());
        assert_eq!(result.metadata.execution_time, Duration::from_secs(3));
    }
}
