//! Bridges Facade
//!
//! Live wiring for bridge edge detection in undirected graphs.

use crate::algo::algorithms::{AlgorithmRunner, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::bridges::storage::BridgesStorageRuntime;
use crate::algo::bridges::{
    Bridge, BridgesComputationRuntime, BridgesConfig, BridgesMutateResult, BridgesMutationSummary,
    BridgesResult, BridgesResultBuilder, BridgesRow, BridgesStats,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Bridges facade bound to a live graph store
#[derive(Clone)]
pub struct BridgesFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: BridgesConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl BridgesFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: BridgesConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: BridgesConfig,
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
        let parsed: BridgesConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: BridgesConfig) -> Result<Self> {
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

    /// Run the algorithm and return the bridges
    pub fn run(&self) -> Result<Vec<Bridge>> {
        // Create both runtimes (factory pattern)
        let storage = BridgesStorageRuntime::new(&*self.graph_store)?;
        let mut computation = BridgesComputationRuntime::new_with_stack_capacity(
            storage.node_count(),
            storage.relationship_count(),
        );

        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("bridges".to_string(), node_count)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let termination = TerminationFlag::running_true();
        let progress_handle = progress_tracker.clone();
        let on_node_scanned = Arc::new(move || {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(1);
        });

        // Call storage.compute_bridges() - Applications talk only to procedures
        let result = storage
            .compute_bridges(&mut computation, &termination, on_node_scanned)
            .map_err(|e| AlgorithmError::Execution(format!("Bridges terminated: {e}")))?;

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result.bridges)
    }

    /// Stream mode: Get bridges for each edge
    ///
    /// Returns an iterator over bridge edge rows.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let results = graph.bridges().stream()?.collect::<Vec<_>>();
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = BridgesRow>>> {
        self.validate()?;
        let result = self.compute()?;
        let rows = BridgesResultBuilder::new(result).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns bridge count and execution time.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let stats = graph.bridges().stats()?;
    /// println!("Found {} bridges", stats.bridge_count);
    /// ```
    pub fn stats(&self) -> Result<BridgesStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(BridgesResultBuilder::new(result).stats())
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores bridge status as edge properties (1.0 for bridges, 0.0 otherwise).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let result = graph.bridges().mutate("is_bridge")?;
    /// println!("Computed and stored for {} edges", result.summary.edges_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<BridgesMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let bridges = result.bridges.clone();
        let builder = BridgesResultBuilder::new(result);
        let bridge_set: HashSet<(u64, u64)> = bridges
            .into_iter()
            .map(|bridge| (bridge.from, bridge.to))
            .collect();

        let mut new_store = (*self.graph_store).clone();
        let mut edges_updated = 0u64;

        for rel_type in self.graph_store.relationship_types() {
            let mut rel_types = HashSet::new();
            rel_types.insert(rel_type.clone());

            let graph = self
                .graph_store
                .get_graph_with_types(&rel_types)
                .map_err(|e| {
                    AlgorithmError::Execution(format!(
                        "Bridges mutate failed to read relationships: {e}"
                    ))
                })?;

            let fallback = graph.default_property_value();
            let mut values: Vec<f64> = Vec::with_capacity(graph.relationship_count());

            for node_id in 0..graph.node_count() {
                let source = node_id as i64;
                for cursor in graph.stream_relationships(source, fallback) {
                    let target = cursor.target_id();
                    if target < 0 {
                        continue;
                    }
                    let (a, b) = if source <= target {
                        (source as u64, target as u64)
                    } else {
                        (target as u64, source as u64)
                    };
                    let is_bridge = bridge_set.contains(&(a, b));
                    values.push(if is_bridge { 1.0 } else { 0.0 });
                }
            }

            edges_updated += values.len() as u64;
            if values.is_empty() {
                continue;
            }

            let element_count = values.len();
            let pv: Arc<dyn RelationshipPropertyValues> = Arc::new(
                DefaultRelationshipPropertyValues::with_values(values, 0.0, element_count),
            );

            new_store
                .add_relationship_property(rel_type, property_name, pv)
                .map_err(|e| {
                    AlgorithmError::Execution(format!("Bridges mutate failed to add property: {e}"))
                })?;
        }

        let summary = BridgesMutationSummary {
            edges_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(BridgesMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for Bridges.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(AlgorithmError::Execution(
            "Bridges mutate/write is not implemented yet".to_string(),
        ))
    }

    /// Estimate memory requirements for bridges computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::BridgesFacade;
    /// let facade = BridgesFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Memory for bridges vector (each bridge is 2 u64s)
        let bridges_memory = node_count * 16; // Rough estimate for bridge storage

        // Memory for DFS stack and discovery arrays in bridges algorithm
        let dfs_memory = node_count * 12; // Rough estimate: 3 integers per node (discovery, low, parent)

        // Additional overhead for computation (temporary vectors, etc.)
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory = bridges_memory + dfs_memory + computation_overhead;
        let total_with_overhead = total_memory + (total_memory / 5); // Add 20% overhead

        MemoryRange::of_range(total_memory, total_with_overhead)
    }

    fn compute_bridges(&self) -> Result<(Vec<Bridge>, std::time::Duration)> {
        let start = Instant::now();

        // Create both runtimes (factory pattern)
        let storage = BridgesStorageRuntime::new(&*self.graph_store)?;
        let mut computation = BridgesComputationRuntime::new_with_stack_capacity(
            storage.node_count(),
            storage.relationship_count(),
        );

        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("bridges".to_string(), node_count)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let termination = TerminationFlag::running_true();
        let progress_handle = progress_tracker.clone();
        let on_node_scanned = Arc::new(move || {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(1);
        });

        // Call storage.compute_bridges() - Applications talk only to procedures
        let result = storage
            .compute_bridges(&mut computation, &termination, on_node_scanned)
            .map_err(|e| AlgorithmError::Execution(format!("Bridges terminated: {e}")))?;

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result.bridges, start.elapsed()))
    }

    fn compute(&self) -> Result<BridgesResult> {
        let (bridges, elapsed) = self.compute_bridges()?;
        Ok(BridgesResult {
            bridges,
            node_count: self.graph_store.node_count(),
            execution_time: elapsed,
        })
    }
}

impl AlgorithmRunner for BridgesFacade {
    fn algorithm_name(&self) -> &'static str {
        "bridges"
    }

    fn description(&self) -> &'static str {
        "Find bridge edges (cut edges) in an undirected graph"
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
    fn facade_finds_bridge_on_path() {
        // 0-1-2-3 => all edges are bridges
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();

        assert_eq!(rows.len(), 3);
        // All edges should be bridges
        assert!(rows
            .iter()
            .any(|r| (r.from == 0 && r.to == 1) || (r.from == 1 && r.to == 0)));
        assert!(rows
            .iter()
            .any(|r| (r.from == 1 && r.to == 2) || (r.from == 2 && r.to == 1)));
        assert!(rows
            .iter()
            .any(|r| (r.from == 2 && r.to == 3) || (r.from == 3 && r.to == 2)));
    }

    #[test]
    fn facade_cycle_has_no_bridges() {
        // 0-1-2-3-0 => no bridges (all edges in cycle)
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn facade_bridge_connects_cycles() {
        // Two cycles connected by a bridge: 0-1-2-0 and 3-4-5-3, connected by 2-3
        let store = store_from_undirected_edges(
            6,
            &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)],
        );
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.bridges().stream().unwrap().collect();

        assert_eq!(rows.len(), 1);
        let bridge = &rows[0];
        assert!((bridge.from == 2 && bridge.to == 3) || (bridge.from == 3 && bridge.to == 2));
    }

    #[test]
    fn mutate_adds_bridge_property() {
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.bridges().mutate("is_bridge").unwrap();
        let rel_type = RelationshipType::of("REL");
        let values = result
            .updated_store
            .relationship_property_values(&rel_type, "is_bridge")
            .unwrap();

        assert_eq!(values.element_count(), 6);
        for idx in 0..values.element_count() {
            assert_eq!(values.double_value(idx as u64).unwrap(), 1.0);
        }
    }
}
