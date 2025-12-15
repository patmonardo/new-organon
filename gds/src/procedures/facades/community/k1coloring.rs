//! K1-Coloring Facade
//!
//! Greedy iterative graph coloring.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.
//! - `max_iterations`: maximum number of coloring/validation iterations (must be >= 1).
//! - `batch_size`: accepted for parity; currently unused.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::k1coloring::{K1ColoringComputationRuntime, K1ColoringResult};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node color assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct K1ColoringRow {
    pub node_id: u64,
    pub color_id: u64,
}

/// Aggregated K1-Coloring stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct K1ColoringStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub color_count: usize,
    pub execution_time_ms: u64,
}

/// K1-Coloring algorithm builder.
#[derive(Clone)]
pub struct K1ColoringBuilder {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_iterations: u64,
    batch_size: usize,
}

impl K1ColoringBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: num_cpus::get().max(1),
            max_iterations: 10,
            batch_size: crate::core::utils::partition::DEFAULT_BATCH_SIZE,
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
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

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
        let color_count = result.colors.iter().copied().collect::<HashSet<u64>>().len();

        Ok(K1ColoringStats {
            did_converge: result.did_converge,
            ran_iterations: result.ran_iterations,
            color_count,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Full result: returns the procedure-level K1Coloring result.
    pub fn run(&self) -> Result<K1ColoringResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
