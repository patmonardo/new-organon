//! Result Builders - Algorithm result construction and processing
//!
//! **Translation Source**: `org.neo4j.gds.result.Abstract*ResultBuilder` classes
//! **Key Features**: Result building, statistics integration, histogram generation
//!
//! This module provides result building capabilities for different algorithm types,
//! integrating with our statistics and progress tracking modules.

use crate::algo::core::scaling::Scaler;
use crate::algo::core::statistics::{
    Histogram, StatisticalSummary, StatisticsConfig, StatisticsEngine,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Base result builder trait
pub trait ResultBuilder<T> {
    /// Build the final result
    fn build(self) -> Result<T, ResultBuilderError>;

    /// Add statistics to the result
    fn with_statistics(self, stats: StatisticalSummary) -> Self;

    /// Add histogram to the result
    fn with_histogram(self, histogram: Option<Histogram>) -> Self;

    /// Add execution metadata
    fn with_metadata(self, metadata: ExecutionMetadata) -> Self;
}

/// Execution metadata for algorithm results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Execution time
    pub execution_time: Duration,
    /// Number of iterations (if applicable)
    pub iterations: Option<u32>,
    /// Convergence status (if applicable)
    pub converged: Option<bool>,
    /// Additional metadata
    pub additional: HashMap<String, String>,
}

impl ExecutionMetadata {
    /// Create new execution metadata
    pub fn new(execution_time: Duration) -> Self {
        Self {
            execution_time,
            iterations: None,
            converged: None,
            additional: HashMap::new(),
        }
    }

    /// Add iteration count
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }

    /// Add convergence status
    pub fn with_convergence(mut self, converged: bool) -> Self {
        self.converged = Some(converged);
        self
    }

    /// Add additional metadata
    pub fn with_additional(mut self, key: String, value: String) -> Self {
        self.additional.insert(key, value);
        self
    }
}

/// Centrality algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Path finding algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathFindingResult {
    /// Paths found during execution
    pub paths: Vec<PathResult>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of path costs
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
    /// Post-processing time in milliseconds
    pub post_processing_millis: i64,
}

/// Path result for individual paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResult {
    pub source: u64,
    pub target: u64,
    pub path: Vec<u64>,
    pub cost: f64,
}

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

/// Community algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityResult {
    /// Community assignments for each node
    pub communities: Vec<u32>,
    /// Size of each community
    pub community_sizes: HashMap<u32, usize>,
    /// Total number of communities
    pub community_count: u32,
    /// Statistical summary of community sizes
    pub size_statistics: Option<StatisticalSummary>,
    /// Histogram of community sizes
    pub size_histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
    /// Post-processing time in milliseconds
    pub post_processing_millis: i64,
}

/// Centrality result builder
///
/// **Translation**: `AbstractCentralityResultBuilder`
///
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
            // TODO: Use centrality_function to compute scores when node count is available
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
                // TODO: Use centrality_function directly for histogram computation
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

/// Community result builder
///
/// **Translation**: `AbstractCommunityResultBuilder`
///
/// Provides result building for community detection algorithms with histogram computation
/// and community statistics.
pub struct CommunityResultBuilder {
    community_function: Option<Box<dyn Fn(u64) -> u32 + Send + Sync>>,
    communities: Option<Vec<u32>>,
    compute_statistics: bool,
    compute_histogram: bool,
    metadata: Option<ExecutionMetadata>,
    post_processing_millis: i64,
}

impl CommunityResultBuilder {
    /// Create a new community result builder
    pub fn new() -> Self {
        Self {
            community_function: None,
            communities: None,
            compute_statistics: true,
            compute_histogram: true,
            metadata: None,
            post_processing_millis: -1,
        }
    }

    /// Set the community function for computing community assignments
    pub fn with_community_function<F>(mut self, community_function: F) -> Self
    where
        F: Fn(u64) -> u32 + Send + Sync + 'static,
    {
        self.community_function = Some(Box::new(community_function));
        self
    }

    /// Set pre-computed communities (alternative to community function)
    pub fn with_communities(mut self, communities: Vec<u32>) -> Self {
        self.communities = Some(communities);
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

impl ResultBuilder<CommunityResult> for CommunityResultBuilder {
    fn build(mut self) -> Result<CommunityResult, ResultBuilderError> {
        use std::time::Instant;

        let post_processing_start = Instant::now();

        // Get or compute communities
        let communities = if let Some(communities) = self.communities {
            communities
        } else if let Some(ref _community_function) = self.community_function {
            // We need node count to compute communities - this should be provided via metadata
            // For now, return an error if we don't have communities
            // TODO: Use community_function to compute communities when node count is available
            return Err(ResultBuilderError::MissingData(
                "Either communities or node count must be provided".to_string(),
            ));
        } else {
            return Err(ResultBuilderError::MissingData(
                "No community data provided".to_string(),
            ));
        };

        // Compute community sizes
        let mut community_sizes: HashMap<u32, usize> = HashMap::new();
        for &community_id in &communities {
            *community_sizes.entry(community_id).or_insert(0) += 1;
        }

        let community_count = community_sizes.len() as u32;

        // Compute statistics for community sizes if requested
        let mut size_statistics = None;
        let mut size_histogram = None;

        if self.compute_statistics {
            let size_values: Vec<f64> = community_sizes.values().map(|&size| size as f64).collect();
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };

            let (stats, hist) =
                StatisticsEngine::compute_statistics_from_values(size_values, config)?;
            size_statistics = Some(stats);
            if self.compute_histogram {
                size_histogram = hist;
            }
        }

        self.post_processing_millis = post_processing_start.elapsed().as_millis() as i64;

        let metadata = self
            .metadata
            .unwrap_or_else(|| ExecutionMetadata::new(Duration::from_secs(0)));

        Ok(CommunityResult {
            communities,
            community_sizes,
            community_count,
            size_statistics,
            size_histogram,
            metadata,
            post_processing_millis: self.post_processing_millis,
        })
    }

    fn with_statistics(self, _stats: StatisticalSummary) -> Self {
        // For community results, statistics are computed from community sizes
        self
    }

    fn with_histogram(self, _hist: Option<Histogram>) -> Self {
        // For community results, histogram is computed from community sizes
        self
    }

    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for CommunityResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Path finding result builder
///
/// **Translation**: `AbstractPathFindingResultBuilder` (inferred from Java patterns)
///
/// Provides result building for path finding algorithms with histogram computation
/// and path statistics.
pub struct PathResultBuilder {
    paths: Option<Vec<PathResult>>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
    post_processing_millis: i64,
}

impl PathResultBuilder {
    /// Create a new path result builder
    pub fn new() -> Self {
        Self {
            paths: None,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
            post_processing_millis: -1,
        }
    }

    /// Set pre-computed paths
    pub fn with_paths(mut self, paths: Vec<PathResult>) -> Self {
        self.paths = Some(paths);
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

impl ResultBuilder<PathFindingResult> for PathResultBuilder {
    fn build(mut self) -> Result<PathFindingResult, ResultBuilderError> {
        use std::time::Instant;

        let post_processing_start = Instant::now();

        // Get paths
        let paths = self
            .paths
            .ok_or_else(|| ResultBuilderError::MissingData("No paths provided".to_string()))?;

        // Compute statistics if requested and not already provided
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;

        if self.compute_statistics && statistics.is_none() {
            // Compute path cost statistics
            let costs: Vec<f64> = paths.iter().map(|p| p.cost).collect();
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };

            let (stats, hist) = StatisticsEngine::compute_statistics_from_values(costs, config)?;

            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }

        self.post_processing_millis = post_processing_start.elapsed().as_millis() as i64;

        let metadata = self
            .metadata
            .unwrap_or_else(|| ExecutionMetadata::new(Duration::from_secs(0)));

        Ok(PathFindingResult {
            paths,
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

impl Default for PathResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Similarity result builder
///
/// **Translation**: `AbstractSimilarityResultBuilder`
///
/// Provides result building for similarity algorithms with histogram computation
/// and post-processing timing.
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

/// Result builder error
#[derive(Debug, thiserror::Error)]
pub enum ResultBuilderError {
    #[error("Statistics computation failed: {0}")]
    StatisticsError(#[from] crate::algo::core::statistics::StatisticsError),

    #[error("Invalid result data: {0}")]
    InvalidData(String),

    #[error("Builder configuration error: {0}")]
    ConfigurationError(String),

    #[error("Missing required data: {0}")]
    MissingData(String),
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
    fn test_community_result_builder() {
        let communities = vec![0, 0, 1, 1, 2];
        let metadata = ExecutionMetadata::new(Duration::from_secs(2));

        let result = CommunityResultBuilder::new()
            .with_communities(communities.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();

        assert_eq!(result.communities, communities);
        assert_eq!(result.community_count, 3);
        assert_eq!(result.community_sizes.get(&0), Some(&2));
        assert_eq!(result.community_sizes.get(&1), Some(&2));
        assert_eq!(result.community_sizes.get(&2), Some(&1));
        assert!(result.size_statistics.is_some());
        assert!(result.size_histogram.is_some());
    }

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

    #[test]
    fn test_execution_metadata() {
        let metadata = ExecutionMetadata::new(Duration::from_secs(5))
            .with_iterations(100)
            .with_convergence(true)
            .with_additional("algorithm".to_string(), "pagerank".to_string());

        assert_eq!(metadata.execution_time, Duration::from_secs(5));
        assert_eq!(metadata.iterations, Some(100));
        assert_eq!(metadata.converged, Some(true));
        assert_eq!(
            metadata.additional.get("algorithm"),
            Some(&"pagerank".to_string())
        );
    }
}
