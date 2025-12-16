use crate::procedures::leiden::computation::leiden as leiden_fn;
use crate::procedures::leiden::{LeidenConfig, LeidenResult};

/// Builder for the Leiden community detection algorithm
///
/// Leiden is a state-of-the-art community detection algorithm that improves
/// upon Louvain by preventing disconnected communities through a refinement phase.
///
/// # Example
///
/// ```ignore
/// let result = LeidenBuilder::new()
///     .gamma(1.0)           // Resolution parameter
///     .theta(0.01)          // Randomness for refinement
///     .tolerance(0.0001)    // Convergence threshold
///     .max_iterations(10)   // Maximum levels
///     .build()
///     .run(graph);
/// ```
#[derive(Debug, Clone)]
pub struct LeidenBuilder {
    config: LeidenConfig,
}

impl Default for LeidenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LeidenBuilder {
    /// Create a new Leiden builder with default parameters
    pub fn new() -> Self {
        Self {
            config: LeidenConfig::default(),
        }
    }

    /// Set the resolution parameter (gamma)
    ///
    /// Higher values lead to more, smaller communities.
    /// Default: 1.0
    pub fn gamma(mut self, gamma: f64) -> Self {
        self.config.gamma = gamma;
        self
    }

    /// Set the randomness parameter (theta)
    ///
    /// Controls randomness in the refinement phase.
    /// Range: [0.0, 1.0] where 0.0 is deterministic.
    /// Default: 0.01
    pub fn theta(mut self, theta: f64) -> Self {
        self.config.theta = theta;
        self
    }

    /// Set the convergence tolerance
    ///
    /// Algorithm stops when modularity improvement < tolerance.
    /// Default: 0.0001
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.config.tolerance = tolerance;
        self
    }

    /// Set the maximum number of iterations/levels
    ///
    /// Default: 10
    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    /// Set initial community assignments (optional)
    ///
    /// If provided, algorithm starts with these communities instead of
    /// singleton communities. Useful for incremental community detection.
    pub fn seed_communities(mut self, seed: Vec<u64>) -> Self {
        self.config.seed_communities = Some(seed);
        self
    }

    /// Set random seed for reproducibility
    ///
    /// Default: 42
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
        self
    }

    /// Build the algorithm with the configured parameters
    pub fn build(self) -> LeidenAlgorithm {
        LeidenAlgorithm {
            config: self.config,
        }
    }
}

/// Configured Leiden algorithm ready to run
pub struct LeidenAlgorithm {
    config: LeidenConfig,
}

impl LeidenAlgorithm {
    /// Run Leiden on a graph represented by a neighbor function
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `get_neighbors` - Function that returns (neighbor_id, edge_weight) pairs for a node
    ///
    /// # Returns
    ///
    /// `LeidenResult` containing:
    /// - Community assignments for each node
    /// - Final modularity score
    /// - Number of levels executed
    /// - Convergence status
    /// - Modularity progression
    pub fn run<F>(self, node_count: usize, get_neighbors: F) -> LeidenResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        let storage = leiden_fn(node_count, get_neighbors, &self.config);
        storage.into_result()
    }

    /// Run Leiden on a graph with adjacency list representation
    pub fn run_on_adjacency_list(
        self,
        adjacency_list: &[Vec<(usize, f64)>],
    ) -> LeidenResult {
        let node_count = adjacency_list.len();
        let get_neighbors = |node: usize| adjacency_list[node].clone();
        self.run(node_count, get_neighbors)
    }

    /// Run Leiden on a graph with edge list representation
    ///
    /// Automatically builds adjacency list from edges.
    pub fn run_on_edge_list(
        self,
        node_count: usize,
        edges: &[(usize, usize, f64)],
    ) -> LeidenResult {
        // Build adjacency list
        let mut adjacency_list = vec![Vec::new(); node_count];
        for &(src, dst, weight) in edges {
            adjacency_list[src].push((dst, weight));
        }

        self.run_on_adjacency_list(&adjacency_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leiden_builder() {
        let builder = LeidenBuilder::new()
            .gamma(1.5)
            .theta(0.05)
            .tolerance(0.001)
            .max_iterations(20)
            .random_seed(123);

        // Builder creates correct config
        assert_eq!(builder.config.gamma, 1.5);
        assert_eq!(builder.config.theta, 0.05);
        assert_eq!(builder.config.tolerance, 0.001);
        assert_eq!(builder.config.max_iterations, 20);
        assert_eq!(builder.config.random_seed, 123);
    }

    #[test]
    fn test_leiden_run() {
        // Simple triangle graph
        let edges = vec![
            (0, 1, 1.0),
            (1, 0, 1.0),
            (1, 2, 1.0),
            (2, 1, 1.0),
            (2, 0, 1.0),
            (0, 2, 1.0),
        ];

        let result = LeidenBuilder::new()
            .build()
            .run_on_edge_list(3, &edges);

        assert_eq!(result.communities.len(), 3);
        assert_eq!(result.community_count, 1); // Fully connected = 1 community
        assert!(result.modularity <= 0.0); // Modularity of fully connected graph
    }
}
