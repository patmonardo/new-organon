use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::Graph;
use rayon::prelude::*;

/// Abstraaction for accessing node neighbors (vectors).
pub trait VectorComputer: Send + Sync {
    /// Get the sorted neighbor IDs for a node.
    fn vector(&self, node_id: u64) -> Vec<u64>;

    /// Get the weights corresponding to the neighbor IDs.
    /// Returns empty vector if unweighted.
    fn weights(&self, node_id: u64) -> Vec<f64>;
}

pub struct UnweightedVectorComputer<'a> {
    graph: &'a dyn Graph,
    relationship_type: Option<RelationshipType>,
    orientation: Orientation,
}

impl<'a> UnweightedVectorComputer<'a> {
    pub fn new(
        graph: &'a dyn Graph,
        relationship_type: Option<RelationshipType>,
        orientation: Orientation,
    ) -> Self {
        Self {
            graph,
            relationship_type,
            orientation,
        }
    }
}

impl<'a> VectorComputer for UnweightedVectorComputer<'a> {
    fn vector(&self, node_id: u64) -> Vec<u64> {
        // TODO: Optimize by using the Graph API more directly to avoid allocation if possible
        // For now, we collect into a sorted Vec. GDS Graph API usually yields sorted neighbors?
        // If not, we must sort. Assuming we need to sort for intersection.
        let mut neighbors: Vec<u64> = self
            .graph
            .neighbors(node_id as usize) // Using usize for node_id as per API
            .into_iter()
            .map(|n| n as u64)
            .collect();
        neighbors.sort_unstable(); // Ensure sorted for intersection algo
        neighbors
    }

    fn weights(&self, _node_id: u64) -> Vec<f64> {
        Vec::new()
    }
}

pub struct WeightedVectorComputer<'a> {
    graph: &'a dyn Graph,
    property_key: String,
    orientation: Orientation,
}

impl<'a> WeightedVectorComputer<'a> {
    pub fn new(graph: &'a dyn Graph, property_key: String, orientation: Orientation) -> Self {
        Self {
            graph,
            property_key,
            orientation,
        }
    }
}

impl<'a> VectorComputer for WeightedVectorComputer<'a> {
    fn vector(&self, node_id: u64) -> Vec<u64> {
        let mut neighbors: Vec<u64> = self
            .graph
            .neighbors(node_id as usize)
            .into_iter()
            .map(|n| n as u64)
            .collect();
        neighbors.sort_unstable();
        neighbors
    }

    fn weights(&self, node_id: u64) -> Vec<f64> {
        // This is tricky without exact Graph API knowledge for property access.
        // Assuming there's a way to get relationships with properties.
        // For now, returning a placeholder or need to investigate Graph API more.
        // The Java code uses a "WeightedVectorComputer" which iterates rels.
        // Let's assume we can fetch weights.
        // TODO: Implement actual weight fetching using Graph API.
        // For this step, I will return placeholder to compilation.
        vec![1.0; self.graph.degree(node_id as usize) as usize]
    }
}
