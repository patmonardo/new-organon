use super::{
    Capabilities, DatabaseInfo, DeletionResult, GraphName, GraphStore, GraphStoreError,
    GraphStoreResult, InducedSubgraphResult, ProjectedPropertiesResult,
};
use crate::collections::backends::arrow::{ArrowDoubleArray, ArrowLongArray};
use crate::collections::backends::factory::{
    create_double_backend_from_config, create_float_backend_from_config,
    create_int_backend_from_config, create_long_backend_from_config, DoubleCollection,
    LongCollection,
};
use crate::collections::backends::vec::{
    VecDouble, VecDoubleArray, VecFloat, VecInt, VecLong, VecLongArray,
};
use crate::config::GraphStoreConfig;
use crate::projection::Orientation;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use crate::types::graph::GraphResult;
use crate::types::graph::{
    id_map::{IdMap, SimpleIdMap},
    DefaultGraph, Graph, GraphCharacteristics, GraphCharacteristicsBuilder, RelationshipTopology,
};
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::graph::{
    DefaultDoubleGraphPropertyValues, DefaultLongGraphPropertyValues,
};
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
};
use crate::types::properties::node::{
    DefaultDoubleNodePropertyValues, DefaultFloatNodePropertyValues, DefaultIntNodePropertyValues,
    DefaultLongNodePropertyValues,
};
use crate::types::properties::relationship::default_relationship_property_store::DefaultRelationshipPropertyStore;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::relationship::{
    DefaultDoubleRelationshipPropertyValues, DefaultIntRelationshipPropertyValues,
    DefaultLongRelationshipPropertyValues,
};
use crate::types::properties::relationship::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
use crate::types::schema::{
    Direction, GraphSchema, MutableGraphSchema, PropertySchemaTrait, RelationshipSchema,
    RelationshipSchemaEntry,
};
use crate::types::PropertyState;
use crate::types::ValueType;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::algo::algorithms::scaling::{MinMaxScaler, Scaler};

/// In-memory [`GraphStore`] backed by [`SimpleIdMap`] and [`RelationshipTopology`].
#[derive(Debug, Clone)]
pub struct DefaultGraphStore {
    config: Arc<GraphStoreConfig>,
    graph_name: GraphName,
    database_info: DatabaseInfo,
    schema: Arc<GraphSchema>,
    capabilities: Capabilities,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    id_map: Arc<SimpleIdMap>,
    relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_relationship_types: Vec<RelationshipType>,
    inverse_indexed_relationship_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_relationships: bool,
    graph_characteristics: GraphCharacteristics,
    graph_properties: HashMap<String, Arc<dyn GraphPropertyValues>>,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    node_properties_by_label: HashMap<String, HashSet<String>>,
    relationship_property_stores: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    has_relationship_properties: bool,
}

impl DefaultGraphStore {
    /// Creates a new store from the provided components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: GraphStoreConfig,
        graph_name: GraphName,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        id_map: SimpleIdMap,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> Self {
        let now = Utc::now();
        let config = Arc::new(config);
        let schema = Arc::new(schema);
        let id_map = Arc::new(id_map);
        let relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(rel_type, topology)| (rel_type, Arc::new(topology)))
            .collect();

        let mut store = Self {
            config: Arc::clone(&config),
            graph_name,
            database_info,
            schema,
            capabilities,
            creation_time: now,
            modification_time: now,
            id_map,
            relationship_topologies,
            ordered_relationship_types: Vec::new(),
            inverse_indexed_relationship_types: HashSet::new(),
            relationship_count: 0,
            has_parallel_relationships: false,
            graph_characteristics: GraphCharacteristicsBuilder::new().build(),
            graph_properties: HashMap::new(),
            node_properties: HashMap::new(),
            node_properties_by_label: HashMap::new(),
            relationship_property_stores: HashMap::new(),
            has_relationship_properties: false,
        };

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        store
    }

    /// Builds a [`DefaultGraph`] view over the current store contents.
    /// Returns the concrete DefaultGraph type for backwards compatibility.
    pub fn graph(&self) -> Arc<DefaultGraph> {
        // Create DefaultGraph directly for backwards compatibility
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            self.graph_characteristics,
            topologies,
            self.ordered_relationship_types.clone(),
            self.inverse_indexed_relationship_types.clone(),
            self.relationship_count,
            self.has_parallel_relationships,
            self.node_properties.clone(),
            self.relationship_property_stores.clone(),
            HashMap::new(),
        ))
    }

    /// Creates an undirected version of this graph store.
    ///
    /// Semantics:
    /// - For each relationship type, creates symmetric adjacency: if (u→v) exists, then (v→u) exists.
    /// - Neighbor sets are deduplicated per source node.
    /// - Relationship properties are not carried over yet.
    pub fn to_undirected(&self, graph_name: GraphName) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
            HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

            for source in 0..node_count {
                let source_id = source as MappedNodeId;
                let Some(neighbors) = topology.outgoing(source_id) else {
                    continue;
                };

                for &target_id in neighbors.iter() {
                    let target = target_id as usize;
                    if target >= node_count {
                        continue;
                    }
                    outgoing[source].push(target_id);
                    outgoing[target].push(source_id);
                }
            }

            for adj in outgoing.iter_mut() {
                adj.sort_unstable();
                adj.dedup();
            }

            new_relationship_topologies
                .insert(rel_type.clone(), RelationshipTopology::new(outgoing, None));
        }

        // Update schema: mark all relationship types as undirected, preserving properties.
        let old_schema = self.schema.as_ref();
        let mut rel_entries: HashMap<RelationshipType, RelationshipSchemaEntry> = HashMap::new();
        for entry in old_schema.relationship_schema().entries() {
            rel_entries.insert(
                entry.identifier().clone(),
                RelationshipSchemaEntry::new(
                    entry.identifier().clone(),
                    Direction::Undirected,
                    entry.properties().clone(),
                ),
            );
        }
        let relationship_schema = RelationshipSchema::new(rel_entries);
        let schema = GraphSchema::new(
            old_schema.node_schema().clone(),
            relationship_schema,
            old_schema.graph_properties().clone(),
        );

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.schema = Arc::new(schema);
        store.relationship_topologies = new_relationship_topologies
            .into_iter()
            .map(|(t, topo)| (t, Arc::new(topo)))
            .collect();
        store.relationship_property_stores.clear();
        store.has_relationship_properties = false;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();

        Ok(store)
    }

    /// Creates a version of this store where all relationship types have inverse indices.
    ///
    /// This enables `graph.stream_inverse_relationships(...)` and `degree_inverse(...)`.
    pub fn with_inverse_indices(
        &self,
        graph_name: GraphName,
    ) -> GraphStoreResult<DefaultGraphStore> {
        self.with_inverse_indices_filtered(graph_name, None)
    }

    /// Creates a version of this store where a subset of relationship types have inverse indices.
    ///
    /// When `relationship_types` is `None`, all types are indexed (same as `with_inverse_indices`).
    /// When provided, only those types are re-built with incoming adjacency; other types are kept
    /// unchanged.
    pub fn with_inverse_indices_filtered(
        &self,
        graph_name: GraphName,
        relationship_types: Option<&HashSet<RelationshipType>>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();
        let selected = relationship_types.cloned();

        for (rel_type, topology) in &self.relationship_topologies {
            let should_index = selected
                .as_ref()
                .map(|set| set.contains(rel_type))
                .unwrap_or(true);

            if !should_index {
                new_relationship_topologies.insert(rel_type.clone(), Arc::clone(topology));
                continue;
            }

            let outgoing = topology.outgoing_lists().to_vec();
            let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

            for source in 0..node_count {
                let source_id = source as MappedNodeId;
                let Some(neighbors) = topology.outgoing(source_id) else {
                    continue;
                };
                for &target_id in neighbors.iter() {
                    let target = target_id as usize;
                    if target >= node_count {
                        continue;
                    }
                    incoming[target].push(source_id);
                }
            }

            for adj in incoming.iter_mut() {
                adj.sort_unstable();
                adj.dedup();
            }

            new_relationship_topologies.insert(
                rel_type.clone(),
                Arc::new(RelationshipTopology::new(outgoing, Some(incoming))),
            );
        }

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = new_relationship_topologies;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    pub(crate) fn with_added_relationship_type(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();
        if outgoing.len() != node_count {
            return Err(GraphStoreError::InvalidOperation(format!(
                "outgoing adjacency length {} does not match node_count {node_count}",
                outgoing.len()
            )));
        }

        let mut relationship_topologies = self.relationship_topologies.clone();
        relationship_topologies.insert(
            rel_type.clone(),
            Arc::new(RelationshipTopology::new(outgoing, None)),
        );

        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), direction);
        let schema = Arc::new(schema.build());

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.schema = schema;
        store.relationship_topologies = relationship_topologies;
        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    pub(crate) fn with_added_relationship_type_preserve_name(
        &self,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<DefaultGraphStore> {
        self.with_added_relationship_type(self.graph_name.clone(), rel_type, outgoing, direction)
    }

    pub(crate) fn with_rebuilt_relationship_topologies(
        &self,
        graph_name: GraphName,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();
        for topology in relationship_topologies.values() {
            if topology.node_capacity() != node_count {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "relationship topology capacity {} does not match node_count {node_count}",
                    topology.node_capacity()
                )));
            }
        }

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(t, topo)| (t, Arc::new(topo)))
            .collect();
        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    /// Creates a new store with a scaled numeric node property added.
    ///
    /// Currently uses MinMax scaling and writes the result as a Double property.
    pub fn with_scaled_node_property_minmax(
        &self,
        graph_name: GraphName,
        source_property: &str,
        target_property: &str,
        concurrency: usize,
    ) -> GraphStoreResult<DefaultGraphStore> {
        if source_property.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "source_property must be non-empty".to_string(),
            ));
        }
        if target_property.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "target_property must be non-empty".to_string(),
            ));
        }
        if concurrency == 0 {
            return Err(GraphStoreError::InvalidOperation(
                "concurrency must be > 0".to_string(),
            ));
        }

        let pv = self.node_property_values(source_property)?;
        let node_count = self.node_count() as u64;

        let property_fn: Box<dyn Fn(u64) -> f64 + Send + Sync> = match pv.value_type() {
            ValueType::Long => {
                Box::new(move |node_id: u64| pv.long_value(node_id).unwrap_or(0) as f64)
            }
            ValueType::Double => {
                Box::new(move |node_id: u64| pv.double_value(node_id).unwrap_or(0.0))
            }
            other => {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "scaleProperties expects Long/Double node property (got {other:?})"
                )))
            }
        };

        let scaler: Box<dyn Scaler> = MinMaxScaler::create(node_count, &property_fn, concurrency);
        let mut scaled: Vec<f64> = Vec::with_capacity(node_count as usize);
        for node_id in 0..node_count {
            scaled.push(scaler.scale_property(node_id, property_fn.as_ref()));
        }

        let pv_out: Arc<dyn NodePropertyValues> = Arc::new(DefaultDoubleNodePropertyValues::<
            VecDouble,
        >::from_collection(
            VecDouble::from(scaled),
            node_count as usize,
        ));

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.add_node_property(
            HashSet::from([NodeLabel::all_nodes()]),
            target_property.to_string(),
            pv_out,
        )?;

        Ok(store)
    }

    /// Collapses linear paths by removing degree-2 intermediate nodes.
    ///
    /// Semantics (directed, per relationship type):
    /// - Detect chains where intermediate nodes have exactly 1 incoming and 1 outgoing edge.
    /// - Replace each chain `s -> ... -> t` with a single edge `s -> t`.
    /// - Does not modify the node set.
    /// - Leaves pure directed cycles unchanged (no natural chain start).
    /// - Relationship properties are not carried over.
    pub fn collapse_paths_degree2(
        &self,
        graph_name: GraphName,
        max_hops: Option<usize>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let outgoing_lists = topology.outgoing_lists();

            // Compute in/out degrees and unique successor (when out_degree==1).
            let mut out_degree: Vec<usize> = vec![0; node_count];
            let mut in_degree: Vec<usize> = vec![0; node_count];
            let mut succ: Vec<Option<MappedNodeId>> = vec![None; node_count];

            for (u, neighbors) in outgoing_lists.iter().enumerate() {
                out_degree[u] = neighbors.len();
                if neighbors.len() == 1 {
                    succ[u] = Some(neighbors[0]);
                }
                for &v in neighbors {
                    let v_usize = v as usize;
                    if v_usize < node_count {
                        in_degree[v_usize] += 1;
                    }
                }
            }

            // Mark nodes whose unique outgoing edge is part of a collapsed chain.
            let mut remove_unique_outgoing: Vec<bool> = vec![false; node_count];
            let mut collapsed_edges: Vec<(MappedNodeId, MappedNodeId)> = Vec::new();

            for s in 0..node_count {
                if out_degree[s] != 1 {
                    continue;
                }
                // Start nodes are where the chain can be entered from "outside".
                if in_degree[s] == 1 {
                    continue;
                }

                let mut next = match succ[s] {
                    Some(v) => v,
                    None => continue,
                };

                let mut hops = 1usize;
                remove_unique_outgoing[s] = true;

                loop {
                    if max_hops.is_some_and(|m| hops >= m) {
                        break;
                    }

                    let next_usize = next as usize;
                    if next_usize >= node_count {
                        break;
                    }

                    // Stop if next isn't a strict intermediate.
                    if in_degree[next_usize] != 1 || out_degree[next_usize] != 1 {
                        break;
                    }

                    // Advance through the intermediate.
                    remove_unique_outgoing[next_usize] = true;
                    next = match succ[next_usize] {
                        Some(v) => v,
                        None => break,
                    };
                    hops += 1;
                }

                // Create the collapsed edge from the original start to the terminal.
                collapsed_edges.push((s as MappedNodeId, next));
            }

            // Rebuild outgoing adjacency, skipping removed unique edges and adding collapsed edges.
            let mut new_outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
            for (u, neighbors) in outgoing_lists.iter().enumerate() {
                if out_degree[u] == 1 && remove_unique_outgoing[u] {
                    continue;
                }
                for &v in neighbors {
                    new_outgoing[u].push(v);
                }
            }

            for (s, t) in collapsed_edges {
                let s_usize = s as usize;
                if s_usize < node_count {
                    new_outgoing[s_usize].push(t);
                }
            }

            for adj in new_outgoing.iter_mut() {
                adj.sort_unstable();
                adj.dedup();
            }

            new_relationship_topologies.insert(
                rel_type.clone(),
                Arc::new(RelationshipTopology::new(new_outgoing, None)),
            );
        }

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = new_relationship_topologies;
        store.relationship_property_stores.clear();
        store.has_relationship_properties = false;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    /// Projects an induced subgraph into a new [`DefaultGraphStore`].
    ///
    /// - `selected_original_node_ids` are original (external) node ids.
    /// - The returned store reuses this store's config/schema/capabilities.
    /// - Properties are not copied; this is a topology-only projection.
    ///
    /// Returns the new store, an `old_mapped_id -> new_mapped_id` map, and per-type relationship counts kept.
    pub fn commit_induced_subgraph_by_original_node_ids(
        &self,
        graph_name: GraphName,
        selected_original_node_ids: &[OriginalNodeId],
    ) -> GraphStoreResult<InducedSubgraphResult<DefaultGraphStore>> {
        use std::collections::HashSet;

        if selected_original_node_ids.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "Selection must be non-empty".to_string(),
            ));
        }

        // Validate selection (no unknown ids, no duplicates), and build old->new mapping.
        let mut seen_original = HashSet::new();
        let mut selected_ordered_old_mapped: Vec<MappedNodeId> =
            Vec::with_capacity(selected_original_node_ids.len());
        let mut old_mapped_to_new: HashMap<MappedNodeId, MappedNodeId> = HashMap::new();

        for (index, &original_id) in selected_original_node_ids.iter().enumerate() {
            if !seen_original.insert(original_id) {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "Duplicate node id in selection: {original_id}"
                )));
            }
            let old_mapped = self
                .id_map
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| {
                    GraphStoreError::InvalidOperation(format!(
                        "Unknown node id in selection: {original_id}"
                    ))
                })?;
            let new_mapped: MappedNodeId = index as MappedNodeId;
            selected_ordered_old_mapped.push(old_mapped);
            old_mapped_to_new.insert(old_mapped, new_mapped);
        }

        let projected_properties = self.project_node_properties(&selected_ordered_old_mapped)?;
        let node_properties = projected_properties.node_properties;
        let node_properties_by_label = projected_properties.property_keys;

        // Build new IdMap, preserving labels.
        let mut new_id_map =
            SimpleIdMap::from_original_ids(selected_original_node_ids.iter().copied());
        for (new_mapped, &original_id) in selected_original_node_ids.iter().enumerate() {
            let new_mapped = new_mapped as MappedNodeId;
            let old_mapped = self
                .id_map
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| {
                    GraphStoreError::InvalidOperation(format!(
                        "Unknown node id in selection: {original_id}"
                    ))
                })?;
            for label in self.id_map.node_labels(old_mapped) {
                new_id_map.add_node_label(label.clone());
                new_id_map.add_node_id_to_label(new_mapped, label);
            }
        }

        // Induce relationship topologies by type.
        let n = selected_original_node_ids.len();
        let mut relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
            HashMap::new();
        let mut kept_by_type: HashMap<RelationshipType, usize> = HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); n];

            for (new_source_index, &old_source) in selected_ordered_old_mapped.iter().enumerate() {
                let neighbors = match topology.outgoing(old_source) {
                    Some(neighbors) => neighbors,
                    None => continue,
                };
                for &old_target in neighbors {
                    if let Some(&new_target) = old_mapped_to_new.get(&old_target) {
                        outgoing[new_source_index].push(new_target);
                    }
                }
            }

            let incoming = if topology.is_inverse_indexed() {
                let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); n];
                for (source, neighbors) in outgoing.iter().enumerate() {
                    let source_id = source as MappedNodeId;
                    for &target in neighbors {
                        let idx = target as usize;
                        if idx < incoming.len() {
                            incoming[idx].push(source_id);
                        }
                    }
                }
                Some(incoming)
            } else {
                None
            };

            let induced = RelationshipTopology::new(outgoing, incoming);
            let kept = induced.relationship_count();
            if kept > 0 {
                kept_by_type.insert(rel_type.clone(), kept);
                relationship_topologies.insert(rel_type.clone(), induced);
            }
        }

        let relationship_property_stores = self.project_relationship_properties(
            &selected_ordered_old_mapped,
            &old_mapped_to_new,
            &relationship_topologies,
        )?;

        let store = DefaultGraphStore::new(
            self.config.as_ref().clone(),
            graph_name,
            self.database_info.clone(),
            self.schema.as_ref().clone(),
            self.capabilities.clone(),
            new_id_map,
            relationship_topologies,
        );

        let mut store = store;
        store.node_properties = node_properties;
        store.node_properties_by_label = node_properties_by_label;
        store.graph_properties = self.graph_properties.clone();
        store.relationship_property_stores = relationship_property_stores;
        store.refresh_relationship_property_state();
        store.set_modified();

        Ok(InducedSubgraphResult {
            store,
            old_to_new_mapping: old_mapped_to_new,
            relationships_kept_by_type: kept_by_type,
        })
    }

    fn project_node_properties(
        &self,
        selected_ordered_old_mapped: &[MappedNodeId],
    ) -> GraphStoreResult<ProjectedPropertiesResult> {
        let node_count = selected_ordered_old_mapped.len();
        let mut projected: HashMap<String, Arc<dyn NodePropertyValues>> = HashMap::new();

        for (key, values) in &self.node_properties {
            match values.value_type() {
                ValueType::Double => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_value(old_id as u64)
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v);
                    }
                    let cfg = self.config.node_collections_config::<f64>(node_count);
                    let backend = create_double_backend_from_config(&cfg, data);
                    let pv = build_node_double_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                ValueType::Long => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_value(old_id as u64)
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v);
                    }
                    let cfg = self.config.node_collections_config::<i64>(node_count);
                    let backend = create_long_backend_from_config(&cfg, data);
                    let pv = build_node_long_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                ValueType::Float => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_value(old_id as u64)
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v as f32);
                    }
                    let cfg = self.config.node_collections_config::<f32>(node_count);
                    let backend = create_float_backend_from_config(&cfg, data);
                    let pv = build_node_float_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                ValueType::Int => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_value(old_id as u64)
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v as i32);
                    }
                    let cfg = self.config.node_collections_config::<i32>(node_count);
                    let backend = create_int_backend_from_config(&cfg, data);
                    let pv = build_node_int_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                ValueType::DoubleArray => {
                    let mut data: Vec<Option<Vec<f64>>> = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_array_value(old_id as u64)
                            .ok()
                            .map(Some)
                            .unwrap_or(None);
                        data.push(v);
                    }
                    let backend = VecDoubleArray::from(data);
                    let pv = build_node_double_array_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                ValueType::LongArray => {
                    let mut data: Vec<Option<Vec<i64>>> = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_array_value(old_id as u64)
                            .ok()
                            .map(Some)
                            .unwrap_or(None);
                        data.push(v);
                    }
                    let backend = VecLongArray::from(data);
                    let pv = build_node_long_array_property_values(backend, node_count);
                    projected.insert(key.clone(), pv);
                }
                _ => {
                    // Skip unsupported projection types for now.
                    continue;
                }
            }
        }

        let mut projected_by_label: HashMap<String, HashSet<String>> = HashMap::new();
        for (label_key, keys) in &self.node_properties_by_label {
            for key in keys {
                if projected.contains_key(key) {
                    projected_by_label
                        .entry(label_key.clone())
                        .or_default()
                        .insert(key.clone());
                }
            }
        }

        Ok(ProjectedPropertiesResult {
            node_properties: projected,
            property_keys: projected_by_label,
        })
    }

    fn project_relationship_properties(
        &self,
        selected_ordered_old_mapped: &[MappedNodeId],
        old_mapped_to_new: &HashMap<MappedNodeId, MappedNodeId>,
        new_relationship_topologies: &HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<HashMap<RelationshipType, DefaultRelationshipPropertyStore>> {
        let mut projected = HashMap::new();

        for (rel_type, new_topology) in new_relationship_topologies {
            let old_topology = match self.relationship_topologies.get(rel_type) {
                Some(t) => t,
                None => continue,
            };
            let old_offsets = relationship_prefix_offsets(old_topology);
            let new_count = new_topology.relationship_count();
            if new_count == 0 {
                continue;
            }

            let old_store = match self.relationship_property_stores.get(rel_type) {
                Some(store) if !store.is_empty() => store,
                _ => continue,
            };

            let mut builder = DefaultRelationshipPropertyStore::builder();

            for (key, property) in old_store.relationship_properties() {
                let values = property.values();
                match values.value_type() {
                    ValueType::Double => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index =
                                            old_offsets[old_source as usize] + neighbor_idx;
                                        let v = values.double_value(old_index as u64).map_err(
                                            |err| {
                                                GraphStoreError::InvalidOperation(format!("{err}"))
                                            },
                                        )?;
                                        data.push(v);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<f64>(data.len());
                        let backend = create_double_backend_from_config(&cfg, data);
                        let pv = build_relationship_double_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(key.clone(), projected_property);
                    }
                    ValueType::Long => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index =
                                            old_offsets[old_source as usize] + neighbor_idx;
                                        let v =
                                            values.long_value(old_index as u64).map_err(|err| {
                                                GraphStoreError::InvalidOperation(format!("{err}"))
                                            })?;
                                        data.push(v);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<i64>(data.len());
                        let backend = create_long_backend_from_config(&cfg, data);
                        let pv = build_relationship_long_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(key.clone(), projected_property);
                    }
                    ValueType::Int => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index =
                                            old_offsets[old_source as usize] + neighbor_idx;
                                        let v =
                                            values.long_value(old_index as u64).map_err(|err| {
                                                GraphStoreError::InvalidOperation(format!("{err}"))
                                            })?;
                                        data.push(v as i32);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<i32>(data.len());
                        let backend = create_int_backend_from_config(&cfg, data);
                        let pv = build_relationship_int_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(key.clone(), projected_property);
                    }
                    _ => continue,
                }
            }

            let projected_store = builder.build();
            if !projected_store.is_empty() {
                projected.insert(rel_type.clone(), projected_store);
            }
        }

        Ok(projected)
    }

    fn set_modified(&mut self) {
        self.modification_time = Utc::now();
    }

    fn schema_labels(&self) -> HashSet<NodeLabel> {
        self.id_map
            .available_node_labels()
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect()
    }

    // === Property Management with Config ===

    /// Add a node property with i64 values using the store's config for backend selection.
    pub fn add_node_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();

        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<i64>(node_count);

        // Create property using config
        let backend = create_long_backend_from_config(&collections_config, values);
        let pv = build_node_long_property_values(backend, node_count);

        self.node_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a node property with f64 values using the store's config for backend selection.
    pub fn add_node_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();

        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<f64>(node_count);

        // Create property using config
        let backend = create_double_backend_from_config(&collections_config, values);
        let pv = build_node_double_property_values(backend, node_count);

        self.node_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a graph property with i64 values using the store's config for backend selection.
    pub fn add_graph_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig (graph properties don't scale with node count)
        let collections_config = self.config.graph_collections_config::<i64>(values.len());

        // Create property using config
        let backend = create_long_backend_from_config(&collections_config, values);
        let pv = build_graph_long_property_values(backend);

        self.graph_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a graph property with f64 values using the store's config for backend selection.
    pub fn add_graph_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig
        let collections_config = self.config.graph_collections_config::<f64>(values.len());

        // Create property using config
        let backend = create_double_backend_from_config(&collections_config, values);
        let pv = build_graph_double_property_values(backend);

        self.graph_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    fn to_schema_label(label: &NodeLabel) -> NodeLabel {
        NodeLabel::of(label.name())
    }

    fn label_key(label: &NodeLabel) -> String {
        label.name().to_string()
    }

    fn rebuild_relationship_metadata(&mut self) {
        let mut ordered: Vec<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();
        ordered.sort_by(|left, right| left.name().cmp(right.name()));

        let mut inverse_indexed = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel = false;

        for rel_type in &ordered {
            if let Some(topology) = self.relationship_topologies.get(rel_type) {
                if topology.is_inverse_indexed() {
                    inverse_indexed.insert(rel_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel = true;
                }
                relationship_count += topology.relationship_count();
            }
        }

        let all_inverse_indexed = !ordered.is_empty()
            && ordered
                .iter()
                .all(|rel_type| inverse_indexed.contains(rel_type));

        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        match self.schema.direction() {
            Direction::Directed => {
                characteristics_builder = characteristics_builder.directed();
            }
            Direction::Undirected => {
                characteristics_builder = characteristics_builder.undirected();
            }
        }

        if all_inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }

        self.ordered_relationship_types = ordered;
        self.inverse_indexed_relationship_types = inverse_indexed;
        self.relationship_count = relationship_count;
        self.has_parallel_relationships = has_parallel;
        self.graph_characteristics = characteristics_builder.build();
    }

    fn refresh_relationship_property_state(&mut self) {
        self.has_relationship_properties = self
            .relationship_property_stores
            .values()
            .any(|store| !store.is_empty());
    }
}

impl GraphStore for DefaultGraphStore {
    fn database_info(&self) -> &DatabaseInfo {
        &self.database_info
    }

    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }

    fn modification_time(&self) -> DateTime<Utc> {
        self.modification_time
    }

    fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        Arc::clone(&self.id_map) as Arc<dyn IdMap>
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        self.graph_properties.keys().cloned().collect()
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        self.graph_properties.contains_key(property_key)
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        if let Some(property_values) = self.graph_properties.get(property_key) {
            return Ok(property_values.value_type());
        }

        // Fall back to schema if property not found in stores
        if let Some(property_schema) = self.schema.graph_properties().get(property_key) {
            return Ok(property_schema.value_type());
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        self.graph_properties
            .get(property_key)
            .cloned()
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_graph_property(
        &mut self,
        property_key: impl Into<String>,
        property_values: Arc<dyn GraphPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        self.graph_properties.insert(key, property_values);
        self.set_modified();
        Ok(())
    }

    fn remove_graph_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        if self.graph_properties.remove(property_key).is_some() {
            self.set_modified();
            Ok(())
        } else {
            Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
        }
    }

    fn node_count(&self) -> usize {
        self.id_map.node_count()
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        let schema_label = Self::to_schema_label(label);
        self.id_map.node_count_for_label(&schema_label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        self.schema_labels()
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        let schema_label = Self::to_schema_label(label);
        self.schema_labels().contains(&schema_label)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) -> GraphStoreResult<()> {
        let schema_label = Self::to_schema_label(&node_label);
        Arc::make_mut(&mut self.id_map).add_node_label(schema_label);
        self.set_modified();
        Ok(())
    }

    fn node_property_keys(&self) -> HashSet<String> {
        self.node_properties.keys().cloned().collect()
    }

    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .cloned()
            .unwrap_or_default()
    }

    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String> {
        if labels.is_empty() {
            return self.node_property_keys();
        }

        let mut iter = labels.iter();
        let first = iter.next().unwrap();
        let mut intersection = self.node_property_keys_for_label(first);

        for label in iter {
            let keys = self.node_property_keys_for_label(label);
            intersection = intersection
                .intersection(&keys)
                .cloned()
                .collect::<HashSet<_>>();
        }

        intersection
    }

    fn has_node_property(&self, property_key: &str) -> bool {
        self.node_properties.contains_key(property_key)
    }

    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .map(|keys| keys.contains(property_key))
            .unwrap_or(false)
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        if let Some(property_values) = self.node_properties.get(property_key) {
            return Ok(property_values.value_type());
        }

        // Fall back to schema if property not found in stores
        for entry in self.schema.node_schema().entries() {
            if let Some(property_schema) = entry.properties().get(property_key) {
                return Ok(property_schema.value_type());
            }
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        self.node_properties
            .get(property_key)
            .cloned()
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_node_property(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property_key: impl Into<String>,
        property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        self.node_properties.insert(key.clone(), property_values);

        for label in node_labels {
            let label_key = Self::label_key(&label);
            self.node_properties_by_label
                .entry(label_key)
                .or_default()
                .insert(key.clone());
        }

        self.set_modified();
        Ok(())
    }

    fn remove_node_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        if self.node_properties.remove(property_key).is_some() {
            for keys in self.node_properties_by_label.values_mut() {
                keys.remove(property_key);
            }
            self.set_modified();
            Ok(())
        } else {
            Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
        }
    }

    fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.relationship_topologies
            .get(relationship_type)
            .map(|topology| topology.relationship_count())
            .unwrap_or(0)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        let mut types: HashSet<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();

        for schema_type in self.schema.relationship_schema().available_types() {
            types.insert(RelationshipType::of(schema_type.name()));
        }

        types
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        self.relationship_types().contains(relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.inverse_indexed_relationship_types.clone()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        self.relationship_property_stores
            .values()
            .flat_map(|store| store.relationship_properties().keys().cloned())
            .collect()
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.relationship_properties().keys().cloned().collect())
            .unwrap_or_default()
    }

    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        rel_types
            .iter()
            .flat_map(|rel_type| self.relationship_property_keys_for_type(rel_type))
            .collect()
    }

    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.contains_key(property_key))
            .unwrap_or(false)
    }

    fn relationship_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        for store in self.relationship_property_stores.values() {
            if let Some(property) = store.get(property_key) {
                return Ok(property.property_schema().value_type());
            }
        }

        // Fall back to schema if property not found in stores
        for entry in self.schema.relationship_schema().entries() {
            if let Some(property_schema) = entry.properties().get(property_key) {
                return Ok(property_schema.value_type());
            }
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        self.relationship_property_stores
            .get(relationship_type)
            .and_then(|store| store.get(property_key))
            .map(|property| property.values_arc())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_relationship_property(
        &mut self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
        property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        let property = RelationshipProperty::with_state(
            key.clone(),
            PropertyState::Persistent,
            property_values,
        );

        let store = self
            .relationship_property_stores
            .remove(&relationship_type)
            .unwrap_or_else(RelationshipPropertyStore::empty);

        let updated_store = store.to_builder().put(key, property).build();
        self.relationship_property_stores
            .insert(relationship_type, updated_store);

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn remove_relationship_property(
        &mut self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<()> {
        let store = self
            .relationship_property_stores
            .remove(relationship_type)
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))?;

        if !store.contains_key(property_key) {
            // Restore the store since the property wasn't found
            self.relationship_property_stores
                .insert(relationship_type.clone(), store);
            return Err(GraphStoreError::PropertyNotFound(property_key.to_string()));
        }

        let updated_store = store.to_builder().remove_property(property_key).build();

        if !updated_store.is_empty() {
            self.relationship_property_stores
                .insert(relationship_type.clone(), updated_store);
        }

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn delete_relationships(
        &mut self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult> {
        if let Some(topology) = self.relationship_topologies.remove(relationship_type) {
            let removed_count = topology.relationship_count();
            self.relationship_property_stores.remove(relationship_type);
            self.rebuild_relationship_metadata();
            self.refresh_relationship_property_state();
            self.set_modified();
            Ok(DeletionResult::with_counts(
                self.graph_name.clone(),
                0,
                removed_count,
            ))
        } else {
            Err(GraphStoreError::RelationshipTypeNotFound(
                relationship_type.name().to_string(),
            ))
        }
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            self.graph_characteristics,
            topologies,
            self.ordered_relationship_types.clone(),
            self.inverse_indexed_relationship_types.clone(),
            self.relationship_count,
            self.has_parallel_relationships,
            self.node_properties.clone(),
            self.relationship_property_stores.clone(),
            HashMap::new(),
        ))
    }

    fn get_graph_with_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> GraphResult<Arc<dyn Graph>> {
        self.graph()
            .relationship_type_filtered_graph(relationship_types)
    }

    fn get_graph_with_types_and_selectors(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
    ) -> GraphResult<Arc<dyn Graph>> {
        // Build a DefaultGraph then let it select properties based on provided selectors
        let selectors = relationship_property_selectors.clone();

        // If selector missing and exactly one property exists for a type, allow DefaultGraph::new to auto-select
        // by passing selectors as-is; DefaultGraph::new handles auto-selection.
        let topologies = self
            .relationship_topologies
            .iter()
            .filter(|(rel_type, _)| relationship_types.contains(*rel_type))
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        let mut ordered_types = self
            .ordered_relationship_types
            .iter()
            .filter(|rel_type| relationship_types.contains(*rel_type))
            .cloned()
            .collect::<Vec<_>>();

        let mut inverse_indexed_types = self
            .inverse_indexed_relationship_types
            .iter()
            .filter(|rel_type| relationship_types.contains(*rel_type))
            .cloned()
            .collect::<HashSet<_>>();

        let relationship_count: usize = topologies
            .values()
            .map(|top| top.relationship_count())
            .sum();

        let has_parallel_edges = topologies.values().any(|top| top.has_parallel_edges());

        // Characteristics: preserve directed/undirected; inverse_indexed only if all selected are inverse indexed
        let all_inverse_indexed = !ordered_types.is_empty()
            && ordered_types
                .iter()
                .all(|t| inverse_indexed_types.contains(t));
        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        match self.schema.direction() {
            Direction::Directed => {
                characteristics_builder = characteristics_builder.directed();
            }
            Direction::Undirected => {
                characteristics_builder = characteristics_builder.undirected();
            }
        }
        if all_inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }
        let filtered_characteristics = characteristics_builder.build();

        // Filter relationship properties and apply selectors
        let filtered_relationship_properties = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_property_stores
                    .get(rel_type)
                    .map(|store| (rel_type.clone(), store.clone()))
            })
            .collect::<HashMap<_, _>>();

        // DefaultGraph::new expects selectors keyed by type name
        let filtered_selectors = selectors
            .into_iter()
            .filter(|(rel_type, _)| ordered_types.contains(rel_type))
            .collect::<HashMap<_, _>>();

        let filtered_graph = DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            filtered_characteristics,
            topologies,
            std::mem::take(&mut ordered_types),
            std::mem::take(&mut inverse_indexed_types),
            relationship_count,
            has_parallel_edges,
            self.node_properties.clone(),
            filtered_relationship_properties,
            filtered_selectors,
        );

        Ok(Arc::new(filtered_graph))
    }

    fn get_graph_with_types_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        _orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>> {
        // Orientation informs traversal; return a filtered view by types.
        self.graph()
            .relationship_type_filtered_graph(relationship_types)
    }

    fn get_graph_with_types_selectors_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>> {
        let view = self.get_graph_with_types_and_selectors(
            relationship_types,
            relationship_property_selectors,
        )?;
        let _ = orientation; // reserved for future orientation-aware views
        Ok(view)
    }
}

fn build_node_long_property_values(
    backend: LongCollection,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongNodePropertyValues::<VecLong>::from_collection(collection, node_count),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
                vec_backend,
                node_count,
            ))
        }
        LongCollection::Arrow(collection) => Arc::new(DefaultLongNodePropertyValues::<
            ArrowLongArray,
        >::from_collection(
            collection, node_count
        )),
    }
}

fn build_node_double_property_values(
    backend: DoubleCollection,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => Arc::new(
            DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(collection, node_count),
        ),
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(
                DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
                    vec_backend,
                    node_count,
                ),
            )
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleNodePropertyValues::<
            ArrowDoubleArray,
        >::from_collection(
            collection, node_count
        )),
    }
}

fn build_node_float_property_values(
    backend: VecFloat,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(DefaultFloatNodePropertyValues::<VecFloat>::from_collection(
        backend, node_count,
    ))
}

fn build_node_int_property_values(
    backend: VecInt,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(DefaultIntNodePropertyValues::<VecInt>::from_collection(
        backend, node_count,
    ))
}

fn build_node_double_array_property_values(
    backend: VecDoubleArray,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(
        DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
            backend, node_count,
        ),
    )
}

fn build_node_long_array_property_values(
    backend: VecLongArray,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(
        DefaultLongArrayNodePropertyValues::<VecLongArray>::from_collection(backend, node_count),
    )
}

fn build_graph_long_property_values(backend: LongCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongGraphPropertyValues::<VecLong>::from_collection(collection),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::from_collection(
                vec_backend,
            ))
        }
        LongCollection::Arrow(collection) => {
            Arc::new(DefaultLongGraphPropertyValues::<ArrowLongArray>::from_collection(collection))
        }
    }
}

fn build_graph_double_property_values(backend: DoubleCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => {
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(collection))
        }
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(vec_backend))
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleGraphPropertyValues::<
            ArrowDoubleArray,
        >::from_collection(collection)),
    }
}

fn build_relationship_long_property_values(
    backend: LongCollection,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                collection,
                relationship_count,
            ),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(
                DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                    vec_backend,
                    relationship_count,
                ),
            )
        }
        LongCollection::Arrow(collection) => Arc::new(DefaultLongRelationshipPropertyValues::<
            ArrowLongArray,
        >::from_collection(
            collection, relationship_count
        )),
    }
}

fn build_relationship_double_property_values(
    backend: DoubleCollection,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => Arc::new(DefaultDoubleRelationshipPropertyValues::<
            VecDouble,
        >::from_collection(
            collection, relationship_count
        )),
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(
                DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
                    vec_backend,
                    relationship_count,
                ),
            )
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleRelationshipPropertyValues::<
            ArrowDoubleArray,
        >::from_collection(
            collection, relationship_count
        )),
    }
}

fn build_relationship_int_property_values(
    backend: VecInt,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    Arc::new(
        DefaultIntRelationshipPropertyValues::<VecInt>::from_collection(
            backend,
            relationship_count,
        ),
    )
}
fn relationship_prefix_offsets(topology: &RelationshipTopology) -> Vec<usize> {
    let mut prefix = Vec::with_capacity(topology.node_capacity() + 1);
    let mut total = 0usize;
    prefix.push(total);
    for node in 0..topology.node_capacity() {
        let degree = topology
            .outgoing(node as MappedNodeId)
            .map(|neighbors| neighbors.len())
            .unwrap_or(0);
        total += degree;
        prefix.push(total);
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation};
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use std::sync::Arc;

    fn store_with_config(config: GraphStoreConfig) -> DefaultGraphStore {
        let graph_name = GraphName::new("g");
        let database_info = DatabaseInfo::new(
            DatabaseId::new("db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let schema = GraphSchema::empty();
        let capabilities = Capabilities::default();
        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

        let topology = RelationshipTopology::new(vec![vec![1, 2], vec![2], vec![]], None);

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(RelationshipType::of("KNOWS"), topology);

        DefaultGraphStore::new(
            config,
            graph_name,
            database_info,
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        )
    }

    fn sample_store() -> DefaultGraphStore {
        store_with_config(GraphStoreConfig::default())
    }

    #[test]
    fn graph_view_reflects_store_data() {
        let store = sample_store();
        assert_eq!(store.node_count(), 3);
        assert_eq!(store.relationship_count(), 3);

        let graph = store.graph();
        assert_eq!(graph.relationship_count(), 3);
        assert!(graph.characteristics().is_undirected());
        assert_eq!(graph.degree(0), 2);
    }

    #[test]
    fn test_add_node_property_with_config() {
        use crate::config::GraphStoreConfig;

        // Create config with specific backend
        let config = GraphStoreConfig::default();

        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );

        // Add property - should use Vec backend from config
        store
            .add_node_property_i64("age".to_string(), vec![1, 2, 3])
            .unwrap();

        // Verify property exists
        assert!(store.node_properties.contains_key("age"));
        assert_eq!(store.node_properties.len(), 1);
    }

    #[test]
    fn test_add_graph_property_with_config() {
        use crate::config::GraphStoreConfig;

        let config = GraphStoreConfig::default();

        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );

        // Add graph property
        store
            .add_graph_property_f64("density".to_string(), vec![0.5])
            .unwrap();

        // Verify property exists
        assert!(store.graph_properties.contains_key("density"));
        assert_eq!(store.graph_properties.len(), 1);
    }

    #[test]
    fn vec_only_config_keeps_node_properties_vec_backed() {
        let mut store = store_with_config(GraphStoreConfig::vec_only());
        store
            .add_node_property_i64("score".to_string(), vec![10, 20, 30])
            .expect("add node property via vec-only config");

        let values = store
            .node_property_values("score")
            .expect("retrieve node property values");
        assert_eq!(values.long_value(1).unwrap(), 20);
        assert_eq!(values.long_value(2).unwrap(), 30);
    }

    #[test]
    fn manages_relationship_properties() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");

        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));

        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .expect("add relationship property");

        assert!(store.has_relationship_property(&rel_type, "weight"));
        assert!(store.relationship_property_keys().contains("weight"));
        let retrieved = store
            .relationship_property_values(&rel_type, "weight")
            .expect("retrieve property");
        assert_eq!(retrieved.double_value(1).unwrap(), 2.0);
        assert!(store.graph().has_relationship_property());

        store
            .remove_relationship_property(&rel_type, "weight")
            .expect("remove relationship property");
        assert!(!store.has_relationship_property(&rel_type, "weight"));
        assert!(!store.graph().has_relationship_property());
    }

    #[test]
    fn builds_node_values_from_arrow_backend() {
        let backend = LongCollection::Arrow(ArrowLongArray::from_vec(vec![5, 10]));
        let values = build_node_long_property_values(backend, 2);
        assert_eq!(values.long_value(1).unwrap(), 10);
        assert_eq!(values.node_count(), 2);
    }

    #[test]
    fn builds_graph_values_from_arrow_backend() {
        let backend = DoubleCollection::Arrow(ArrowDoubleArray::from_vec(vec![0.25, 0.75]));
        let values = build_graph_double_property_values(backend);
        let collected: Vec<f64> = values.double_values().collect();
        assert_eq!(collected, vec![0.25, 0.75]);
    }
}
