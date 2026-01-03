use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Drops a graph-level property from a graph store.
/// Mirrors Java `dropGraphProperty` behavior.
pub struct DropGraphPropertyApplication;

impl DropGraphPropertyApplication {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        graph_property: &str,
        fail_if_missing: bool,
    ) -> Result<(DefaultGraphStore, u64), String> {
        let mut modified = graph_store.clone();

        if !modified.has_graph_property(graph_property) {
            if fail_if_missing {
                return Err(format!("Graph property '{graph_property}' does not exist"));
            }
            return Ok((modified, 0));
        }

        modified
            .remove_graph_property(graph_property)
            .map_err(|e| e.to_string())?;

        Ok((modified, 1))
    }
}

impl Default for DropGraphPropertyApplication {
    fn default() -> Self {
        Self::new()
    }
}
