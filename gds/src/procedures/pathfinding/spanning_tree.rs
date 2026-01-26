//! Spanning Tree (Prim) Facade
//!
//! Computes a minimum or maximum spanning tree rooted at a start node.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::spanning_tree::{
    SpanningTreeConfig, SpanningTreeMutateResult, SpanningTreeMutationSummary, SpanningTreeResult,
    SpanningTreeResultBuilder, SpanningTreeRow, SpanningTreeStats, SpanningTreeStorageRuntime,
    SpanningTreeWriteSummary,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::projection::eval::algorithm::AlgorithmError;

/// Spanning tree facade builder.
///
/// Defaults:
/// - start_node: None (must be set)
/// - compute_minimum: true
/// - relationship_types: all
/// - direction: "undirected" (MST semantics)
/// - weight_property: "weight"
/// - concurrency: 4
pub struct SpanningTreeFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: SpanningTreeConfig,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type SpanningTreeBuilder = SpanningTreeFacade;

impl SpanningTreeFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: SpanningTreeConfig::default(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: SpanningTreeConfig,
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
        let parsed: SpanningTreeConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: SpanningTreeConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Set start node
    ///
    /// The algorithm starts the spanning tree from this node.
    /// Must be a valid node ID in the graph.
    pub fn start_node(mut self, start_node: u64) -> Self {
        self.config.start_node_id = u32::try_from(start_node).unwrap_or(u32::MAX);
        self
    }

    /// Set start node ID directly
    pub fn start_node_id(mut self, start_node: u32) -> Self {
        self.config.start_node_id = start_node;
        self
    }

    /// Set whether to compute minimum or maximum spanning tree
    ///
    /// If true, computes minimum spanning tree (MST).
    /// If false, computes maximum spanning tree.
    /// Default: true
    pub fn compute_minimum(mut self, compute_minimum: bool) -> Self {
        self.config.compute_minimum = compute_minimum;
        self
    }

    /// Set weight property name
    ///
    /// Property must exist on relationships and contain numeric values.
    /// Default: "weight"
    pub fn weight_property(mut self, property: &str) -> Self {
        self.config.weight_property = property.to_string();
        self
    }

    /// Restrict traversal to the provided relationship types.
    ///
    /// Empty means all relationship types.
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction.
    ///
    /// Accepted values: "outgoing", "incoming", "undirected" (default).
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Spanning tree benefits from parallelism in large graphs.
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
        if self.config.start_node_id == u32::MAX {
            return Err(AlgorithmError::Execution(
                "start_node must fit into u32".to_string(),
            ));
        }

        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(self) -> Result<SpanningTreeResult> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let start_node_id = self.config.start_node_id;

        let rel_types: HashSet<RelationshipType> = if self.config.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) =
            match self.config.direction.to_ascii_lowercase().as_str() {
                "incoming" => (Orientation::Reverse, 1u8),
                "undirected" => (Orientation::Natural, 2u8),
                _ => (Orientation::Natural, 0u8),
            };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.config.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = SpanningTreeStorageRuntime::new(
            start_node_id,
            self.config.compute_minimum,
            self.config.concurrency,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("spanning_tree".to_string(), graph_view.relationship_count()),
            self.config.concurrency,
        );

        let start = Instant::now();

        let tree = storage
            .compute_spanning_tree_with_graph(
                graph_view.as_ref(),
                direction_byte,
                &mut progress_tracker,
            )
            .map_err(|e| {
                AlgorithmError::Execution(format!("Spanning tree computation failed: {e}"))
            })?;

        let computation_time_ms = start.elapsed().as_millis() as u64;
        Ok(SpanningTreeResult::new(tree, computation_time_ms))
    }

    /// Stream mode: yield per-node rows.
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = SpanningTreeRow>>> {
        let result = self.compute()?;
        let rows = SpanningTreeResultBuilder::new(result).rows();
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats.
    pub fn stats(self) -> Result<SpanningTreeStats> {
        let result = self.compute()?;
        Ok(SpanningTreeResultBuilder::new(result).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<SpanningTreeMutateResult> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let result = self.compute()?;
        let execution_time_ms = result.computation_time_ms;
        let paths: Vec<PathResult> = SpanningTreeResultBuilder::new(result).paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = SpanningTreeMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms,
        };
        Ok(SpanningTreeMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<SpanningTreeWriteSummary> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(SpanningTreeWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for spanning tree execution
    ///
    /// Returns a memory range estimate based on:
    /// - Priority queue storage (for Prim's algorithm)
    /// - Tree structure storage (parent and cost arrays)
    /// - Graph structure overhead
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph: Graph = unimplemented!();
    /// let builder = graph.spanning_tree().start_node(0);
    /// let memory = builder.estimate_memory();
    /// println!("Estimated memory: {} bytes", memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Priority queue (open set) - worst case: all nodes in queue
        // Each entry: node_id (8 bytes) + cost (8 bytes) + heap overhead (16 bytes) = 32 bytes
        let priority_queue_memory = node_count * 32;

        // Tree storage: parent array (8 bytes per node) + cost array (8 bytes per node)
        let tree_storage_memory = node_count * 8 * 2;

        // Visited set: bit vector or hash set (~1 byte per node)
        let visited_memory = node_count;

        // Graph structure overhead (adjacency lists, etc.)
        let avg_degree = 10.0; // Conservative estimate
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16; // ~16 bytes per relationship

        let total_memory =
            priority_queue_memory + tree_storage_memory + visited_memory + graph_overhead;

        // Add 20% overhead for algorithm-specific structures
        let overhead = total_memory / 5;
        let total_with_overhead = total_memory + overhead;

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(11),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let builder = SpanningTreeBuilder::new(store());
        assert_eq!(builder.config.start_node_id, 0);
        assert!(builder.config.compute_minimum);
        assert!(builder.config.relationship_types.is_empty());
        assert_eq!(builder.config.direction, "undirected");
        assert_eq!(builder.config.weight_property, "weight");
        assert_eq!(builder.config.concurrency, 1);
    }

    #[test]
    fn test_stream_smoke() {
        let store = store();
        let rows: Vec<_> = GraphFacade::new(store)
            .spanning_tree()
            .start_node(0)
            .compute_minimum(true)
            .stream()
            .unwrap()
            .collect();

        assert!(!rows.is_empty());
    }

    #[test]
    fn test_stats_smoke() {
        let store = store();
        let stats = GraphFacade::new(store)
            .spanning_tree()
            .start_node(0)
            .compute_minimum(true)
            .stats()
            .unwrap();

        assert!(stats.effective_node_count > 0);
    }
}
