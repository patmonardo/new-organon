//! Topological Sort Facade
//!
//! Orders nodes in a directed acyclic graph (DAG) such that for every edge (u, v),
//! u appears before v. Optionally computes longest path distances.

use crate::algo::topological_sort::{
    TopologicalSortComputationRuntime, TopologicalSortConfig, TopologicalSortMutateResult,
    TopologicalSortMutationSummary, TopologicalSortResult, TopologicalSortResultBuilder,
    TopologicalSortRow, TopologicalSortStats, TopologicalSortWriteSummary,
};
use crate::mem::MemoryRange;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, ProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::projection::eval::algorithm::AlgorithmError;

/// Topological Sort algorithm facade
pub struct TopologicalSortFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: TopologicalSortConfig,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type TopologicalSortBuilder = TopologicalSortFacade;

impl TopologicalSortFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: TopologicalSortConfig::default(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: TopologicalSortConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: TopologicalSortConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: TopologicalSortConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn compute_max_distance(mut self, value: bool) -> Self {
        self.config.compute_max_distance_from_source = value;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Topological sort benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(self) -> Result<(TopologicalSortResult, std::time::Duration)> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let start = Instant::now();

        // Topological sort works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((
                TopologicalSortResult {
                    sorted_nodes: Vec::new(),
                    max_source_distances: None,
                },
                start.elapsed(),
            ));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("topological_sort".to_string(), node_count),
            self.config.concurrency,
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();

        // Get neighbors with weights
        let get_neighbors = |node_idx: i64| -> Vec<(i64, f64)> {
            graph_view
                .stream_relationships(node_idx, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target < 0 {
                        return None;
                    }
                    let weight = cursor.property();
                    Some((target, weight))
                })
                .collect()
        };

        let mut runtime = TopologicalSortComputationRuntime::new(
            node_count,
            self.config.compute_max_distance_from_source,
        );
        let result = runtime.compute(node_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result, start.elapsed()))
    }

    /// Stream mode: yields (node_id, max_distance) for each node in topological order
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = TopologicalSortRow>>> {
        let (result, elapsed) = self.compute()?;
        let rows = TopologicalSortResultBuilder::new(result, elapsed).rows();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(self) -> Result<TopologicalSortStats> {
        let (result, elapsed) = self.compute()?;
        Ok(TopologicalSortResultBuilder::new(result, elapsed).stats())
    }

    /// Mutate mode: Compute and update in-memory graph projection
    ///
    /// Stores topological order and distances as node properties.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort().compute_max_distance(true);
    /// let result = builder.mutate("topological_order")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<TopologicalSortMutateResult> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let execution_time_ms = elapsed.as_millis() as u64;
        let paths: Vec<PathResult> = TopologicalSortResultBuilder::new(result, elapsed).paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = TopologicalSortMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms,
        };

        Ok(TopologicalSortMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists topological sort results to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort().compute_max_distance(true);
    /// let result = builder.write("topological_sort")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<TopologicalSortWriteSummary> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(TopologicalSortWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for topological sort execution
    ///
    /// Returns a memory range estimate based on:
    /// - Node ordering arrays
    /// - Distance arrays (if computing max distances)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.topological_sort();
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Node ordering array: node_count * 8 bytes (u64 per node)
        let ordering_memory = node_count * 8;

        // Distance array: node_count * 8 bytes (f64 per node, if computing distances)
        let distance_memory = if self.config.compute_max_distance_from_source {
            node_count * 8
        } else {
            0
        };

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory = ordering_memory + distance_memory + graph_overhead;

        // Add 20% overhead for algorithm-specific structures
        let overhead = total_memory / 5;
        let total_with_overhead = total_memory + overhead;

        MemoryRange::of_range(total_memory, total_with_overhead)
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
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
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
    fn facade_computes_topological_order() {
        // Simple DAG: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.topological_sort().stream().unwrap().collect();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].node_id, 0);
        assert_eq!(rows[1].node_id, 1);
        assert_eq!(rows[2].node_id, 2);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph.topological_sort().stats().unwrap();

        assert_eq!(stats.node_count, 3);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_computes_max_distances() {
        // Diamond DAG
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .topological_sort()
            .compute_max_distance(true)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 4);

        // All nodes should have distances computed
        assert!(rows.iter().all(|r| r.max_distance.is_some()));

        // Node 0 (source) should have distance 0
        let node_0 = rows.iter().find(|r| r.node_id == 0).unwrap();
        assert_eq!(node_0.max_distance, Some(0.0));
    }
}
