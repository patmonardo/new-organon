//! Weakly Connected Components (WCC) Facade
//!
//! Finds connected components in a graph under *undirected* semantics.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; current runtime is single-threaded.

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::wcc::{WccComputationRuntime, WccStorageRuntime};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node WCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// Aggregated WCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WccStats {
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// WCC algorithm builder.
#[derive(Clone)]
pub struct WccBuilder {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
}

impl WccBuilder {
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

    fn compute(&self) -> Result<(crate::procedures::wcc::WccResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let storage = WccStorageRuntime::new(self.concurrency);
        let mut computation = WccComputationRuntime::new();
        let result = storage.compute_wcc(&mut computation, graph_view.as_ref());

        Ok((
            crate::procedures::wcc::WccResult {
                components: result.components,
                component_count: result.component_count,
            },
            start.elapsed().as_millis() as u64,
        ))
    }

    /// Stream mode: yields `(node_id, component_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = WccRow>>> {
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

    /// Stats mode: yields component count and execution time.
    pub fn stats(&self) -> Result<WccStats> {
        let (result, elapsed) = self.compute()?;
        Ok(WccStats {
            component_count: result.component_count,
            execution_time_ms: elapsed,
        })
    }

    /// Full result: returns the procedure-level WCC result.
    pub fn run(&self) -> Result<crate::procedures::wcc::WccResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
