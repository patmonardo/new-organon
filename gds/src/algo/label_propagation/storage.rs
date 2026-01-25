//! Label Propagation storage runtime

use super::computation::{
    LabelPropComputationRuntime, LabelPropResult as LabelPropComputationResult,
};
use super::spec::LabelPropConfig;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::default_value::LONG_DEFAULT_FALLBACK;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use crate::types::graph::Graph;

#[derive(Clone)]
pub struct LabelPropStorageRuntime {
    graph: Arc<dyn Graph>,
}

impl LabelPropStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self { graph })
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    /// Controller entrypoint: owns graph access, materializes weights/seeds, wires progress,
    /// enforces termination, and delegates to the pure computation runtime.
    pub fn compute_label_propagation(
        &self,
        computation: LabelPropComputationRuntime,
        config: &LabelPropConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LabelPropComputationResult, AlgorithmError> {
        let node_count = self.graph.node_count() as usize;
        if node_count == 0 {
            return Ok(LabelPropComputationResult {
                labels: Vec::new(),
                did_converge: true,
                ran_iterations: 0,
            });
        }

        termination_flag.assert_running();

        let weights: Vec<f64> = match &config.node_weight_property {
            Some(key) if self.graph.available_node_properties().contains(key) => {
                let pv = self.graph.node_properties(key).ok_or_else(|| {
                    AlgorithmError::Execution(format!(
                        "node_weight_property '{}' not available",
                        key
                    ))
                })?;

                (0..node_count)
                    .map(|i| {
                        termination_flag.assert_running();
                        pv.double_value(i as u64).unwrap_or(1.0)
                    })
                    .collect()
            }
            _ => vec![1.0; node_count],
        };

        // Seed labels (Java InitStep parity):
        // - If a seedProperty exists and has a value: use it.
        // - Otherwise: label = maxLabelId + originalNodeId + 1.
        // This avoids collisions with node IDs while keeping determinism.
        progress_tracker
            .begin_subtask_with_volume(node_count.saturating_add(config.max_iterations as usize));

        let seed_pv = config.seed_property.as_ref().and_then(|key| {
            if self.graph.available_node_properties().contains(key) {
                self.graph.node_properties(key)
            } else {
                None
            }
        });

        let max_label_id: i64 = seed_pv
            .as_deref()
            .and_then(|pv| pv.get_max_long_property_value())
            .unwrap_or(-1);

        let mut seeds: Vec<u64> = Vec::with_capacity(node_count);
        for i in 0..node_count {
            termination_flag.assert_running();

            let node_id = i as i64;
            let original = self.graph.to_original_node_id(node_id).unwrap_or(node_id);

            let label = match seed_pv.as_deref() {
                Some(pv) if pv.has_value(i as u64) => {
                    pv.long_value(i as u64).unwrap_or(LONG_DEFAULT_FALLBACK) as u64
                }
                _ => (max_label_id + original + 1) as u64,
            };

            seeds.push(label);
            progress_tracker.log_progress(1);
        }

        let fallback = self.graph.default_property_value();
        let graph = Arc::clone(&self.graph);
        let neighbors = move |node_idx: usize| -> Vec<(usize, f64)> {
            graph
                .stream_relationships_weighted(node_idx as i64, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .filter(|(target, _w)| *target >= 0)
                .map(|(target, w)| (target as usize, w))
                .collect()
        };

        let result = computation
            .with_weights(weights)
            .with_seeds(seeds)
            .compute(node_count as u64, neighbors);
        progress_tracker.log_progress(result.ran_iterations as usize);
        progress_tracker.end_subtask();

        Ok(result)
    }
}
