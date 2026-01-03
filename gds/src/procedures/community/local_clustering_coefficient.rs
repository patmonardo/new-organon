//! Local Clustering Coefficient Facade
//!
//! Computes local clustering coefficient for each node:
//! $$C(v) = \frac{2 t(v)}{d(v)(d(v)-1)}$$
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: reserved for future parallel implementation
//! - `max_degree`: passed through to TriangleCount (performance / approximation)
//! - `seed_property`: optional triangle-count property name (not yet wired; we compute triangles)

use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::community::triangle_count::TriangleCountFacade;
use crate::procedures::traits::Result;
use crate::algo::local_clustering_coefficient::LocalClusteringCoefficientComputationRuntime;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Per-node local clustering coefficient row.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct LocalClusteringCoefficientRow {
    pub node_id: u64,
    pub local_clustering_coefficient: f64,
}

/// Aggregated local clustering coefficient statistics.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct LocalClusteringCoefficientStats {
    pub average_clustering_coefficient: f64,
    pub execution_time_ms: u64,
}

/// Local Clustering Coefficient algorithm facade.
#[derive(Clone)]
pub struct LocalClusteringCoefficientFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_degree: u64,
    seed_property: Option<String>,
    task_registry: Option<TaskRegistry>,
}

impl LocalClusteringCoefficientFacade {
    /// Create a new facade bound to a live graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            max_degree: u64::MAX,
            seed_property: None,
            task_registry: None,
        }
    }

    /// Concurrency hint (reserved for future parallel implementation).
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Skip nodes with degree > max_degree in triangle counting.
    pub fn max_degree(mut self, max_degree: u64) -> Self {
        self.max_degree = max_degree;
        self
    }

    /// Optional triangle-count seed property.
    ///
    /// For now, this facade still computes triangle counts directly.
    pub fn seed_property(mut self, seed_property: &str) -> Self {
        self.seed_property = Some(seed_property.to_string());
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        if let Some(prop) = &self.seed_property {
            ConfigValidator::non_empty_string(prop, "seed_property")?;
        }
        Ok(())
    }

    fn compute(&self) -> Result<(Vec<f64>, f64, Duration)> {
        self.validate()?;
        let start = Instant::now();

        // Triangle counts (seed_property currently ignored; we always compute).
        let triangle_result = TriangleCountFacade::new(Arc::clone(&self.graph_store))
            .concurrency(self.concurrency)
            .max_degree(self.max_degree)
            .run()?;

        let rel_graph = self.graph_store.get_graph();
        let node_count = rel_graph.node_count();

        let mut progress_tracker = ProgressTracker::with_concurrency(
            Tasks::leaf("local_clustering_coefficient", node_count),
            self.concurrency,
        );
        progress_tracker.begin_subtask(node_count);

        let mut degrees: Vec<i32> = Vec::with_capacity(node_count);
        for node_id in 0..node_count {
            degrees.push(rel_graph.degree(node_id as i64) as i32);
        }

        let mut runtime = LocalClusteringCoefficientComputationRuntime::new(node_count);
        runtime.compute(&triangle_result.local_triangles, &degrees);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok((
            runtime.local_clustering_coefficients.clone(),
            runtime.average_clustering_coefficient,
            start.elapsed(),
        ))
    }

    /// Stream mode: yields `(node_id, coefficient)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LocalClusteringCoefficientRow>>> {
        let (coeffs, _avg, _elapsed) = self.compute()?;
        let iter = coeffs
            .into_iter()
            .enumerate()
            .map(
                |(node_id, local_clustering_coefficient)| LocalClusteringCoefficientRow {
                    node_id: node_id as u64,
                    local_clustering_coefficient,
                },
            );
        Ok(Box::new(iter))
    }

    /// Stats mode: yields average clustering coefficient.
    pub fn stats(&self) -> Result<LocalClusteringCoefficientStats> {
        let (_coeffs, avg, elapsed) = self.compute()?;
        Ok(LocalClusteringCoefficientStats {
            average_clustering_coefficient: avg,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: writes clustering coefficients back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes clustering coefficients to a new graph.
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
}
