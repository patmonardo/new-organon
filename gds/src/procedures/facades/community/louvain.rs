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

use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::louvain::{
    LouvainComputationRuntime, LouvainConfig, LouvainResult, LouvainStorageRuntime,
};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node Louvain assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LouvainRow {
    pub node_id: u64,
    pub community_id: u64,
}

/// Aggregated Louvain stats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LouvainStats {
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Louvain algorithm builder.
#[derive(Clone)]
pub struct LouvainBuilder {
    graph_store: Arc<DefaultGraphStore>,
    config: LouvainConfig,
}

impl LouvainBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LouvainConfig {
                concurrency: num_cpus::get().max(1),
            },
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.config.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<(LouvainResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string()))?;

        let storage = LouvainStorageRuntime::new(self.config.concurrency);
        let mut computation = LouvainComputationRuntime::new();

        let result = storage.compute_louvain(&mut computation, graph_view.as_ref());
        Ok((result, start.elapsed().as_millis() as u64))
    }

    /// Stream mode: yields `(node_id, community_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LouvainRow>>> {
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

    /// Stats mode: yields final modularity and ran levels.
    pub fn stats(&self) -> Result<LouvainStats> {
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

    /// Full result: returns the procedure-level Louvain result.
    pub fn run(&self) -> Result<LouvainResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
