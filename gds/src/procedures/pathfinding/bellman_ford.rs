use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::algo::bellman_ford::{
    BellmanFordComputationRuntime, BellmanFordConfig, BellmanFordMutateResult, BellmanFordResult,
    BellmanFordResultBuilder, BellmanFordStats, BellmanFordStorageRuntime, BellmanFordWriteSummary,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Import upgraded systems
use crate::algo::algorithms::pathfinding::PathResult;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskProgressTracker, TaskRegistryFactory, Tasks,
};

/// Bellman-Ford algorithm facade - config-oriented builder
pub struct BellmanFordFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: BellmanFordConfig,
    weight_property: String,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type BellmanFordBuilder = BellmanFordFacade;

impl BellmanFordFacade {
    /// Create a new Bellman-Ford facade bound to a live graph store.
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
            config: BellmanFordConfig::default(),
            weight_property: "weight".to_string(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: BellmanFordConfig,
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
        raw_config: &JsonValue,
    ) -> Result<Self> {
        let parsed: BellmanFordConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: BellmanFordConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
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

    pub fn track_negative_cycles(mut self, enabled: bool) -> Self {
        self.config.track_negative_cycles = enabled;
        self
    }

    pub fn track_paths(mut self, enabled: bool) -> Self {
        self.config.track_paths = enabled;
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

    fn compute(self) -> Result<(BellmanFordResult, std::time::Duration)> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        // Set up progress tracking
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        // Create progress tracker for Bellman-Ford execution
        let node_count = self.graph_store.node_count();
        let _progress_tracker = TaskProgressTracker::new(Tasks::leaf_with_volume(
            "BellmanFord".to_string(),
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

        let mut storage = BellmanFordStorageRuntime::new(
            source_node,
            self.config.track_negative_cycles,
            self.config.track_paths,
            self.config.concurrency,
        );

        let mut computation = BellmanFordComputationRuntime::new(
            source_node,
            self.config.track_negative_cycles,
            self.config.track_paths,
            self.config.concurrency,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("bellman_ford".to_string(), graph_view.relationship_count()),
            self.config.concurrency,
        );

        let start = std::time::Instant::now();
        let result: BellmanFordResult = storage.compute_bellman_ford(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;
        Ok((result, start.elapsed()))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let (result, elapsed) = self.compute()?;
        let paths = BellmanFordResultBuilder::new(result, elapsed).paths();
        Ok(Box::new(paths.into_iter()))
    }

    pub fn stats(self) -> Result<BellmanFordStats> {
        let (result, elapsed) = self.compute()?;
        Ok(BellmanFordResultBuilder::new(result, elapsed).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<BellmanFordMutateResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let graph_store = Arc::clone(&self.graph_store);
        let (result, elapsed) = self.compute()?;
        let builder = BellmanFordResultBuilder::new(result, elapsed);
        let paths = builder.paths();

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = builder.mutation_summary(property_name, paths.len() as u64);

        Ok(BellmanFordMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<BellmanFordWriteSummary> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let res = self.mutate(property_name)?;
        Ok(BellmanFordWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for Bellman-Ford execution
    ///
    /// Returns a memory range estimate based on distance arrays, predecessor tracking, and negative cycle detection.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Distance array (updated in each iteration)
        let distance_memory = node_count * 8;

        // Predecessor array (if tracking paths)
        let predecessor_memory = if self.config.track_paths {
            node_count * 8
        } else {
            0
        };

        // Negative cycle tracking
        let cycle_tracking_memory = if self.config.track_negative_cycles {
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
