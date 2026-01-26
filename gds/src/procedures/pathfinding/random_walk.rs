//! Random Walk Facade
//!
//! Generates random walks from nodes in the graph using biased sampling.
//! Supports node2vec-style exploration with configurable return and in-out factors.

use crate::algo::random_walk::{
    RandomWalkComputationRuntime, RandomWalkConfig, RandomWalkMutateResult,
    RandomWalkMutationSummary, RandomWalkResult, RandomWalkResultBuilder, RandomWalkRow,
    RandomWalkStats, RandomWalkWriteSummary,
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

/// Random Walk algorithm facade
pub struct RandomWalkFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: RandomWalkConfig,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type RandomWalkBuilder = RandomWalkFacade;

impl RandomWalkFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: RandomWalkConfig::default(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: RandomWalkConfig,
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
        let parsed: RandomWalkConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: RandomWalkConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn walks_per_node(mut self, count: usize) -> Self {
        self.config.walks_per_node = count;
        self
    }

    pub fn walk_length(mut self, length: usize) -> Self {
        self.config.walk_length = length;
        self
    }

    pub fn return_factor(mut self, factor: f64) -> Self {
        self.config.return_factor = factor;
        self
    }

    pub fn in_out_factor(mut self, factor: f64) -> Self {
        self.config.in_out_factor = factor;
        self
    }

    pub fn source_nodes(mut self, nodes: Vec<u64>) -> Self {
        self.config.source_nodes = nodes;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = Some(seed);
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Random walk benefits from parallelism when generating many walks.
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

    fn compute(self) -> Result<(RandomWalkResult, std::time::Duration)> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let start = Instant::now();

        // Random walk works on directed graphs (Natural orientation)
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((RandomWalkResult { walks: Vec::new() }, start.elapsed()));
        }

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("random_walk".to_string(), node_count),
            self.config.concurrency,
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph_view.default_property_value();

        // Convert source nodes to internal IDs
        let source_nodes_internal: Vec<usize> = self
            .config
            .source_nodes
            .clone()
            .into_iter()
            .map(|n| n as usize)
            .collect();

        // Get neighbors
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            graph_view
                .stream_relationships(node_idx as i64, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        Some(target as usize)
                    } else {
                        None
                    }
                })
                .collect()
        };

        let seed = self.config.random_seed.unwrap_or_else(|| {
            use std::time::SystemTime;
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let runtime = RandomWalkComputationRuntime::new(
            self.config.walks_per_node,
            self.config.walk_length,
            self.config.return_factor,
            self.config.in_out_factor,
            source_nodes_internal,
            seed,
        );

        let result = runtime.compute(node_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((result, start.elapsed()))
    }

    /// Stream mode: yields walk sequences
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = RandomWalkRow>>> {
        let (result, elapsed) = self.compute()?;
        let rows = RandomWalkResultBuilder::new(result, elapsed).rows();

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: returns aggregated statistics
    pub fn stats(self) -> Result<RandomWalkStats> {
        let (result, elapsed) = self.compute()?;
        Ok(RandomWalkResultBuilder::new(result, elapsed).stats())
    }

    /// Mutate mode: Compute and update in-memory graph projection
    ///
    /// Stores random walks as node properties.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.random_walk();
    /// let result = builder.mutate("walks")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<RandomWalkMutateResult> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let execution_time_ms = elapsed.as_millis() as u64;
        let paths: Vec<PathResult> = RandomWalkResultBuilder::new(result, elapsed).paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = RandomWalkMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms,
        };

        Ok(RandomWalkMutateResult {
            summary,
            updated_store,
        })
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists random walks to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.random_walk();
    /// let result = builder.write("walks")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<RandomWalkWriteSummary> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(RandomWalkWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for random walk execution
    ///
    /// Returns a memory range estimate based on:
    /// - Walk storage (walks_per_node * walk_length * node_count)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.random_walk();
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Walk storage: each walk is walk_length * 8 bytes (u64 per node)
        // Total walks: walks_per_node * source_nodes.len() or node_count if empty
        let source_count = if self.config.source_nodes.is_empty() {
            node_count
        } else {
            self.config.source_nodes.len()
        };
        let total_walks = self.config.walks_per_node * source_count;
        let walk_storage = total_walks * self.config.walk_length * 8;

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory = walk_storage + graph_overhead;

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
    fn facade_generates_walks() {
        // Simple path: 0 -> 1 -> 2
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let walks: Vec<_> = graph
            .random_walk()
            .walks_per_node(1)
            .walk_length(3)
            .source_nodes(vec![0])
            .random_seed(42)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(walks.len(), 1);
        assert_eq!(walks[0].path, vec![0, 1, 2]);
    }

    #[test]
    fn facade_computes_stats() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph
            .random_walk()
            .walks_per_node(5)
            .walk_length(3)
            .source_nodes(vec![0])
            .stats()
            .unwrap();

        assert_eq!(stats.walk_count, 5);
        assert!(stats.execution_time_ms < 1000);
    }

    #[test]
    fn facade_walks_from_all_nodes() {
        // Triangle
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2), (2, 0)]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph
            .random_walk()
            .walks_per_node(2)
            .walk_length(5)
            .stats()
            .unwrap();

        // 3 nodes * 2 walks per node = 6 walks
        assert_eq!(stats.walk_count, 6);
    }
}
