use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph_store::GraphStore;

/// Validation helpers mirroring Java's GraphStoreValidationService.
pub struct GraphStoreValidationService;

impl GraphStoreValidationService {
    pub fn new() -> Self {
        Self
    }

    pub fn ensure_node_properties_exist<G: GraphStore>(
        &self,
        graph_store: &G,
        node_properties: &[String],
    ) -> Result<(), String> {
        for property in node_properties {
            if !graph_store.has_node_property(property) {
                return Err(format!("Unknown node property '{property}'"));
            }
        }
        Ok(())
    }

    pub fn filter_existing_node_properties<G: GraphStore>(
        &self,
        graph_store: &G,
        node_properties: &[String],
    ) -> Vec<String> {
        node_properties
            .iter()
            .filter(|prop| graph_store.has_node_property(prop))
            .cloned()
            .collect()
    }

    pub fn ensure_relationships_may_be_deleted<G: GraphStore>(
        &self,
        graph_store: &G,
        relationship_type: &str,
        graph_name: &str,
    ) -> Result<(), String> {
        let rel_type = RelationshipType::of(relationship_type);
        if !graph_store.has_relationship_type(&rel_type) {
            return Err(format!(
                "Relationship type '{relationship_type}' does not exist in graph '{graph_name}'"
            ));
        }
        Ok(())
    }

    pub fn ensure_graph_property_exists<G: GraphStore>(
        &self,
        graph_store: &G,
        graph_property: &str,
    ) -> Result<(), String> {
        if !graph_store.has_graph_property(graph_property) {
            return Err(format!("Graph property '{graph_property}' does not exist"));
        }
        Ok(())
    }

    pub fn ensure_node_properties_match_node_labels<G: GraphStore>(
        &self,
        graph_store: &G,
        node_labels: &[String],
        node_properties: &[String],
    ) -> Result<(), String> {
        for label in node_labels {
            let node_label = NodeLabel::of(label);
            if !graph_store.has_node_label(&node_label) {
                return Err(format!("Unknown node label '{label}'"));
            }

            for property in node_properties {
                if !graph_store.has_node_property_for_label(&node_label, property) {
                    return Err(format!(
                        "Node property '{property}' does not exist for label '{label}'"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn ensure_relationship_properties_match_relationship_types<G: GraphStore>(
        &self,
        graph_store: &G,
        relationship_types: &[String],
        relationship_properties: &[String],
    ) -> Result<(), String> {
        for rel_type in relationship_types {
            let rel = RelationshipType::of(rel_type);
            if !graph_store.has_relationship_type(&rel) {
                return Err(format!("Unknown relationship type '{rel_type}'"));
            }
            for property in relationship_properties {
                if !graph_store.has_relationship_property(&rel, property) {
                    return Err(format!(
                        "Relationship property '{property}' does not exist for type '{rel_type}'"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn ensure_relationship_properties_match_relationship_type<G: GraphStore>(
        &self,
        graph_store: &G,
        relationship_type: &str,
        relationship_properties: &[String],
    ) -> Result<(), String> {
        let rel = RelationshipType::of(relationship_type);
        if !graph_store.has_relationship_type(&rel) {
            return Err(format!("Unknown relationship type '{relationship_type}'"));
        }

        for property in relationship_properties {
            if !graph_store.has_relationship_property(&rel, property) {
                return Err(format!(
                    "Relationship property '{property}' does not exist for type '{relationship_type}'"
                ));
            }
        }
        Ok(())
    }

    pub fn ensure_possible_relationship_property_matches_relationship_type<G: GraphStore>(
        &self,
        graph_store: &G,
        relationship_type: &str,
        possible_property: Option<&str>,
    ) -> Result<(), String> {
        if let Some(property) = possible_property {
            self.ensure_relationship_properties_match_relationship_type(
                graph_store,
                relationship_type,
                &[property.to_string()],
            )?
        }
        Ok(())
    }

    pub fn ensure_relationship_types_present<G: GraphStore>(
        &self,
        graph_store: &G,
        relationship_types: &[String],
    ) -> Result<(), String> {
        for rel_type in relationship_types {
            let rel = RelationshipType::of(rel_type);
            if !graph_store.has_relationship_type(&rel) {
                return Err(format!("Unknown relationship type '{rel_type}'"));
            }
        }
        Ok(())
    }

    pub fn ensure_read_access<G: GraphStore>(
        &self,
        _graph_store: &G,
        _should_export_additional_node_properties: bool,
    ) -> Result<(), String> {
        // Placeholder for permission checks; keep signature for compatibility.
        Ok(())
    }

    pub fn ensure_node_properties_not_exist<G: GraphStore>(
        &self,
        graph_store: &G,
        additional_properties: &[String],
    ) -> Result<(), String> {
        for property in additional_properties {
            if graph_store.has_node_property(property) {
                return Err(format!("Node property '{property}' already exists"));
            }
        }
        Ok(())
    }

    // Helper methods for checking graph store state
    #[allow(dead_code)]
    fn has_node_property<G: GraphStore>(&self, _graph_store: &G, _property: &str) -> bool {
        _graph_store.has_node_property(_property)
    }

    #[allow(dead_code)]
    fn has_relationship_type<G: GraphStore>(&self, _graph_store: &G, _rel_type: &str) -> bool {
        _graph_store.has_relationship_type(&RelationshipType::of(_rel_type))
    }

    #[allow(dead_code)]
    fn has_graph_property<G: GraphStore>(&self, _graph_store: &G, _property: &str) -> bool {
        _graph_store.has_graph_property(_property)
    }

    #[allow(dead_code)]
    fn has_node_property_for_label<G: GraphStore>(
        &self,
        _graph_store: &G,
        _label: &str,
        _property: &str,
    ) -> bool {
        _graph_store.has_node_property_for_label(&NodeLabel::of(_label), _property)
    }

    #[allow(dead_code)]
    fn has_relationship_property_for_type<G: GraphStore>(
        &self,
        _graph_store: &G,
        _rel_type: &str,
        _property: &str,
    ) -> bool {
        _graph_store.has_relationship_property(&RelationshipType::of(_rel_type), _property)
    }
}

impl Default for GraphStoreValidationService {
    fn default() -> Self {
        Self::new()
    }
}
