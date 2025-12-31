use std::collections::HashSet;
use std::sync::Arc;

use crate::projection::NodeLabel;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use crate::types::graph_store::GraphStore;
use crate::types::graph_store::{GraphName, GraphStoreError, GraphStoreResult, InducedSubgraphResult};

/// Minimal surface that Form needs from a graph store.
///
/// This is a *boundary interface*: it lets us grow a new substrate-backed store without forcing
/// the evaluator stack to depend on the legacy `core/` translation layer.
///
/// Notes:
/// - This is intentionally small (B + A): induced-subgraph commit + numeric node property writes.
/// - This trait is not currently used for dynamic dispatch; it exists to keep capability shapes
///   stable as we evolve the substrate.
pub trait FormStoreSurface {
    /// Concrete store type produced by commitment.
    type Store;

    /// (B) Commit an induced subgraph into a new store.
    ///
    /// Returns:
    /// - the new store (topology-only)
    /// - `old_mapped_id -> new_mapped_id`
    /// - per-relationship-type kept counts
    fn commit_induced_subgraph_by_original_node_ids(
        &self,
        graph_name: GraphName,
        selected_original_node_ids: &[OriginalNodeId],
    ) -> GraphStoreResult<InducedSubgraphResult<Self::Store>>;

    /// Map an internal contiguous (mapped) node id back to its original id.
    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId>;

    /// Relationship types which are inverse-indexed.
    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType>;

    /// (A) Write an i64 node property onto the store.
    fn add_node_property_i64(&mut self, key: String, values: Vec<i64>) -> GraphStoreResult<()>;

    /// (A) Write an f64 node property onto the store.
    fn add_node_property_f64(&mut self, key: String, values: Vec<f64>) -> GraphStoreResult<()>;

    /// Ensure a newly-written node property is discoverable via schema/label indexes.
    ///
    /// For `DefaultGraphStore`, `add_node_property_{i64,f64}` is config-based and can bypass
    /// label-scoped discovery. This hook lets the store implementation patch up any indexes.
    fn ensure_node_property_discoverable(&mut self, key: &str) -> GraphStoreResult<()>;

    /// Convenience: node count for shape checks.
    fn node_count(&self) -> usize;

    // =============================================================================
    // Relationship Properties (numeric-only for now)
    // =============================================================================

    /// Relationship count for a specific type.
    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize;

    /// Relationship endpoints in the store's property index order.
    ///
    /// This order matches the flat CSR ordering used by relationship property values.
    fn relationship_edges_in_store_order(
        &self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<Vec<(MappedNodeId, MappedNodeId)>>;

    /// (A) Write an f64 relationship property onto the store for a specific relationship type.
    fn add_relationship_property_f64(
        &mut self,
        relationship_type: RelationshipType,
        key: String,
        values: Vec<f64>,
    ) -> GraphStoreResult<()>;
}

impl FormStoreSurface for crate::types::graph_store::DefaultGraphStore {
    type Store = crate::types::graph_store::DefaultGraphStore;

    fn commit_induced_subgraph_by_original_node_ids(
        &self,
        graph_name: GraphName,
        selected_original_node_ids: &[OriginalNodeId],
    ) -> GraphStoreResult<InducedSubgraphResult<Self::Store>> {
        self.commit_induced_subgraph_by_original_node_ids(graph_name, selected_original_node_ids)
    }

    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId> {
        self.nodes().to_original_node_id(mapped_node_id)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        crate::types::graph_store::GraphStore::inverse_indexed_relationship_types(self)
    }

    fn add_node_property_i64(&mut self, key: String, values: Vec<i64>) -> GraphStoreResult<()> {
        self.add_node_property_i64(key, values)
    }

    fn add_node_property_f64(&mut self, key: String, values: Vec<f64>) -> GraphStoreResult<()> {
        self.add_node_property_f64(key, values)
    }

    fn ensure_node_property_discoverable(&mut self, key: &str) -> GraphStoreResult<()> {
        let label_set: HashSet<NodeLabel> = self
            .nodes()
            .available_node_labels()
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect();

        if label_set.is_empty() {
            return Ok(());
        }

        let values = self.node_property_values(key)?;
        self.add_node_property(label_set, key.to_string(), values)
    }

    fn node_count(&self) -> usize {
        crate::types::graph_store::GraphStore::node_count(self)
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        crate::types::graph_store::GraphStore::relationship_count_for_type(self, relationship_type)
    }

    fn relationship_edges_in_store_order(
        &self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<Vec<(MappedNodeId, MappedNodeId)>> {
        let mut types = HashSet::new();
        types.insert(relationship_type.clone());

        let graph = self
            .get_graph_with_types(&types)
            .map_err(|e| GraphStoreError::InvalidOperation(e.to_string()))?;

        let mut out: Vec<(MappedNodeId, MappedNodeId)> =
            Vec::with_capacity(graph.relationship_count());
        for source in 0..graph.node_count() {
            for cursor in
                graph.stream_relationships(source as MappedNodeId, graph.default_property_value())
            {
                out.push((cursor.source_id(), cursor.target_id()));
            }
        }
        Ok(out)
    }

    fn add_relationship_property_f64(
        &mut self,
        relationship_type: RelationshipType,
        key: String,
        values: Vec<f64>,
    ) -> GraphStoreResult<()> {
        use crate::types::properties::relationship::impls::default_relationship_property_values::DefaultRelationshipPropertyValues;

        let element_count = crate::types::graph_store::GraphStore::relationship_count_for_type(
            self,
            &relationship_type,
        );
        if values.len() != element_count {
            return Err(GraphStoreError::InvalidOperation(format!(
                "relationship property length ({}) must equal relationship_count_for_type ({element_count})",
                values.len()
            )));
        }

        let property_values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            values,
            element_count,
        ));
        self.add_relationship_property(relationship_type, key, property_values)
    }
}
