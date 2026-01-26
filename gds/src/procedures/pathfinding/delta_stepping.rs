//! Delta Stepping Facade
//!
//! Single-source shortest paths using the Delta Stepping binning strategy.
//!
//! This facade runs the translated Delta Stepping runtime against a live
//! `DefaultGraphStore`.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::delta_stepping::{
    DeltaSteppingComputationRuntime, DeltaSteppingConfig, DeltaSteppingMutateResult,
    DeltaSteppingMutationSummary, DeltaSteppingResultBuilder, DeltaSteppingStats,
    DeltaSteppingStorageRuntime, DeltaSteppingWriteSummary,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::PathFindingResult;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;

/// Delta Stepping algorithm builder - fluent configuration
pub struct DeltaSteppingFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: DeltaSteppingConfig,
    weight_property: String,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type DeltaSteppingBuilder = DeltaSteppingFacade;

impl DeltaSteppingFacade {
    /// Create a new Delta Stepping builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - delta: 1.0
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - store_predecessors: true
    /// - concurrency: 4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: DeltaSteppingConfig::default(),
            weight_property: "weight".to_string(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: DeltaSteppingConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            weight_property: "weight".to_string(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: DeltaSteppingConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: DeltaSteppingConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    pub fn delta(mut self, delta: f64) -> Self {
        self.config.delta = delta;
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    pub fn store_predecessors(mut self, enabled: bool) -> Self {
        self.config.store_predecessors = enabled;
        self
    }

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
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        if self.weight_property.is_empty() {
            return Err(AlgorithmError::Execution(
                "weight_property cannot be empty".to_string(),
            ));
        }

        Ok(())
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

        // Create progress tracker for Delta Stepping execution
        let node_count = self.graph_store.node_count();
        let _progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "DeltaStepping".to_string(),
            node_count,
        ));

        let source_node = self.config.source_node;

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

        let mut storage = DeltaSteppingStorageRuntime::new(
            source_node,
            self.config.delta,
            self.config.concurrency,
            self.config.store_predecessors,
        );

        let mut computation = DeltaSteppingComputationRuntime::new(
            source_node,
            self.config.delta,
            self.config.concurrency,
            self.config.store_predecessors,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "delta_stepping".to_string(),
                graph_view.relationship_count(),
            ),
            self.config.concurrency,
        );

        let start = std::time::Instant::now();
        let result = storage.compute_delta_stepping(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;
        let execution_time = start.elapsed();
        DeltaSteppingResultBuilder::result(result, execution_time)
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    pub fn stats(self) -> Result<DeltaSteppingStats> {
        let result = self.compute()?;
        let computation_time_ms = result
            .metadata
            .additional
            .get("computation_time_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(DeltaSteppingStats {
            paths_found: result.paths.len() as u64,
            computation_time_ms,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<DeltaSteppingMutateResult> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let result = self.compute()?;
        let paths = result.paths;

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = DeltaSteppingMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(DeltaSteppingMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<DeltaSteppingWriteSummary> {
        self.validate()?;
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(DeltaSteppingWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for Delta Stepping execution
    ///
    /// Returns a memory range estimate based on bucket storage, distance arrays, and graph overhead.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Bucket storage (delta stepping uses bins/buckets)
        let bucket_memory = node_count * 16; // bucket index + node_id

        // Distance array
        let distance_memory = node_count * 8;

        // Predecessor tracking (if enabled)
        let predecessor_memory = if self.config.store_predecessors {
            node_count * 8
        } else {
            0
        };

        // Graph structure overhead
        let avg_degree = 10.0;
        let relationship_count = (node_count as f64 * avg_degree) as usize;
        let graph_overhead = relationship_count * 16;

        let total_memory = bucket_memory + distance_memory + predecessor_memory + graph_overhead;
        let overhead = total_memory / 5;
        MemoryRange::of_range(total_memory, total_memory + overhead)
    }
}
