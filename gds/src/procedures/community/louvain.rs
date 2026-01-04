//! Louvain Facade
//!
//! Louvain is a modularity-optimization community detection algorithm.
//!
//! Note: the underlying Louvain implementation in this crate is currently a
//! placeholder. The facade is wired to the procedure module and supports seeding
//! to keep API parity and determinism while the full modularity optimization
//! runtime is built out.
//!
//! Parameters:
//! - `concurrency`

use crate::core::utils::progress::TaskRegistry;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::louvain::{
    LouvainComputationRuntime, LouvainConfig, LouvainResult, LouvainStorageRuntime,
};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use crate::core::utils::progress::Tasks;

/// Per-node Louvain assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LouvainRow {
    pub node_id: u64,
    pub community_id: u64,
}

/// Aggregated Louvain stats.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct LouvainStats {
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Louvain algorithm facade.
#[derive(Clone)]
pub struct LouvainFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: LouvainConfig,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl LouvainFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LouvainConfig { concurrency: 4 },
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = LouvainRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .data
            .into_iter()
            .enumerate()
            .map(|(node_id, community_id)| LouvainRow {
                node_id: node_id as u64,
                community_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(self) -> Result<LouvainStats> {
        let (result, elapsed) = self.compute()?;
        let community_count = result
            .data
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();
        Ok(LouvainStats {
            community_count,
            execution_time_ms: elapsed,
        })
    }

    pub fn mutate(self, _property_name: &str) -> Result<MutationResult> {
        let (_result, _elapsed) = self.compute()?;

        // Note: node property mutation is deferred.
        // For now, return a placeholder result
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "Louvain mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        // For Louvain, write is the same as mutate since it's node properties
        self.mutate(property_name).map(|_| {
            WriteResult::new(
                0, // Note: placeholder count until mutation is wired.
                property_name.to_string(),
                std::time::Duration::from_millis(0), // Note: placeholder time until mutation is wired.
            )
        })
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for Louvain computation
        // - Community assignments: node_count * 8 bytes
        // - Modularity calculations: node_count * 8 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let modularity_memory = node_count * 8;
        let graph_memory = node_count * 16;

        let total = assignment_memory + modularity_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        Ok(())
    }

    fn compute(&self) -> Result<(LouvainResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let storage = LouvainStorageRuntime::new(self.config.concurrency);
        let mut computation = LouvainComputationRuntime::new();

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("louvain".to_string(), graph_view.relationship_count()),
            self.config.concurrency,
        );

        let result = storage.compute_louvain(
            &mut computation,
            graph_view.as_ref(),
            &mut progress_tracker,
        );
        Ok((result, start.elapsed().as_millis() as u64))
    }

    /// Full result: returns the procedure-level Louvain result.
    pub fn run(&self) -> Result<LouvainResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
