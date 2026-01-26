use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;

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
}

impl UnweightedVectorComputer<'_> {
    pub fn new<'a>(
        graph: &'a dyn Graph,
        _relationship_type: Option<RelationshipType>,
        _orientation: Orientation,
    ) -> UnweightedVectorComputer<'a> {
        UnweightedVectorComputer { graph }
    }
}

impl<'a> VectorComputer for UnweightedVectorComputer<'a> {
    fn vector(&self, node_id: u64) -> Vec<u64> {
        let node_id_mapped = node_id as MappedNodeId;
        self.graph
            .stream_relationships(node_id_mapped, self.graph.default_property_value())
            .map(|cursor| cursor.target_id() as u64)
            .collect()
    }

    fn weights(&self, node_id: u64) -> Vec<f64> {
        let node_id_mapped = node_id as MappedNodeId;
        // Optimization: use degree to pre-allocate?
        let degree = self.graph.degree(node_id_mapped);
        vec![1.0; degree]
    }
}

pub struct WeightedVectorComputer<'a> {
    graph: &'a dyn Graph,
}

impl<'a> WeightedVectorComputer<'a> {
    pub fn new(graph: &'a dyn Graph, _weight_property: String, _orientation: Orientation) -> Self {
        Self { graph }
    }
}

impl<'a> VectorComputer for WeightedVectorComputer<'a> {
    fn vector(&self, node_id: u64) -> Vec<u64> {
        let node_id_mapped = node_id as MappedNodeId;
        self.graph
            .stream_relationships(node_id_mapped, self.graph.default_property_value())
            .map(|cursor| cursor.target_id() as u64)
            .collect()
    }

    fn weights(&self, node_id: u64) -> Vec<f64> {
        let node_id_mapped = node_id as MappedNodeId;
        let degree = self.graph.degree(node_id_mapped);
        vec![1.0; degree]
    }
}
