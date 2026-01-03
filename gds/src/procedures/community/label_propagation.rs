//! Label Propagation Facade
//!
//! Community detection by iterative label voting.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.
//! - `max_iterations`: max number of propagation iterations (must be >= 1).
//! - `node_weight_property`: optional node weight property (defaults to 1.0).
//! - `seed_property`: optional seed labels property.

use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::label_propagation::{LabelPropComputationRuntime, LabelPropResult};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node label assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LabelPropagationRow {
    pub node_id: u64,
    pub label_id: u64,
}

/// Aggregated label propagation stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LabelPropagationStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Label Propagation algorithm facade.
#[derive(Clone)]
pub struct LabelPropagationFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_iterations: u64,
    node_weight_property: Option<String>,
    seed_property: Option<String>,
    task_registry: Option<TaskRegistry>,
}

impl LabelPropagationFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            max_iterations: 10,
            node_weight_property: None,
            seed_property: None,
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

    pub fn node_weight_property(mut self, property: &str) -> Self {
        self.node_weight_property = Some(property.to_string());
        self
    }

    pub fn seed_property(mut self, property: &str) -> Self {
        self.seed_property = Some(property.to_string());
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
        if let Some(prop) = &self.node_weight_property {
            ConfigValidator::non_empty_string(prop, "node_weight_property")?;
        }
        if let Some(prop) = &self.seed_property {
            ConfigValidator::non_empty_string(prop, "seed_property")?;
        }
        Ok(())
    }

    fn compute(&self) -> Result<(LabelPropResult, u64)> {
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
                LabelPropResult {
                    labels: Vec::new(),
                    did_converge: true,
                    ran_iterations: 0,
                },
                start.elapsed().as_millis() as u64,
            ));
        }

        let mut progress_tracker =
            ProgressTracker::new(Tasks::leaf("label_propagation", self.max_iterations as usize));
        progress_tracker.begin_subtask(self.max_iterations as usize);

        // Node weights
        let weights: Vec<f64> = if let Some(key) = &self.node_weight_property {
            if graph_view.available_node_properties().contains(key) {
                let pv = graph_view
                    .node_properties(key)
                    .expect("property exists by available_node_properties");
                (0..node_count)
                    .map(|i| pv.double_value(i as u64).unwrap_or(1.0))
                    .collect()
            } else {
                vec![1.0; node_count]
            }
        } else {
            vec![1.0; node_count]
        };

        // Seed labels
        let seeds: Option<Vec<u64>> = if let Some(key) = &self.seed_property {
            if graph_view.available_node_properties().contains(key) {
                let pv = graph_view
                    .node_properties(key)
                    .expect("property exists by available_node_properties");
                Some(
                    (0..node_count)
                        .map(|i| pv.long_value(i as u64).unwrap_or(0).max(0) as u64)
                        .collect(),
                )
            } else {
                None
            }
        } else {
            None
        };

        let fallback = graph_view.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<(usize, f64)> {
            let node_id: NodeId = node_idx as i64;
            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0)
                .map(|target| (target as usize, 1.0f64))
                .collect()
        };

        let mut runtime =
            LabelPropComputationRuntime::new(node_count, self.max_iterations).with_weights(weights);
        if let Some(seeds) = seeds {
            runtime = runtime.with_seeds(seeds);
        }

        let result = runtime.compute(node_count, get_neighbors);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        progress_tracker.log_progress(self.max_iterations as usize);
        progress_tracker.end_subtask();

        Ok((
            LabelPropResult {
                labels: result.labels,
                did_converge: result.did_converge,
                ran_iterations: result.ran_iterations,
            },
            elapsed_ms,
        ))
    }

    /// Stream mode: yields `(node_id, label_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LabelPropagationRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .labels
            .into_iter()
            .enumerate()
            .map(|(node_id, label_id)| LabelPropagationRow {
                node_id: node_id as u64,
                label_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields convergence info + community count.
    pub fn stats(&self) -> Result<LabelPropagationStats> {
        let (result, elapsed_ms) = self.compute()?;
        let community_count = result
            .labels
            .iter()
            .copied()
            .collect::<HashSet<u64>>()
            .len();

        Ok(LabelPropagationStats {
            did_converge: result.did_converge,
            ran_iterations: result.ran_iterations,
            community_count,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Mutate mode: writes labels back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes labels to a new graph.
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

    /// Full result: returns the procedure-level Label Propagation result.
    pub fn run(&self) -> Result<LabelPropResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
