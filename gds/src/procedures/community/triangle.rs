//! Triangle Facade
//!
//! Counts triangles in an undirected graph and returns per-node and global counts.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: reserved for future parallel implementation
//! - `max_degree`: filter to skip high-degree nodes (performance / approximation)

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::triangle::{
    TriangleComputationRuntime, TriangleConfig, TriangleMutateResult, TriangleMutationSummary,
    TriangleResult, TriangleResultBuilder, TriangleStats, TriangleStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, LeafTask, ProgressTracker, TaskProgressTracker, Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node triangle count row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct TriangleRow {
    pub node_id: u64,
    pub triangles: u64,
}

/// Triangle algorithm facade.
#[derive(Clone)]
pub struct TriangleFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: TriangleConfig,
}

impl TriangleFacade {
    /// Create a new facade bound to a live graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: TriangleConfig::default(),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: TriangleConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: TriangleConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: TriangleConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Concurrency hint (reserved for future parallel implementation).
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Skip nodes with degree > max_degree.
    pub fn max_degree(mut self, max_degree: u64) -> Self {
        self.config.max_degree = max_degree;
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<TriangleResult> {
        self.validate()?;
        let start = Instant::now();

        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok(TriangleResult {
                local_triangles: Vec::new(),
                global_triangles: 0,
                node_count: 0,
                execution_time: start.elapsed(),
            });
        }

        let leaf: LeafTask = Tasks::leaf_with_volume("triangle".to_string(), node_count);
        let base_task = leaf.base().clone();
        let registry_factory = EmptyTaskRegistryFactory;
        let mut progress_tracker: Box<dyn ProgressTracker> =
            Box::new(TaskProgressTracker::with_registry(
                base_task,
                Concurrency::of(self.config.concurrency.max(1)),
                JobId::new(),
                &registry_factory,
            ));

        let config = self.config.clone();

        let termination_flag = TerminationFlag::default();
        let storage = TriangleStorageRuntime::new();
        let mut runtime = TriangleComputationRuntime::new();
        let result = storage
            .compute(
                &mut runtime,
                self.graph_store.as_ref(),
                &config,
                progress_tracker.as_mut(),
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(TriangleResult {
            local_triangles: result.local_triangles,
            global_triangles: result.global_triangles,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: yields `(node_id, triangles)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = TriangleRow>>> {
        let result = self.compute()?;
        let iter = result
            .local_triangles
            .into_iter()
            .enumerate()
            .map(|(node_id, triangles)| TriangleRow {
                node_id: node_id as u64,
                triangles,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields global triangle count.
    pub fn stats(&self) -> Result<TriangleStats> {
        let result = self.compute()?;
        Ok(TriangleResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes triangle counts back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<TriangleMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let local = result.local_triangles;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        // Convert u64 counts to i64 backend
        let longs: Vec<i64> = local.into_iter().map(|v| v as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Triangle mutate failed to add property: {e}"))
            })?;

        let summary = TriangleMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(TriangleMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes triangle counts to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Full result: returns both local and global counts.
    pub fn run(&self) -> Result<TriangleResult> {
        let result = self.compute()?;
        Ok(result)
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Triangle count stores per-node local counts and uses neighbor traversal.
        // Some implementations keep adjacency lists; we estimate proportional to (n + m).
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: u64 local triangle count + temporary counters.
        let per_node = 64usize;
        // Per relationship: potential adjacency materialization (conservative).
        let per_relationship = 24usize;

        let base: usize = 64 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(3)))
    }
}
