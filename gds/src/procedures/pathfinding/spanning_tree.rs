//! Spanning Tree (Prim) Facade
//!
//! Computes a minimum or maximum spanning tree rooted at a start node.

use crate::algo::spanning_tree::SpanningTreeComputationRuntime;
use crate::algo::spanning_tree::SpanningTreeStorageRuntime;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::{PathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::projection::eval::algorithm::AlgorithmError;

/// Per-node spanning tree row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SpanningTreeRow {
    pub node: u64,
    pub parent: Option<u64>,
    pub cost_to_parent: f64,
}

/// Aggregated stats for spanning tree.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SpanningTreeStats {
    pub effective_node_count: u64,
    pub total_weight: f64,
    pub computation_time_ms: u64,
}

/// Mutate result for spanning tree: summary + updated store
#[derive(Debug, Clone)]
pub struct SpanningTreeMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

/// Spanning tree facade builder.
///
/// Defaults:
/// - start_node: None (must be set)
/// - compute_minimum: true
/// - relationship_types: all
/// - direction: "undirected" (MST semantics)
/// - weight_property: "weight"
/// - concurrency: 4
pub struct SpanningTreeBuilder {
    graph_store: Arc<DefaultGraphStore>,
    start_node: Option<u64>,
    compute_minimum: bool,
    relationship_types: Vec<String>,
    direction: String,
    weight_property: String,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl SpanningTreeBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            start_node: None,
            compute_minimum: true,
            relationship_types: vec![],
            direction: "undirected".to_string(),
            weight_property: "weight".to_string(),
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Set start node
    ///
    /// The algorithm starts the spanning tree from this node.
    /// Must be a valid node ID in the graph.
    pub fn start_node(mut self, start_node: u64) -> Self {
        self.start_node = Some(start_node);
        self
    }

    /// Set whether to compute minimum or maximum spanning tree
    ///
    /// If true, computes minimum spanning tree (MST).
    /// If false, computes maximum spanning tree.
    /// Default: true
    pub fn compute_minimum(mut self, compute_minimum: bool) -> Self {
        self.compute_minimum = compute_minimum;
        self
    }

    /// Set weight property name
    ///
    /// Property must exist on relationships and contain numeric values.
    /// Default: "weight"
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Restrict traversal to the provided relationship types.
    ///
    /// Empty means all relationship types.
    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    /// Set traversal direction.
    ///
    /// Accepted values: "outgoing", "incoming", "undirected" (default).
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Spanning tree benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
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
        match self.start_node {
            None => {
                return Err(AlgorithmError::Execution(
                    "start_node must be specified".to_string(),
                ))
            }
            Some(id) if id == u64::MAX => {
                return Err(AlgorithmError::Execution(
                    "start_node ID cannot be u64::MAX".to_string(),
                ))
            }
            _ => {}
        }

        if self.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be > 0".to_string(),
            ));
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" | "undirected" => {}
            other => {
                return Err(AlgorithmError::Execution(format!(
                    "direction must be 'outgoing', 'incoming', or 'undirected' (got '{other}')"
                )));
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    fn compute(self) -> Result<(Vec<SpanningTreeRow>, SpanningTreeStats)> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let start_node_id: u32 = u32::try_from(self.start_node.unwrap()).map_err(|_| {
            AlgorithmError::Execution(format!(
                "start_node must fit into u32 (got {})",
                self.start_node.unwrap()
            ))
        })?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            "undirected" => (Orientation::Natural, 2u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage =
            SpanningTreeStorageRuntime::new(start_node_id, self.compute_minimum, self.concurrency);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("spanning_tree".to_string(), graph_view.relationship_count()),
            self.concurrency,
        );

        let start = std::time::Instant::now();

        // Create computation runtime (factory pattern)
        let mut computation = SpanningTreeComputationRuntime::new(
            start_node_id,
            self.compute_minimum,
            graph_view.node_count() as u32,
            self.concurrency,
        );

        // Call storage.compute_spanning_tree() - Applications never call ::algo:: directly
        let tree = storage.compute_spanning_tree(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;

        let mut rows = Vec::with_capacity(tree.node_count as usize);
        for node_id in 0..tree.node_count {
            let parent = tree.parent(node_id);
            let parent_u64 = if parent < 0 {
                None
            } else {
                Some(parent as u64)
            };
            rows.push(SpanningTreeRow {
                node: node_id as u64,
                parent: parent_u64,
                cost_to_parent: tree.cost_to_parent(node_id),
            });
        }

        let stats = SpanningTreeStats {
            effective_node_count: tree.effective_node_count() as u64,
            total_weight: tree.total_weight(),
            computation_time_ms: start.elapsed().as_millis() as u64,
        };

        Ok((rows, stats))
    }

    /// Stream mode: yield per-node rows.
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = SpanningTreeRow>>> {
        let (rows, _) = self.compute()?;
        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregated tree stats.
    pub fn stats(self) -> Result<SpanningTreeStats> {
        let (_, stats) = self.compute()?;
        Ok(stats)
    }

    pub fn mutate(self, property_name: &str) -> Result<SpanningTreeMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (rows, stats) = self.compute()?;

        let paths: Vec<PathResult> = rows
            .into_iter()
            .filter_map(|row| {
                row.parent.map(|parent| PathResult {
                    source: parent,
                    target: row.node,
                    path: vec![parent, row.node],
                    cost: row.cost_to_parent,
                })
            })
            .collect();

        let updated_store =
            super::build_path_relationship_store(graph_store.as_ref(), property_name, &paths)?;

        let summary = MutationResult::new(
            paths.len() as u64,
            property_name.to_string(),
            std::time::Duration::from_millis(stats.computation_time_ms),
        );

        Ok(SpanningTreeMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
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
        assert_eq!(builder.start_node, None);
        assert!(builder.compute_minimum);
        assert!(builder.relationship_types.is_empty());
        assert_eq!(builder.direction, "undirected");
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.concurrency, 4);
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
