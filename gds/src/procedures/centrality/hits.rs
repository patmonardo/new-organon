//! HITS Facade - Bidirectional Pregel implementation

use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::hits::{
    computation::HitsComputationRuntime, HitsCentralityMutateResult, HitsCentralityMutationSummary,
    HitsCentralityStats, HitsConfig, HitsResult, HitsResultBuilder, HitsStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::algo::algorithms::{CentralityScore, Result};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// HITS centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct HitsCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: HitsConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl HitsCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: HitsConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: HitsConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: HitsConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: HitsConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Set maximum number of iterations
    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    /// Set convergence tolerance
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.config.tolerance = tolerance;
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
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
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<HitsResult> {
        self.validate()?;
        let start = Instant::now();

        let storage = HitsStorageRuntime::with_default_projection(self.graph_store.as_ref())?;
        let computation = HitsComputationRuntime::new(self.config.tolerance);

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("hits".to_string(), self.config.max_iterations)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );

        let result = storage.run(
            &computation,
            self.config.max_iterations,
            self.config.concurrency,
            &mut progress_tracker,
        );

        Ok(HitsResult {
            hub_scores: result.hub_scores,
            authority_scores: result.authority_scores,
            iterations: result.iterations_ran,
            converged: result.did_converge,
            execution_time: start.elapsed(),
        })
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
        let result = self.compute()?;
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
    pub fn stats(&self) -> Result<HitsCentralityStats> {
        let result = self.compute()?;
        Ok(HitsResultBuilder::new(result).stats())
    }

    /// Run the algorithm and return hub and authority scores
    pub fn run(&self) -> Result<(Vec<f64>, Vec<f64>)> {
        let result = self.compute()?;
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
    /// println!("Computed and stored for {} nodes", result.summary.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<HitsCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let hub_scores = result.hub_scores.clone();
        let nodes_updated = hub_scores.len() as u64;
        let node_count = hub_scores.len();
        let backend = VecDouble::from(hub_scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("HITS mutate failed to add property: {e}"))
            })?;

        let summary = HitsCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(HitsCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: Compute scores and return write summary.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let nodes_written = result.hub_scores.len() as u64;

        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            result.execution_time,
        ))
    }

    /// Estimate memory requirements for HITS computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::HitsCentralityFacade;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::PropertyValues;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(11),
            node_count: 6,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn mutate_adds_hub_property() {
        let facade = HitsCentralityFacade::new(store());
        let result = facade.mutate("hits_hub").unwrap();

        let values = result
            .updated_store
            .node_property_values("hits_hub")
            .unwrap();

        assert_eq!(values.element_count(), 6);
    }
}
