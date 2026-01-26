//! K-Means storage runtime.
//!
//! Acts as the controller: owns the undirected graph view, materializes the
//! feature vectors from the configured node property, wires progress, enforces
//! termination, and delegates pure computation to `KMeansComputationRuntime`.

use super::spec::{KMeansConfig, KMeansResult};
use super::KMeansComputationRuntime;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use crate::types::ValueType;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct KMeansStorageRuntime;

impl KMeansStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_kmeans(
        &self,
        computation: &mut KMeansComputationRuntime,
        graph_store: &impl GraphStore,
        config: &KMeansConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<KMeansResult, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok(KMeansResult {
                communities: Vec::new(),
                distance_from_center: Vec::new(),
                centers: Vec::new(),
                average_distance_to_centroid: 0.0,
                silhouette: config.compute_silhouette.then_some(Vec::new()),
                average_silhouette: 0.0,
                ran_iterations: 0,
                restarts: 0,
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        if !graph_view
            .available_node_properties()
            .contains(&config.node_property)
        {
            return Err(AlgorithmError::Execution(format!(
                "node_property '{}' not found on graph",
                config.node_property
            )));
        }

        let pv = graph_view
            .node_properties(&config.node_property)
            .ok_or_else(|| {
                AlgorithmError::Execution(format!(
                    "node_property '{}' not available",
                    config.node_property
                ))
            })?;

        let dims = pv.dimension().ok_or_else(|| {
            AlgorithmError::Execution(format!(
                "node_property '{}' has no dimension (no values?)",
                config.node_property
            ))
        })?;

        if !config.seed_centroids.is_empty() {
            if config.seed_centroids.len() != config.k {
                return Err(AlgorithmError::Execution(format!(
                    "seed_centroids must contain exactly k={} centroids, got {}",
                    config.k,
                    config.seed_centroids.len()
                )));
            }
            for (i, c) in config.seed_centroids.iter().enumerate() {
                if c.len() != dims {
                    return Err(AlgorithmError::Execution(format!(
                        "seed_centroids[{}] dimension mismatch: expected {}, got {}",
                        i,
                        dims,
                        c.len()
                    )));
                }
            }
        }

        progress_tracker.begin_subtask_with_volume(node_count);

        let mut points: Vec<Vec<f64>> = Vec::with_capacity(node_count);
        for i in 0..node_count {
            termination_flag.assert_running();

            let arr: Vec<f64> = match pv.value_type() {
                ValueType::DoubleArray => match pv.double_array_value(i as u64) {
                    Ok(value) => value,
                    Err(e) => {
                        progress_tracker.end_subtask_with_failure();
                        return Err(AlgorithmError::Execution(format!(
                            "failed to read node_property '{}' for node {}: {}",
                            config.node_property, i, e
                        )));
                    }
                },
                ValueType::FloatArray => match pv.float_array_value(i as u64) {
                    Ok(value) => value.into_iter().map(|x| x as f64).collect(),
                    Err(e) => {
                        progress_tracker.end_subtask_with_failure();
                        return Err(AlgorithmError::Execution(format!(
                            "failed to read node_property '{}' for node {}: {}",
                            config.node_property, i, e
                        )));
                    }
                },
                other => {
                    progress_tracker.end_subtask_with_failure();
                    return Err(AlgorithmError::Execution(format!(
                        "node_property '{}' must be FLOAT_ARRAY or DOUBLE_ARRAY, got {}",
                        config.node_property,
                        other.name()
                    )));
                }
            };

            if arr.len() != dims {
                progress_tracker.end_subtask_with_failure();
                return Err(AlgorithmError::Execution(format!(
                    "node_property '{}' dimension mismatch at node {}: expected {}, got {}",
                    config.node_property,
                    i,
                    dims,
                    arr.len()
                )));
            }

            points.push(arr);
            progress_tracker.log_progress(1);
        }

        let mut result = computation.compute(&points, config);
        result.node_count = node_count;

        progress_tracker.end_subtask();

        Ok(result)
    }
}
