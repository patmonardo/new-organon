//! K1-Coloring Facade
//!
//! Greedy iterative graph coloring.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.
//! - `max_iterations`: maximum number of coloring/validation iterations (must be >= 1).
//! - `batch_size`: accepted for parity; currently unused.

use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::k1coloring::{K1ColoringComputationRuntime, K1ColoringResult};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node color assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct K1ColoringRow {
    pub node_id: u64,
    pub color_id: u64,
}

/// Aggregated K1-Coloring stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct K1ColoringStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub color_count: usize,
    pub execution_time_ms: u64,
}

/// K1-Coloring algorithm facade.
#[derive(Clone)]
pub struct K1ColoringFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_iterations: u64,
    batch_size: usize,
    task_registry: Option<TaskRegistry>,
}

impl K1ColoringFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            max_iterations: 10,
            batch_size: crate::core::utils::partition::DEFAULT_BATCH_SIZE,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn max_iterations(mut self, max_iterations: u64) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        ConfigValidator::in_range(
            self.max_iterations as f64,
            1.0,
            1_000_000_000.0,
            "max_iterations",
        )?;
        ConfigValidator::in_range(self.batch_size as f64, 1.0, 1_000_000_000.0, "batch_size")?;
        Ok(())
    }

    fn compute(&self) -> Result<(K1ColoringResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((
                K1ColoringResult {
                    colors: Vec::new(),
                    ran_iterations: 0,
                    did_converge: true,
                },
                start.elapsed().as_millis() as u64,
            ));
        }

        let volume = self.max_iterations as usize;
        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
                Tasks::leaf_with_volume("k1coloring".to_string(), volume),
                self.concurrency,
            );
        progress_tracker.begin_subtask_with_volume(volume);

        let fallback = graph_view.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            let node_id: NodeId = node_idx as i64;
            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0)
                .map(|target| target as usize)
                .collect()
        };

        let mut runtime = K1ColoringComputationRuntime::new(node_count, self.max_iterations);
        let result = runtime.compute(node_count, get_neighbors);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        progress_tracker.log_progress(volume);
        progress_tracker.end_subtask();

        Ok((
            K1ColoringResult {
                colors: result.colors,
                ran_iterations: result.ran_iterations,
                did_converge: result.did_converge,
            },
            elapsed_ms,
        ))
    }

    /// Stream mode: yields `(node_id, color_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = K1ColoringRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .colors
            .into_iter()
            .enumerate()
            .map(|(node_id, color_id)| K1ColoringRow {
                node_id: node_id as u64,
                color_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields convergence info + number of distinct colors used.
    pub fn stats(&self) -> Result<K1ColoringStats> {
        let (result, elapsed_ms) = self.compute()?;
        let color_count = result
            .colors
            .iter()
            .copied()
            .collect::<HashSet<u64>>()
            .len();

        Ok(K1ColoringStats {
            did_converge: result.did_converge,
            ran_iterations: result.ran_iterations,
            color_count,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Mutate mode: writes color assignments back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes color assignments to a new graph.
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write not yet implemented".to_string(),
            ),
        )
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Note: memory estimation is deferred.
        Ok(MemoryRange::of_range(0, 1024 * 1024)) // placeholder
    }

    /// Full result: returns the procedure-level K1Coloring result.
    pub fn run(&self) -> Result<K1ColoringResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
