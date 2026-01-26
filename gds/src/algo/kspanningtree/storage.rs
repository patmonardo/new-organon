//! KSpanningTree Storage Runtime

use super::computation::KSpanningTreeComputationRuntime;
use super::spec::KSpanningTreeResult;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;

/// KSpanningTree Storage Runtime - handles persistent data access and algorithm orchestration
pub struct KSpanningTreeStorageRuntime {
    /// Source node for spanning tree
    pub source_node: NodeId,
    /// Number of spanning trees to create (k)
    pub k: u64,
    /// Objective: "min" or "max"
    pub objective: String,
}

impl KSpanningTreeStorageRuntime {
    /// Create new KSpanningTree storage runtime
    pub fn new(source_node: NodeId, k: u64, objective: String) -> Self {
        Self {
            source_node,
            k,
            objective,
        }
    }

    /// Compute k-spanning tree using controller pattern
    ///
    /// **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree.compute()`
    ///
    /// This method orchestrates the k-spanning tree algorithm:
    /// 1. Compute MST using Prim's algorithm
    /// 2. Apply k-limiting logic to create exactly k spanning trees
    ///
    /// # Arguments
    ///
    /// * `computation` - Mutable reference to computation runtime for state management
    /// * `graph` - Optional graph interface for neighbor access
    /// * `progress_tracker` - Progress tracking interface
    ///
    /// # Returns
    ///
    /// A `Result` containing the `KSpanningTreeResult` or an error.
    pub fn compute_kspanningtree(
        &self,
        computation: &mut KSpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<KSpanningTreeResult, AlgorithmError> {
        let graph = graph.ok_or_else(|| {
            AlgorithmError::Execution("Graph interface required for k-spanning tree".to_string())
        })?;

        let node_count = graph.node_count();

        // Step 1: Compute MST using Prim's algorithm via spanning tree storage
        // NOTE: We avoid starting our own progress subtask here because callers often pass a
        // `TaskProgressTracker` with a leaf base task; nested `begin_subtask*` calls would
        // attempt to restart the same leaf task and panic. The spanning tree implementation
        // manages its own begin/end calls.
        let is_min = self.objective == "min";
        let mst_result = self.compute_mst_using_prim(graph, is_min, progress_tracker)?;

        // Step 2: Initialize computation runtime with MST data
        computation.initialize_from_mst(
            &mst_result
                .parent
                .iter()
                .map(|&x| x as i64)
                .collect::<Vec<_>>(),
            &mst_result.cost_to_parent,
            mst_result.total_weight,
            self.source_node as usize,
            node_count,
        );

        // Step 3: If k >= number of nodes, return MST as-is
        if self.k as usize >= node_count {
            return Ok(KSpanningTreeResult {
                parent: mst_result.parent.iter().map(|&x| x as i64).collect(),
                cost_to_parent: mst_result.cost_to_parent,
                total_cost: mst_result.total_weight,
                root: self.source_node as u64,
                node_count,
            });
        }

        // Step 4: Apply k-limiting logic
        computation.apply_k_limiting(self.k as usize, is_min, |node_id| {
            self.get_neighbors_from_graph(graph, node_id)
        });

        Ok(KSpanningTreeResult {
            parent: computation.get_parent().iter().map(|&x| x as i64).collect(),
            cost_to_parent: computation.get_cost_to_parent().clone(),
            total_cost: computation.get_total_cost(),
            root: self.source_node as u64,
            node_count,
        })
    }

    /// Compute MST using Prim's algorithm via spanning tree storage
    fn compute_mst_using_prim(
        &self,
        graph: &dyn Graph,
        is_min: bool,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<super::super::spanning_tree::SpanningTree, AlgorithmError> {
        use crate::algo::spanning_tree::{
            SpanningTreeComputationRuntime, SpanningTreeStorageRuntime,
        };

        // Create spanning tree runtimes
        let spanning_tree_storage = SpanningTreeStorageRuntime::new(
            self.source_node as u32,
            is_min,
            1, // concurrency
        );
        let mut spanning_tree_computation = SpanningTreeComputationRuntime::new(
            self.source_node as u32,
            is_min,
            graph.node_count() as u32,
            1, // concurrency
        );

        // Compute MST
        spanning_tree_storage.compute_spanning_tree(
            &mut spanning_tree_computation,
            Some(graph),
            2, // undirected
            progress_tracker,
        )
    }

    /// Get neighbors from graph interface
    fn get_neighbors_from_graph(&self, graph: &dyn Graph, node_id: usize) -> Vec<(usize, f64)> {
        let fallback = 1.0;
        graph
            .stream_relationships_weighted(node_id as i64, fallback)
            .map(|cursor| (cursor.target_id() as usize, cursor.weight()))
            .collect()
    }
}
