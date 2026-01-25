//! Algorithm configuration types and builders

use super::base_types::{ConcurrencyConfig, IterationsConfig};

// PageRankConfig canonicalized into the algorithm spec (`algo/pagerank/spec.rs`).
// Backwards-compatibility re-export removed per migration policy.
// LouvainConfig moved to algorithm spec (algo/louvain/spec.rs). Re-export for compatibility.
pub use crate::algo::louvain::spec::LouvainConfig;

// The algorithm spec struct is not a generated `Config` type, so implement `Config` marker here.
impl crate::config::Config for LouvainConfig {}

impl ConcurrencyConfig for LouvainConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl IterationsConfig for LouvainConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        Some(self.tolerance)
    }
}

// NodeSimilarityConfig is defined in the algorithm spec (algo/similarity/node_similarity/spec.rs)
// and re-exported here so consumer code can reference `crate::config::NodeSimilarityConfig`.
pub use crate::algo::similarity::node_similarity::NodeSimilarityConfig;

// The algo spec struct is not a generated `Config` type, so implement the marker
// `Config` trait here to satisfy base-type bounds (e.g., `ConcurrencyConfig`).
impl crate::config::Config for NodeSimilarityConfig {}

impl ConcurrencyConfig for NodeSimilarityConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

// Betweenness config canonicalized to algo spec (algo/betweenness/spec.rs)
pub use crate::algo::betweenness::spec::BetweennessCentralityConfig;

impl crate::config::Config for BetweennessCentralityConfig {}

impl ConcurrencyConfig for BetweennessCentralityConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config_trait::ValidatedConfig;

    #[test]
    fn test_louvain_default() {
        let config = LouvainConfig::default();
        assert_eq!(config.max_iterations, 10);
        assert_eq!(config.gamma, 1.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_node_similarity_default() {
        let config = NodeSimilarityConfig::default();
        assert_eq!(config.top_k, 10);
        // Default follows algo spec (0.1)
        assert_eq!(config.similarity_cutoff, 0.1);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_betweenness_centrality_config_fields() {
        let mut config = BetweennessCentralityConfig::default();
        config.sampling_size = Some(100);
        config.random_seed = 42;

        assert_eq!(config.sampling_size, Some(100));
        assert_eq!(config.random_seed, 42);
    }
}
