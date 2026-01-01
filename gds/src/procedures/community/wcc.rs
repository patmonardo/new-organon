//! Weakly Connected Components (WCC) Facade
//!
//! Finds connected components in a graph under *undirected* semantics.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; current runtime is single-threaded.

use crate::core::utils::progress::TaskRegistry;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::wcc::{WccComputationRuntime, WccStorageRuntime};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node WCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct WccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// Aggregated WCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct WccStats {
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// WCC algorithm facade.
#[derive(Clone)]
pub struct WccFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl WccFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = WccRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| WccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(self) -> Result<WccStats> {
        let (result, elapsed) = self.compute()?;
        Ok(WccStats {
            component_count: result.component_count,
            execution_time_ms: elapsed,
        })
    }

    pub fn mutate(self, _property_name: &str) -> Result<MutationResult> {
        let (_result, _elapsed) = self.compute()?;

        // TODO: Implement actual node property mutation
        // For now, return a placeholder result
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "WCC mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        // For WCC, write is the same as mutate since it's node properties
        self.mutate(property_name).map(|_| {
            WriteResult::new(
                0, // TODO: Return actual count
                property_name.to_string(),
                std::time::Duration::from_millis(0), // TODO: Return actual elapsed time
            )
        })
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for WCC computation
        // - Component assignments: node_count * 8 bytes
        // - Union-find structures: node_count * 16 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let union_find_memory = node_count * 16;
        let graph_memory = node_count * 16;

        let total = assignment_memory + union_find_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<(crate::algo::wcc::WccResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let storage = WccStorageRuntime::new(self.concurrency);
        let mut computation = WccComputationRuntime::new();
        let result = storage.compute_wcc(&mut computation, graph_view.as_ref());

        Ok((
            crate::algo::wcc::WccResult {
                components: result.components,
                component_count: result.component_count,
            },
            start.elapsed().as_millis() as u64,
        ))
    }

    /// Full result: returns the procedure-level WCC result.
    pub fn run(&self) -> Result<crate::algo::wcc::WccResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
