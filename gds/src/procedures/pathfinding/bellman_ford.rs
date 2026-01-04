//! Bellman-Ford Facade
//!
//! Single-source shortest paths with negative-cycle detection.
//!
//! This facade runs the translated Bellman-Ford runtime against a live
//! `DefaultGraphStore` (no dummy outputs).

use crate::mem::MemoryRange;
use crate::algo::bellman_ford::{BellmanFordComputationRuntime, BellmanFordStorageRuntime};
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{PathResult as ProcedurePathResult, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks,
};
use crate::algo::core::result_builders::{
    ExecutionMetadata,
    PathFindingResult,
    PathResult as CorePathResult,
    PathResultBuilder,
    ResultBuilder,
};

/// Statistics about Bellman-Ford execution
#[derive(Debug, Clone, serde::Serialize)]
pub struct BellmanFordStats {
    pub paths_found: u64,
    pub negative_cycles_found: u64,
    pub contains_negative_cycle: bool,
    pub execution_time_ms: u64,
}

/// Bellman-Ford algorithm builder - fluent configuration
pub struct BellmanFordBuilder {
    graph_store: Arc<DefaultGraphStore>,
    source: Option<u64>,
    weight_property: String,
    relationship_types: Vec<String>,
    direction: String,
    track_negative_cycles: bool,
    track_paths: bool,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl BellmanFordBuilder {
    /// Create a new Bellman-Ford builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - track_negative_cycles: true
    /// - track_paths: true
    /// - concurrency: 4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            source: None,
            weight_property: "weight".to_string(),
            relationship_types: vec![],
            direction: "outgoing".to_string(),
            track_negative_cycles: true,
            track_paths: true,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    pub fn track_negative_cycles(mut self, enabled: bool) -> Self {
        self.track_negative_cycles = enabled;
        self
    }

    pub fn track_paths(mut self, enabled: bool) -> Self {
        self.track_paths = enabled;
        self
    }

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
        if self.source.is_none() {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "source node must be specified".to_string(),
                ),
            );
        }

        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        match self.direction.to_ascii_lowercase().as_str() {
            "outgoing" | "incoming" => {}
            other => {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                        "direction must be 'outgoing' or 'incoming' (got '{other}')"
                    )),
                );
            }
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    fn checked_node_id(value: u64, field: &str) -> Result<NodeId> {
        NodeId::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "{field} must fit into i64 (got {value})",
            ))
        })
    }

    fn checked_u64(value: NodeId, context: &str) -> Result<u64> {
        u64::try_from(value).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "Bellman-Ford returned invalid node id for {context}: {value}",
            ))
        })
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.validate()?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        // Create progress tracker for Bellman-Ford execution
        let node_count = self.graph_store.node_count();
        let _progress_tracker = crate::core::utils::progress::TaskProgressTracker::new(
            Tasks::leaf_with_volume("BellmanFord".to_string(), node_count),
        );

        let source_u64 = self.source.expect("validate ensures source is set");
        let source_node = Self::checked_node_id(source_u64, "source")?;

        let rel_types: HashSet<RelationshipType> = if self.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) = match self.direction.to_ascii_lowercase().as_str() {
            "incoming" => (Orientation::Reverse, 1u8),
            _ => (Orientation::Natural, 0u8),
        };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let mut storage = BellmanFordStorageRuntime::new(
            source_node,
            self.track_negative_cycles,
            self.track_paths,
            self.concurrency,
        );

        let mut computation = BellmanFordComputationRuntime::new(
            source_node,
            self.track_negative_cycles,
            self.track_paths,
            self.concurrency,
        );

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("bellman_ford".to_string(), graph_view.relationship_count()),
            self.concurrency,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_bellman_ford(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;

        let paths: Vec<CorePathResult> = result
            .shortest_paths
            .iter()
            .filter(|p| p.source_node >= 0 && p.target_node >= 0)
            .map(|p| {
                let source = Self::checked_u64(p.source_node, "source").unwrap_or(0);
                let target = Self::checked_u64(p.target_node, "target").unwrap_or(0);
                let path = p
                    .node_ids
                    .iter()
                    .copied()
                    .filter(|node_id| *node_id >= 0)
                    .map(|node_id| Self::checked_u64(node_id, "path").unwrap_or(0))
                    .collect();

                CorePathResult {
                    source,
                    target,
                    path,
                    cost: p.total_cost,
                }
            })
            .collect();

        // Create execution metadata
        let execution_time = start.elapsed();
        let metadata = ExecutionMetadata {
            execution_time,
            iterations: None,
            converged: Some(!result.contains_negative_cycle),
            additional: std::collections::HashMap::from([
                (
                    "negative_cycles_found".to_string(),
                    result.negative_cycles.len().to_string(),
                ),
                (
                    "contains_negative_cycle".to_string(),
                    result.contains_negative_cycle.to_string(),
                ),
            ]),
        };

        // Build result using upgraded result builder
        let path_result = PathResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(e.to_string())
            })?;

        Ok(path_result)
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = ProcedurePathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(
            result
                .paths
                .into_iter()
                .map(super::core_to_procedure_path_result),
        ))
    }

    pub fn stats(self) -> Result<BellmanFordStats> {
        let result = self.compute()?;
        let negative_cycles_found = result
            .metadata
            .additional
            .get("negative_cycles_found")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let contains_negative_cycle = result
            .metadata
            .additional
            .get("contains_negative_cycle")
            .and_then(|s| s.parse().ok())
            .unwrap_or(false);

        Ok(BellmanFordStats {
            paths_found: result.paths.len() as u64,
            negative_cycles_found,
            contains_negative_cycle,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Bellman-Ford mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Bellman-Ford mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for Bellman-Ford execution
    ///
    /// Returns a memory range estimate based on distance arrays, predecessor tracking, and negative cycle detection.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Distance array (updated in each iteration)
        let distance_memory = node_count * 8;

        // Predecessor array (if tracking paths)
        let predecessor_memory = if self.track_paths { node_count * 8 } else { 0 };

        // Negative cycle tracking
        let cycle_tracking_memory = if self.track_negative_cycles {
            node_count * 8
        } else {
            0
        };

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory =
            distance_memory + predecessor_memory + cycle_tracking_memory + graph_overhead;
        let overhead = total_memory / 5;
        MemoryRange::of_range(total_memory, total_memory + overhead)
    }
}
