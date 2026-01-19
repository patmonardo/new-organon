//! Harmonic Centrality Facade
//!
//! **What is it?**: A closeness variant that sums reciprocal distances.
//! **Why care?**: Highlights nodes that are, on average, close to many others,
//! including in disconnected graphs (unreachable pairs contribute 0).
//! **Complexity**: O(V*(V+E)) in the worst case (all-pairs BFS).
//!
//! This implementation follows the Neo4j GDS behavior:
//! - Uses MSBFS-style aggregated neighbor processing
//! - Accumulates into the *reached node* per depth
//! - Normalizes by `(nodeCount - 1)`

use crate::algo::harmonic::{HarmonicComputationRuntime, HarmonicStorageRuntime};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::projection::orientation::Orientation;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Statistics about harmonic centrality.
#[derive(Debug, Clone, serde::Serialize)]
pub struct HarmonicCentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub isolated_nodes: u64,
    pub execution_time_ms: u64,
}

/// Mutation result for harmonic centrality, including the updated graph store.
#[derive(Debug, Clone)]
pub struct HarmonicCentralityMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

/// Harmonic centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct HarmonicCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    direction: String,
    concurrency: usize,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl HarmonicCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            direction: "both".to_string(),
            concurrency: 4,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
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

    fn orientation(&self) -> Orientation {
        match self.direction.as_str() {
            "incoming" => Orientation::Reverse,
            "outgoing" => Orientation::Natural,
            _ => Orientation::Undirected,
        }
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

    fn compute_scores(&self) -> Result<(Vec<f64>, std::time::Duration)> {
        let start = Instant::now();

        let storage = HarmonicStorageRuntime::with_orientation(
            self.graph_store.as_ref(),
            self.orientation(),
        )?;

        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("harmonic".to_string(), node_count)
                .base()
                .clone(),
            crate::concurrency::Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let computation = HarmonicComputationRuntime::new(node_count);
        let termination = TerminationFlag::default();

        let tracker = Arc::new(Mutex::new(progress_tracker));
        let on_sources_done = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |n: usize| {
                tracker.lock().unwrap().log_progress(n);
            })
        };

        let scores = match storage.compute_parallel(
            &computation,
            self.concurrency,
            &termination,
            on_sources_done,
        ) {
            Ok(scores) => scores,
            Err(e) => {
                tracker.lock().unwrap().end_subtask_with_failure();
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                        "Harmonic terminated: {e}"
                    )),
                );
            }
        };

        tracker.lock().unwrap().end_subtask();

        Ok((scores, start.elapsed()))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        let (scores, _elapsed) = self.compute_scores()?;
        let iter = scores
            .into_iter()
            .enumerate()
            .map(|(node_id, score)| CentralityScore {
                node_id: node_id as u64,
                score,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<HarmonicCentralityStats> {
        self.validate()?;
        let (scores, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(HarmonicCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                isolated_nodes: 0,
                execution_time_ms: elapsed.as_millis() as u64,
            });
        }

        let isolated_nodes = scores.iter().filter(|v| **v == 0.0).count() as u64;

        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / scores.len() as f64;
        let stddev = var.sqrt();

        let percentile = |p: f64| -> f64 {
            let idx =
                ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
            sorted[idx]
        };

        Ok(HarmonicCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            isolated_nodes,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: compute scores, write them to a new graph store, and return it.
    pub fn mutate(self, property_name: &str) -> Result<HarmonicCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        let nodes_updated = scores.len() as u64;

        // Build property values
        let node_count = scores.len();
        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn crate::types::properties::node::NodePropertyValues> = Arc::new(values);

        // Clone store, add property, and return updated store
        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Harmonic mutate failed to add property: {e}"
                ))
            })?;

        let execution_time = start_time.elapsed();
        let summary = MutationResult::new(nodes_updated, property_name.to_string(), execution_time);

        Ok(HarmonicCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for harmonic.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "HarmonicCentrality mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for harmonic centrality computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::HarmonicCentralityFacade;
    /// let facade = HarmonicCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Memory for harmonic centrality scores (one f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Memory for MSBFS processing
        let msbfs_memory = node_count * 8; // Rough estimate for MSBFS structures

        // Additional overhead for computation (temporary vectors, etc.)
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory = scores_memory + msbfs_memory + computation_overhead;
        let total_with_overhead = total_memory + (total_memory / 5); // Add 20% overhead

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(11),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = HarmonicCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = HarmonicCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = HarmonicCentralityFacade::new(store());
        assert!(facade.clone().mutate("").is_err());
        let result = facade.mutate("harmonic");
        assert!(result.is_ok());
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "harmonic");
        assert!(mutation_result.updated_store.has_node_property("harmonic"));
    }
}
