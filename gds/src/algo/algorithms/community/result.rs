//! Community algorithm result types and builders
//!
//! **Translation Source**: `org.neo4j.gds.result.AbstractCommunityResultBuilder`

use crate::algo::algorithms::{
    ExecutionMetadata, ResultBuilder, ResultBuilderError,
};
use crate::algo::algorithms::statistics::{
    Histogram, StatisticalSummary, StatisticsConfig, StatisticsEngine,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
            // Note: computing communities via community_function is deferred until node count is available here.
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
