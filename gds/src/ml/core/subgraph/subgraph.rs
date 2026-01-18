//! SubGraph implementation for GDS.
//!
//! Translated from Java GDS ml-core SubGraph.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{BatchNeighbors, LocalIdMap};
use crate::ml::core::neighborhood_function::NeighborhoodFunction;
use crate::ml::core::relationship_weights::{
    ClosureRelationshipWeights, RelationshipWeights, DEFAULT_VALUE, UNWEIGHTED,
};
use crate::types::graph::Graph;
use std::sync::Arc;

/// SubGraph represents a sampled neighborhood subgraph for batch processing.
///
/// This is the main implementation of BatchNeighbors used in GNN training.
#[derive(Clone)]
pub struct SubGraph {
    /// Local IDs of nodes in the input batch
    mapped_batch_node_ids: Vec<usize>,

    /// All original node IDs in the subgraph (batch + neighbors)
    original_node_ids: Vec<u64>,

    /// Adjacency list: neighbors[node_id] = [neighbor_ids...]
    neighbors: Vec<Vec<usize>>,

    /// Relationship weights function (graph-backed or unweighted).
    relationship_weights_function: Arc<dyn RelationshipWeights>,

    /// Whether the graph has relationship weights.
    weighted: bool,
}

impl SubGraph {
    /// Create a new SubGraph.
    pub fn new(
        mapped_batch_node_ids: Vec<usize>,
        original_node_ids: Vec<u64>,
        neighbors: Vec<Vec<usize>>,
        relationship_weights_function: Arc<dyn RelationshipWeights>,
        weighted: bool,
    ) -> Self {
        Self {
            mapped_batch_node_ids,
            original_node_ids,
            neighbors,
            relationship_weights_function,
            weighted,
        }
    }

    /// Build subgraphs for each neighborhood function (Java: buildSubGraphs).
    pub fn build_sub_graphs(
        batch_node_ids: &[u64],
        neighborhood_functions: &[Arc<dyn NeighborhoodFunction>],
        weight_function: Arc<dyn RelationshipWeights>,
        weighted: bool,
    ) -> Vec<SubGraph> {
        let mut result = Vec::with_capacity(neighborhood_functions.len());
        let mut previous_nodes = batch_node_ids.to_vec();

        for neighborhood_function in neighborhood_functions {
            let last_graph = SubGraph::build_sub_graph(
                &previous_nodes,
                neighborhood_function.as_ref(),
                Arc::clone(&weight_function),
                weighted,
            );
            previous_nodes = last_graph.original_node_ids.clone();
            result.push(last_graph);
        }

        result
    }

    /// Build a subgraph from a batch of node IDs (Java: buildSubGraph).
    pub fn build_sub_graph(
        batch_node_ids: &[u64],
        neighborhood_function: &dyn NeighborhoodFunction,
        weight_function: Arc<dyn RelationshipWeights>,
        weighted: bool,
    ) -> SubGraph {
        let mut mapped_batch_node_ids = vec![0usize; batch_node_ids.len()];

        let mut id_map = LocalIdMap::new();

        for (node_offset, &node_id) in batch_node_ids.iter().enumerate() {
            let mapped_node_id = id_map.to_mapped(node_id);
            mapped_batch_node_ids[node_offset] = mapped_node_id;
        }

        let batch_size = id_map.size();
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); batch_size];

        for mapped_node_id in 0..batch_size {
            let original_id = id_map.to_original(mapped_node_id);
            let neighbor_internal_ids = neighborhood_function
                .sample(original_id)
                .map(|id| id_map.to_mapped(id))
                .collect::<Vec<_>>();
            adjacency[mapped_node_id] = neighbor_internal_ids;
        }

        SubGraph::new(
            mapped_batch_node_ids,
            id_map.original_ids_vec(),
            adjacency,
            weight_function,
            weighted,
        )
    }

    /// Get the original node IDs for all nodes in the subgraph.
    pub fn original_node_ids(&self) -> &[u64] {
        &self.original_node_ids
    }

    /// Check if this subgraph has relationship weights.
    pub fn is_weighted(&self) -> bool {
        self.weighted
    }

    /// Java: SubGraph.relationshipWeightFunction(Graph graph)
    pub fn relationship_weight_function(graph: &dyn Graph) -> (Arc<dyn RelationshipWeights>, bool) {
        if graph.has_relationship_property() {
            let g = Graph::concurrent_copy(graph);
            (
                Arc::new(ClosureRelationshipWeights::new(move |s, t, default| {
                    g.relationship_property(s as i64, t as i64, default)
                })),
                true,
            )
        } else {
            (Arc::new(UNWEIGHTED), false)
        }
    }
}

impl BatchNeighbors for SubGraph {
    fn batch_ids(&self) -> &[usize] {
        &self.mapped_batch_node_ids
    }

    fn node_count(&self) -> usize {
        self.original_node_ids.len()
    }

    fn degree(&self, batch_id: usize) -> usize {
        self.neighbors[batch_id].len()
    }

    fn neighbors(&self, batch_id: usize) -> &[usize] {
        &self.neighbors[batch_id]
    }

    fn relationship_weight(&self, src: usize, trg: usize) -> f64 {
        if !self.weighted {
            return DEFAULT_VALUE;
        }

        self.relationship_weights_function
            .weight(self.original_node_ids[src], self.original_node_ids[trg])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subgraph_creation() {
        let subgraph = SubGraph::new(
            vec![0, 1, 2],
            vec![10, 20, 30, 40, 50],
            vec![vec![3, 4], vec![4], vec![3]],
            Arc::new(UNWEIGHTED),
            false,
        );

        assert_eq!(subgraph.batch_size(), 3);
        assert_eq!(subgraph.node_count(), 5);
        assert!(!subgraph.is_weighted());
    }

    #[test]
    fn test_subgraph_batch_neighbors() {
        let subgraph = SubGraph::new(
            vec![0, 1],
            vec![100, 200, 300, 400],
            vec![vec![2, 3], vec![3]],
            Arc::new(UNWEIGHTED),
            false,
        );

        assert_eq!(subgraph.degree(0), 2);
        assert_eq!(subgraph.degree(1), 1);
        assert_eq!(subgraph.neighbors(0), &[2, 3]);
        assert_eq!(subgraph.neighbors(1), &[3]);
    }

    #[test]
    fn test_subgraph_original_ids() {
        let original_ids = vec![10, 20, 30, 40];
        let subgraph = SubGraph::new(
            vec![0, 1],
            original_ids.clone(),
            vec![vec![2], vec![3]],
            Arc::new(UNWEIGHTED),
            false,
        );

        assert_eq!(subgraph.original_node_ids(), &original_ids[..]);
    }
}
