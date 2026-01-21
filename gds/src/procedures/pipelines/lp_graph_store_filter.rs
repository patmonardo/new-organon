use std::collections::HashSet;

use crate::projection::{NodeLabel, RelationshipType};

#[derive(Debug, Clone)]
pub struct LPGraphStoreFilter {
    source_node_labels: Vec<NodeLabel>,
    target_node_labels: Vec<NodeLabel>,
    predict_relationship_types: Vec<RelationshipType>,
    node_property_steps_base_labels: Vec<NodeLabel>,
}

impl LPGraphStoreFilter {
    pub fn new(
        source_node_labels: Vec<NodeLabel>,
        target_node_labels: Vec<NodeLabel>,
        predict_relationship_types: Vec<RelationshipType>,
        node_property_steps_base_labels: Vec<NodeLabel>,
    ) -> Self {
        Self {
            source_node_labels,
            target_node_labels,
            predict_relationship_types,
            node_property_steps_base_labels,
        }
    }

    pub fn source_node_labels(&self) -> &[NodeLabel] {
        &self.source_node_labels
    }

    pub fn target_node_labels(&self) -> &[NodeLabel] {
        &self.target_node_labels
    }

    pub fn predict_relationship_types(&self) -> &[RelationshipType] {
        &self.predict_relationship_types
    }

    pub fn node_property_steps_base_labels(&self) -> &[NodeLabel] {
        &self.node_property_steps_base_labels
    }

    pub fn predict_node_labels(&self) -> HashSet<NodeLabel> {
        self.source_node_labels
            .iter()
            .chain(self.target_node_labels.iter())
            .cloned()
            .collect()
    }
}
