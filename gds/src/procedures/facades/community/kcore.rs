//! K-Core Decomposition Facade
//!
//! Finds the k-core values for each node in an undirected graph.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::kcore::{KCoreComputationResult, KCoreComputationRuntime};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node k-core value row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KCoreRow {
    pub node_id: u64,
    pub core_value: i32,
}

/// Aggregated k-core stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KCoreStats {
    pub degeneracy: i32,
    pub execution_time_ms: u64,
}

/// K-Core Decomposition algorithm builder.
#[derive(Clone)]
pub struct KCoreBuilder {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
}

impl KCoreBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: num_cpus::get().max(1),
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<(KCoreComputationResult, u64)> {
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
                KCoreComputationResult {
                    core_values: Vec::new(),
                    degeneracy: 0,
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

        let mut runtime = KCoreComputationRuntime::new();
        let result = runtime.compute(node_count, get_neighbors);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok((result, elapsed_ms))
    }

    /// Stream mode: yields `(node_id, core_value)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KCoreRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .core_values
            .into_iter()
            .enumerate()
            .map(|(node_id, core_value)| KCoreRow {
                node_id: node_id as u64,
                core_value,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields degeneracy and execution time.
    pub fn stats(&self) -> Result<KCoreStats> {
        let (result, elapsed_ms) = self.compute()?;

        Ok(KCoreStats {
            degeneracy: result.degeneracy,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Full result: returns the procedure-level k-core result.
    pub fn run(&self) -> Result<KCoreComputationResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
