//! HITS Facade - Bidirectional Pregel implementation

use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory};
use crate::mem::MemoryRange;
use crate::procedures::facades::builder_base::{ConfigValidator, WriteResult};
use crate::procedures::facades::traits::{CentralityScore, Result};
use crate::procedures::hits::computation::run_hits;
use crate::projection::RelationshipType;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Statistics for HITS algorithm
#[derive(Debug, Clone, serde::Serialize)]
pub struct HitsStats {
    pub iterations: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// HITS centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct HitsCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    max_iterations: usize,
    tolerance: f64,
    concurrency: usize,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl HitsCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            max_iterations: 20,
            tolerance: 1e-4,
            concurrency: 4,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Set maximum number of iterations
    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Set convergence tolerance
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    /// Validate the facade configuration.
    ///
    /// # Returns
    /// Ok(()) if configuration is valid, Err otherwise
    ///
    /// # Errors
    /// Returns an error if concurrency is not positive
    pub fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be positive".to_string(),
                ),
            );
        }
        Ok(())
    }

    /// Stream mode: Get HITS scores for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    /// Note: HITS produces two scores per node (hub and authority).
    /// This returns hub scores - use run() for both scores.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let results = graph.hits().stream()?.collect::<Vec<_>>();
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);

        let iter = result
            .hub_scores
            .into_iter()
            .enumerate()
            .map(|(node_id, score)| CentralityScore {
                node_id: node_id as u64,
                score,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns iteration count, convergence status, and execution time.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let stats = graph.hits().stats()?;
    /// println!("Converged in {} iterations", stats.iterations);
    /// ```
    pub fn stats(&self) -> Result<HitsStats> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);
        let elapsed = start.elapsed();

        Ok(HitsStats {
            iterations: result.iterations_ran,
            converged: result.did_converge,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Run the algorithm and return hub and authority scores
    pub fn run(&self) -> Result<(Vec<f64>, Vec<f64>)> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);
        Ok((result.hub_scores, result.authority_scores))
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores hub scores as a node property.
    /// Use run() to get both hub and authority scores.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// let result = graph.hits().mutate("hits_hub")?;
    /// println!("Computed and stored for {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(
        self,
        property_name: &str,
    ) -> Result<crate::procedures::facades::builder_base::MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "HITS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Write mode is not implemented yet for HITS.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "HITS mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for HITS computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::HitsCentralityFacade;
    /// let facade = HitsCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Memory for hub and authority scores (two f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>() * 2;

        // Memory for HITS algorithm structures (iteration tracking, convergence checking)
        let hits_memory = node_count * 8; // Rough estimate for algorithm structures

        // Additional overhead for computation (temporary vectors, etc.)
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory = scores_memory + hits_memory + computation_overhead;
        let total_with_overhead = total_memory + (total_memory / 5); // Add 20% overhead

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}
