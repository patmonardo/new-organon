//! Strongly Connected Components (SCC) Facade
//!
//! Finds SCCs in a directed graph and returns:
//! - per-node component assignment
//! - component count and execution time stats
//!
//! Parameters:
//! - `concurrency`: accepted for Java GDS alignment; currently unused.

use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::scc::{SccComputationRuntime, SccResult, SccStorageRuntime};
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

/// Per-node SCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// Aggregated SCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SccStats {
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// SCC algorithm builder.
#[derive(Clone)]
pub struct SccBuilder {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
}

impl SccBuilder {
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

    fn compute(&self) -> Result<SccResult> {
        self.validate()?;

        let mut computation = SccComputationRuntime::new();
        let storage = SccStorageRuntime::new(self.concurrency);

        let progress_tracker = ProgressTracker::new(Tasks::Leaf(
            "SCC".to_string(),
            self.graph_store.node_count(),
        ));
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_scc(
                &mut computation,
                self.graph_store.as_ref(),
                &progress_tracker,
                &termination_flag,
            )
            .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

        Ok(SccResult::new(
            result.components,
            result.component_count,
            result.computation_time_ms,
        ))
    }

    /// Stream mode: yields `(node_id, component_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = SccRow>>> {
        let result = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| SccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields component count and execution time.
    pub fn stats(&self) -> Result<SccStats> {
        let result = self.compute()?;
        Ok(SccStats {
            component_count: result.component_count,
            execution_time_ms: result.computation_time_ms,
        })
    }

    /// Full result: returns the procedure-level SCC result.
    pub fn run(&self) -> Result<SccResult> {
        self.compute()
    }
}
