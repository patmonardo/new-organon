use std::collections::{HashMap, HashSet};

use crate::projection::RelationshipType;
use crate::substrate::FormStoreSurface;
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use crate::types::graph_store::{GraphName, GraphStoreError, GraphStoreResult};

/// Minimal in-memory substrate store intended for "PureForm".
///
/// This is not a replacement for `DefaultGraphStore`. It's a small substrate-native
/// store that implements exactly the `FormStoreSurface` requirements so the Form ISA
/// can exist independently of the legacy graph store.
#[derive(Debug, Clone, Default)]
pub struct InMemoryFormStore {
    original_by_mapped: Vec<OriginalNodeId>,
    mapped_by_original: HashMap<OriginalNodeId, MappedNodeId>,

    edges_by_type: HashMap<RelationshipType, Vec<(MappedNodeId, MappedNodeId)>>,
    inverse_indexed: HashSet<RelationshipType>,

    node_props_i64: HashMap<String, Vec<i64>>,
    node_props_f64: HashMap<String, Vec<f64>>,

    rel_props_f64: HashMap<(RelationshipType, String), Vec<f64>>,

    _graph_name: Option<GraphName>,
}

impl InMemoryFormStore {
    pub fn new(original_node_ids: Vec<OriginalNodeId>) -> GraphStoreResult<Self> {
        if original_node_ids.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "node set must be non-empty".to_string(),
            ));
        }

        let mut mapped_by_original = HashMap::with_capacity(original_node_ids.len());
        for (mapped, original) in original_node_ids.iter().copied().enumerate() {
            if mapped_by_original
                .insert(original, mapped as MappedNodeId)
                .is_some()
            {
                return Err(GraphStoreError::InvalidOperation(
                    "original node ids must be unique".to_string(),
                ));
            }
        }

        Ok(Self {
            original_by_mapped: original_node_ids,
            mapped_by_original,
            ..Default::default()
        })
    }

    pub fn with_graph_name(mut self, name: GraphName) -> Self {
        self._graph_name = Some(name);
        self
    }

    pub fn with_inverse_indexed(
        mut self,
        rel_types: impl IntoIterator<Item = RelationshipType>,
    ) -> Self {
        self.inverse_indexed = rel_types.into_iter().collect();
        self
    }

    /// Add directed edges for a relationship type, expressed in original node ids.
    pub fn add_relationships_by_original(
        &mut self,
        rel_type: RelationshipType,
        edges: impl IntoIterator<Item = (OriginalNodeId, OriginalNodeId)>,
    ) -> GraphStoreResult<()> {
        let mut out: Vec<(MappedNodeId, MappedNodeId)> = Vec::new();
        for (src_orig, dst_orig) in edges {
            let src = *self.mapped_by_original.get(&src_orig).ok_or_else(|| {
                GraphStoreError::InvalidOperation("edge source original id not found".to_string())
            })?;
            let dst = *self.mapped_by_original.get(&dst_orig).ok_or_else(|| {
                GraphStoreError::InvalidOperation("edge target original id not found".to_string())
            })?;
            out.push((src, dst));
        }

        self.edges_by_type.entry(rel_type).or_default().extend(out);
        Ok(())
    }

    pub fn node_property_f64(&self, key: &str) -> Option<&[f64]> {
        self.node_props_f64.get(key).map(|v| v.as_slice())
    }

    pub fn node_property_i64(&self, key: &str) -> Option<&[i64]> {
        self.node_props_i64.get(key).map(|v| v.as_slice())
    }

    pub fn edges_by_type(&self) -> &HashMap<RelationshipType, Vec<(MappedNodeId, MappedNodeId)>> {
        &self.edges_by_type
    }

    pub fn relationship_property_f64(
        &self,
        rel_type: &RelationshipType,
        key: &str,
    ) -> Option<&[f64]> {
        self.rel_props_f64
            .get(&(rel_type.clone(), key.to_string()))
            .map(|v| v.as_slice())
    }
}

impl FormStoreSurface for InMemoryFormStore {
    type Store = InMemoryFormStore;

    fn commit_induced_subgraph_by_original_node_ids(
        &self,
        graph_name: GraphName,
        selected_original_node_ids: &[OriginalNodeId],
    ) -> GraphStoreResult<(
        Self::Store,
        HashMap<MappedNodeId, MappedNodeId>,
        HashMap<RelationshipType, usize>,
    )> {
        if selected_original_node_ids.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "selection must be non-empty".to_string(),
            ));
        }

        let mut seen: HashSet<OriginalNodeId> =
            HashSet::with_capacity(selected_original_node_ids.len());
        let mut new_original_by_mapped: Vec<OriginalNodeId> =
            Vec::with_capacity(selected_original_node_ids.len());
        let mut old_mapped_to_new: HashMap<MappedNodeId, MappedNodeId> =
            HashMap::with_capacity(selected_original_node_ids.len());

        for (new_mapped, original) in selected_original_node_ids.iter().copied().enumerate() {
            if !seen.insert(original) {
                return Err(GraphStoreError::InvalidOperation(
                    "selection must not contain duplicates".to_string(),
                ));
            }

            let old_mapped = *self.mapped_by_original.get(&original).ok_or_else(|| {
                GraphStoreError::InvalidOperation(
                    "selection contains unknown original node id".to_string(),
                )
            })?;

            let new_mapped_id = new_mapped as MappedNodeId;
            new_original_by_mapped.push(original);
            old_mapped_to_new.insert(old_mapped, new_mapped_id);
        }

        let mut committed =
            InMemoryFormStore::new(new_original_by_mapped)?.with_graph_name(graph_name);
        committed.inverse_indexed = self.inverse_indexed.clone();

        let mut kept_by_type: HashMap<RelationshipType, usize> = HashMap::new();

        for (rel_type, edges) in self.edges_by_type.iter() {
            let mut kept: usize = 0;
            let mut new_edges: Vec<(MappedNodeId, MappedNodeId)> = Vec::new();

            for (src_old, dst_old) in edges {
                let Some(src_new) = old_mapped_to_new.get(src_old).copied() else {
                    continue;
                };
                let Some(dst_new) = old_mapped_to_new.get(dst_old).copied() else {
                    continue;
                };

                kept += 1;
                new_edges.push((src_new, dst_new));
            }

            if kept > 0 {
                kept_by_type.insert(rel_type.clone(), kept);
                committed.edges_by_type.insert(rel_type.clone(), new_edges);
            }
        }

        Ok((committed, old_mapped_to_new, kept_by_type))
    }

    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId> {
        if mapped_node_id < 0 {
            return None;
        }
        self.original_by_mapped
            .get(mapped_node_id as usize)
            .copied()
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.inverse_indexed.clone()
    }

    fn add_node_property_i64(&mut self, key: String, values: Vec<i64>) -> GraphStoreResult<()> {
        if values.len() != self.node_count() {
            return Err(GraphStoreError::InvalidOperation(
                "property length must equal node_count".to_string(),
            ));
        }
        self.node_props_i64.insert(key, values);
        Ok(())
    }

    fn add_node_property_f64(&mut self, key: String, values: Vec<f64>) -> GraphStoreResult<()> {
        if values.len() != self.node_count() {
            return Err(GraphStoreError::InvalidOperation(
                "property length must equal node_count".to_string(),
            ));
        }
        self.node_props_f64.insert(key, values);
        Ok(())
    }

    fn ensure_node_property_discoverable(&mut self, _key: &str) -> GraphStoreResult<()> {
        Ok(())
    }

    fn node_count(&self) -> usize {
        self.original_by_mapped.len()
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.edges_by_type
            .get(relationship_type)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    fn relationship_edges_in_store_order(
        &self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<Vec<(MappedNodeId, MappedNodeId)>> {
        Ok(self
            .edges_by_type
            .get(relationship_type)
            .cloned()
            .unwrap_or_default())
    }

    fn add_relationship_property_f64(
        &mut self,
        relationship_type: RelationshipType,
        key: String,
        values: Vec<f64>,
    ) -> GraphStoreResult<()> {
        let element_count = self.relationship_count_for_type(&relationship_type);
        if values.len() != element_count {
            return Err(GraphStoreError::InvalidOperation(
                "relationship property length must equal relationship_count_for_type".to_string(),
            ));
        }

        self.rel_props_f64.insert((relationship_type, key), values);
        Ok(())
    }
}
