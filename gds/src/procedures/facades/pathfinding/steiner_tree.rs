use crate::procedures::steiner_tree::computation::SteinerTreeComputationRuntime;
use crate::procedures::steiner_tree::{SteinerTreeConfig, SteinerTreeResult};

/// Builder for the Steiner Tree algorithm
///
/// Finds the minimum-weight tree connecting a source node to a set of terminal nodes.
/// This is a fundamental problem in network design and optimization.
///
/// # Example
///
/// ```ignore
/// let result = SteinerTreeBuilder::new()
///     .source_node(0)
///     .target_nodes(vec![3, 5, 7])
///     .delta(0.5)           // Pruning sensitivity
///     .apply_rerouting(true) // Enable rerouting optimization
///     .build()
///     .run(graph);
/// ```
#[derive(Debug, Clone)]
pub struct SteinerTreeBuilder {
    config: SteinerTreeConfig,
}

impl Default for SteinerTreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SteinerTreeBuilder {
    /// Create a new Steiner Tree builder
    pub fn new() -> Self {
        Self {
            config: SteinerTreeConfig {
                source_node: 0,
                target_nodes: Vec::new(),
                relationship_weight_property: None,
                delta: 1.0,
                apply_rerouting: false,
            },
        }
    }

    /// Set the source node (root of the tree)
    pub fn source_node(mut self, source: u64) -> Self {
        self.config.source_node = source;
        self
    }

    /// Set the target nodes (terminals that must be connected)
    pub fn target_nodes(mut self, targets: Vec<u64>) -> Self {
        self.config.target_nodes = targets;
        self
    }

    /// Set the delta parameter for pruning
    ///
    /// Controls how aggressively non-terminal leaves are pruned.
    /// Higher values = more aggressive pruning.
    /// Default: 0.0 (no pruning)
    pub fn delta(mut self, delta: f64) -> Self {
        self.config.delta = delta;
        self
    }

    /// Enable or disable rerouting optimization
    ///
    /// When enabled, the algorithm attempts to find better paths by
    /// considering alternative routes through the partially built tree.
    /// Default: false
    pub fn apply_rerouting(mut self, apply: bool) -> Self {
        self.config.apply_rerouting = apply;
        self
    }

    /// Build the algorithm with the configured parameters
    pub fn build(self) -> SteinerTreeAlgorithm {
        SteinerTreeAlgorithm {
            config: self.config,
        }
    }
}

/// Configured Steiner Tree algorithm ready to run
pub struct SteinerTreeAlgorithm {
    config: SteinerTreeConfig,
}

impl SteinerTreeAlgorithm {
    /// Run Steiner Tree on a graph represented by a neighbor function
    ///
    /// # Arguments
    ///
    /// * `node_count` - Number of nodes in the graph
    /// * `get_neighbors` - Function that returns (neighbor_id, edge_weight) pairs for a node
    ///
    /// # Returns
    ///
    /// `SteinerTreeResult` containing:
    /// - Parent array (tree structure)
    /// - Cost to reach each node
    /// - Total tree cost
    /// - Nodes included in the tree
    pub fn run<F>(self, node_count: usize, get_neighbors: F) -> SteinerTreeResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        let runtime = SteinerTreeComputationRuntime::new(self.config);
        runtime.compute(node_count, get_neighbors)
    }

    /// Run on adjacency list
    pub fn run_on_adjacency_list(
        self,
        adjacency_list: &[Vec<(usize, f64)>],
    ) -> SteinerTreeResult {
        let node_count = adjacency_list.len();
        let get_neighbors = |node: usize| adjacency_list[node].clone();
        self.run(node_count, get_neighbors)
    }

    /// Run on edge list
    pub fn run_on_edge_list(
        self,
        node_count: usize,
        edges: &[(usize, usize, f64)],
    ) -> SteinerTreeResult {
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
    fn test_steiner_tree_builder() {
        let builder = SteinerTreeBuilder::new()
            .source_node(0)
            .target_nodes(vec![3, 5, 7])
            .delta(0.5)
            .apply_rerouting(true);

        // Builder creates correct config
        assert_eq!(builder.config.source_node, 0);
        assert_eq!(builder.config.target_nodes, vec![3, 5, 7]);
        assert_eq!(builder.config.delta, 0.5);
        assert_eq!(builder.config.apply_rerouting, true);
    }

    #[test]
    fn test_steiner_tree_simple() {
        // Path graph: 0 - 1 - 2 - 3
        let edges = vec![
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
        ];

        let result = SteinerTreeBuilder::new()
            .source_node(0)
            .target_nodes(vec![3])
            .build()
            .run_on_edge_list(4, &edges);

        assert_eq!(result.total_cost, 3.0); // Cost to reach node 3
        assert_eq!(result.effective_target_nodes_count, 1); // Found 1 target
    }
}
