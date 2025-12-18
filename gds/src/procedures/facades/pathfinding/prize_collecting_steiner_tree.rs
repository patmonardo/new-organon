use crate::procedures::prize_collecting_steiner_tree::computation::PCSTreeComputationRuntime;
use crate::procedures::prize_collecting_steiner_tree::{PCSTreeConfig, PCSTreeResult};

/// Builder for the Prize-Collecting Steiner Tree (PCST) algorithm
///
/// PCST finds a tree that balances edge costs against node prizes. Unlike regular
/// Steiner Tree, PCST can choose which nodes to include based on their value.
///
/// This is used in network design, facility location, and feature selection problems
/// where including certain nodes has value (prize) but adding edges has cost.
///
/// # Example
///
/// ```ignore
/// let result = PCSTreeBuilder::new()
///     .prizes(vec![0.0, 5.0, 10.0, 3.0, 8.0]) // Prize for including each node
///     .build()
///     .run(graph);
/// ```
#[derive(Debug, Clone)]
pub struct PCSTreeBuilder {
    config: PCSTreeConfig,
}

impl Default for PCSTreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PCSTreeBuilder {
    /// Create a new Prize-Collecting Steiner Tree builder
    pub fn new() -> Self {
        Self {
            config: PCSTreeConfig {
                prizes: Vec::new(),
                relationship_weight_property: None,
            },
        }
    }

    /// Set the prizes for each node
    ///
    /// `prizes[i]` is the value gained by including node `i` in the tree.
    /// The algorithm seeks to maximize: sum(prizes) - sum(edge_costs)
    ///
    /// # Panics
    ///
    /// Will panic during execution if prizes.len() != node_count
    pub fn prizes(mut self, prizes: Vec<f64>) -> Self {
        self.config.prizes = prizes;
        self
    }

    /// Build the algorithm with the configured parameters
    pub fn build(self) -> PCSTreeAlgorithm {
        PCSTreeAlgorithm {
            config: self.config,
        }
    }
}

/// Configured Prize-Collecting Steiner Tree algorithm ready to run
pub struct PCSTreeAlgorithm {
    config: PCSTreeConfig,
}

impl PCSTreeAlgorithm {
    /// Run PCST on a graph represented by a neighbor function
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `get_neighbors` - Function that returns (neighbor_id, edge_weight) pairs for a node
    ///
    /// # Returns
    ///
    /// `PCSTreeResult` containing:
    /// - Parent array (tree structure)
    /// - Edge costs in the tree
    /// - Total prize collected
    /// - Total edge cost
    /// - Net value (prizes - costs)
    /// - Nodes included in the tree
    pub fn run<F>(self, node_count: usize, get_neighbors: F) -> PCSTreeResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        if self.config.prizes.len() != node_count {
            panic!(
                "Prize vector length ({}) must match node count ({})",
                self.config.prizes.len(),
                node_count
            );
        }

        let runtime = PCSTreeComputationRuntime::new(self.config);
        runtime.compute(node_count, get_neighbors)
    }

    /// Run on adjacency list
    pub fn run_on_adjacency_list(self, adjacency_list: &[Vec<(usize, f64)>]) -> PCSTreeResult {
        let node_count = adjacency_list.len();
        let get_neighbors = |node: usize| adjacency_list[node].clone();
        self.run(node_count, get_neighbors)
    }

    /// Run on edge list
    pub fn run_on_edge_list(
        self,
        node_count: usize,
        edges: &[(usize, usize, f64)],
    ) -> PCSTreeResult {
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
    fn test_pcst_builder() {
        let builder = PCSTreeBuilder::new().prizes(vec![0.0, 5.0, 10.0, 3.0]);

        // Builder creates correct config
        assert_eq!(builder.config.prizes, vec![0.0, 5.0, 10.0, 3.0]);
    }

    #[test]
    fn test_pcst_high_prize() {
        // Simple graph where node 1 has very high prize
        let edges = vec![
            (0, 1, 1.0),  // Cheap edge to high-prize node
            (1, 2, 10.0), // Expensive edge to low-prize node
        ];

        let prizes = vec![0.0, 100.0, 1.0]; // Node 1 has huge prize

        let result = PCSTreeBuilder::new()
            .prizes(prizes)
            .build()
            .run_on_edge_list(3, &edges);

        // Should have positive net value (high prize from node 1)
        assert!(result.net_value > 0.0); // Positive net value
        assert!(result.total_prize > 0.0); // Collected prizes
    }

    #[test]
    #[should_panic(expected = "Prize vector length")]
    fn test_pcst_wrong_prize_count() {
        let edges = vec![(0, 1, 1.0)];
        let prizes = vec![1.0]; // Only 1 prize but 2 nodes

        PCSTreeBuilder::new()
            .prizes(prizes)
            .build()
            .run_on_edge_list(2, &edges);
    }
}
