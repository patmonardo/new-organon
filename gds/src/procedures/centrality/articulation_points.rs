//! Articulation Points Facade
//!
//! Articulation points (cut vertices) are nodes whose removal increases the
//! number of connected components in an undirected graph.
//!
//! This facade is the "live wiring" layer: it binds the algorithm runtime to a
//! `DefaultGraphStore` graph view.

use crate::algo::algorithms::{AlgorithmRunner, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::articulation_points::storage::ArticulationPointsStorageRuntime;
use crate::algo::articulation_points::{
    ArticulationPointRow, ArticulationPointsComputationRuntime, ArticulationPointsConfig,
    ArticulationPointsMutateResult, ArticulationPointsMutationSummary, ArticulationPointsResult,
    ArticulationPointsResultBuilder, ArticulationPointsStats, STACK_EVENT_SIZE_BYTES,
};
use crate::collections::backends::vec::VecDouble;
use crate::collections::BitSet;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, ProgressTracker, TaskProgressTracker, TaskRegistryFactory,
    Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Articulation points facade bound to a live graph store.
#[derive(Clone)]
pub struct ArticulationPointsFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ArticulationPointsConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl ArticulationPointsFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ArticulationPointsConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ArticulationPointsConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: ArticulationPointsConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: ArticulationPointsConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    /// Validate the facade configuration.
    ///
    /// # Returns
    /// Ok(()) if configuration is valid, Err otherwise
    ///
    /// # Errors
    /// Returns an error if concurrency is not positive
    pub fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    /// Run the algorithm and return the articulation points as a bitset
    pub fn run(&self) -> Result<BitSet> {
        // Articulation points are defined on undirected connectivity.
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        let _relationship_count = graph_view.relationship_count();
        if node_count == 0 {
            return Ok(BitSet::new(0));
        }

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("articulation_points".to_string(), node_count)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        // Create both runtimes (factory pattern)
        let storage = ArticulationPointsStorageRuntime::new(&*self.graph_store)?;
        let mut computation = ArticulationPointsComputationRuntime::new(node_count);

        // Call storage.compute_articulation_points - Applications talk only to procedures
        let result = storage.compute_articulation_points(
            &mut computation,
            Some(graph_view.as_ref()),
            &mut progress_tracker,
        )?;

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result.articulation_points)
    }

    /// Estimate memory requirements for articulation points computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::ArticulationPointsFacade;
    /// let facade = ArticulationPointsFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        let bitset_bytes = (node_count + 7) / 8;
        let visited_bytes = bitset_bytes;
        let articulation_bytes = bitset_bytes;

        // tin/low/children: i64 per node (HugeLongArray is long-backed).
        let per_node_arrays_bytes = node_count.saturating_mul(3).saturating_mul(8);

        // Java parity: DFS event stack sized by relationship count.
        let stack_bytes = relationship_count.saturating_mul(STACK_EVENT_SIZE_BYTES);

        let total_memory = visited_bytes
            .saturating_add(articulation_bytes)
            .saturating_add(per_node_arrays_bytes)
            .saturating_add(stack_bytes);

        // Conservative overhead for Vec/BitSet headers + allocator slack.
        let total_with_overhead = total_memory.saturating_add(total_memory / 5);

        MemoryRange::of_range(total_memory, total_with_overhead)
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            AlgorithmError::Execution(format!("node_id must fit into i64 (got {})", value))
        })
    }

    fn compute_bitset(&self) -> Result<(BitSet, std::time::Duration)> {
        let start = Instant::now();

        // Articulation points are defined on undirected connectivity.
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        let relationship_count = graph_view.relationship_count();
        if node_count == 0 {
            return Ok((BitSet::new(0), start.elapsed()));
        }

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("articulation_points".to_string(), node_count)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0)
                .map(|target| target as usize)
                .collect()
        };

        let mut runtime = ArticulationPointsComputationRuntime::new(node_count);
        let result =
            runtime.compute_with_relationship_count(node_count, relationship_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result.articulation_points, start.elapsed()))
    }

    fn compute(&self) -> Result<ArticulationPointsResult> {
        let (bitset, elapsed) = self.compute_bitset()?;
        let mut points: Vec<u64> = Vec::with_capacity(bitset.cardinality());
        let mut idx = bitset.next_set_bit(0);
        while let Some(i) = idx {
            points.push(i as u64);
            idx = bitset.next_set_bit(i + 1);
        }

        Ok(ArticulationPointsResult {
            articulation_points: points,
            node_count: self.graph_store.node_count(),
            execution_time: elapsed,
        })
    }

    /// Stream mode: Get articulation points for each node
    ///
    /// Returns an iterator over articulation point rows.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let results = graph.articulation_points().stream()?.collect::<Vec<_>>();
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ArticulationPointRow>>> {
        self.validate()?;
        let result = self.compute()?;
        let rows = ArticulationPointsResultBuilder::new(result).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns articulation point count and execution time.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let stats = graph.articulation_points().stats()?;
    /// println!("Found {} articulation points", stats.articulation_point_count);
    /// ```
    pub fn stats(&self) -> Result<ArticulationPointsStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(ArticulationPointsResultBuilder::new(result).stats())
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores articulation point status as a node property (1.0 for articulation points, 0.0 otherwise).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let result = graph.articulation_points().mutate("is_articulation_point")?;
    /// println!("Computed and stored for {} nodes", result.summary.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<ArticulationPointsMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = Instant::now();
        let (bitset, _elapsed) = self.compute_bitset()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;
        let mut scores: Vec<f64> = Vec::with_capacity(node_count);
        for node_id in 0..node_count {
            let is_articulation = bitset.get(node_id);
            scores.push(if is_articulation { 1.0 } else { 0.0 });
        }

        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "Articulation Points mutate failed to add property: {e}"
                ))
            })?;

        let execution_time = start_time.elapsed();
        let summary = ArticulationPointsMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        };

        Ok(ArticulationPointsMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for Articulation Points.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(AlgorithmError::Execution(
            "Articulation Points mutate/write is not implemented yet".to_string(),
        ))
    }
}

impl AlgorithmRunner for ArticulationPointsFacade {
    fn algorithm_name(&self) -> &'static str {
        "articulationPoints"
    }

    fn description(&self) -> &'static str {
        "Find cut vertices (articulation points) in an undirected graph"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;

    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::PropertyValues;
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_undirected_edges(
        node_count: usize,
        edges: &[(usize, usize)],
    ) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            outgoing[b].push(a as i64);
            incoming[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Undirected);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn facade_finds_articulation_points_on_path() {
        // 0-1-2-3-4 => 1,2,3
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&0));
        assert!(!ids.contains(&4));
    }

    #[test]
    fn facade_cycle_has_no_articulation_points() {
        // 0-1-2-3-0
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn mutate_adds_articulation_point_property() {
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .articulation_points()
            .mutate("is_articulation_point")
            .unwrap();

        let values = result
            .updated_store
            .node_property_values("is_articulation_point")
            .unwrap();

        assert_eq!(values.element_count(), 5);
        assert_eq!(values.double_value(0).unwrap(), 0.0);
        assert_eq!(values.double_value(1).unwrap(), 1.0);
        assert_eq!(values.double_value(2).unwrap(), 1.0);
        assert_eq!(values.double_value(3).unwrap(), 1.0);
        assert_eq!(values.double_value(4).unwrap(), 0.0);
    }
}
