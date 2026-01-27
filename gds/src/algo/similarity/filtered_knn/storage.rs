use super::{FilteredKnnComputationResult, FilteredKnnComputationRuntime};
use crate::algo::similarity::knn::KnnNnDescentConfig;
use crate::algo::similarity::knn::KnnNnDescentStats;
use crate::algo::similarity::knn::metrics::{
    KnnNodePropertySpec, SimilarityComputer, SimilarityMetric,
};
use crate::algo::similarity::knn::{KnnSamplerType, KnnStorageRuntime};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::graph_store::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FilteredKnnStorageRuntime {
    concurrency: usize,
}

impl FilteredKnnStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    fn with_thread_pool<T>(&self, f: impl FnOnce() -> T + Send) -> T
    where
        T: Send,
    {
        let thread_count = self.concurrency.max(1);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build();

        match pool {
            Ok(pool) => pool.install(f),
            Err(_) => f(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_single(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_property: &str,
        k: usize,
        sampled_k: usize,
        max_iterations: usize,
        similarity_cutoff: f64,
        metric: SimilarityMetric,
        perturbation_rate: f64,
        random_joins: usize,
        update_threshold: u64,
        random_seed: Option<u64>,
        initial_sampler: KnnSamplerType,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<Vec<FilteredKnnComputationResult>, AlgorithmError> {
        Ok(self
            .compute_single_with_stats(
                computation,
                graph_store,
                node_property,
                k,
                sampled_k,
                max_iterations,
                similarity_cutoff,
                metric,
                perturbation_rate,
                random_joins,
                update_threshold,
                random_seed,
                initial_sampler,
                source_node_labels,
                target_node_labels,
                progress_tracker,
            )?
            .0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_single_with_stats(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_property: &str,
        k: usize,
        sampled_k: usize,
        max_iterations: usize,
        similarity_cutoff: f64,
        metric: SimilarityMetric,
        perturbation_rate: f64,
        random_joins: usize,
        update_threshold: u64,
        random_seed: Option<u64>,
        initial_sampler: KnnSamplerType,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<(Vec<FilteredKnnComputationResult>, KnnNnDescentStats), AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            let values = graph_store
                .node_property_values(node_property)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let similarity =
                <dyn SimilarityComputer>::of_property_values(node_property, values, metric)
                    .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let (source_allowed, target_allowed) =
                Self::build_filters(graph_store, source_node_labels, target_node_labels)?;

            let source_allowed = source_allowed.map(Arc::new);
            let target_allowed = target_allowed.map(Arc::new);

            let initial_neighbors = KnnStorageRuntime::build_initial_neighbors(
                graph_store,
                node_count,
                k,
                initial_sampler,
                random_seed.unwrap_or(0),
                source_allowed.clone(),
                target_allowed.clone(),
            );

            let cfg = KnnNnDescentConfig {
                k,
                sampled_k,
                max_iterations,
                similarity_cutoff,
                perturbation_rate,
                random_joins,
                update_threshold,
                random_seed: random_seed.unwrap_or(0),
            };

            Ok(self.with_thread_pool(|| {
                computation.compute_nn_descent_with_stats(
                    node_count,
                    initial_neighbors,
                    cfg,
                    similarity,
                    source_allowed,
                    target_allowed,
                )
            }))
        })();

        match result {
            Ok(value) => {
                progress_tracker.log_progress(node_count);
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_multi(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_properties: &[KnnNodePropertySpec],
        k: usize,
        sampled_k: usize,
        max_iterations: usize,
        similarity_cutoff: f64,
        perturbation_rate: f64,
        random_joins: usize,
        update_threshold: u64,
        random_seed: Option<u64>,
        initial_sampler: KnnSamplerType,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<Vec<FilteredKnnComputationResult>, AlgorithmError> {
        Ok(self
            .compute_multi_with_stats(
                computation,
                graph_store,
                node_properties,
                k,
                sampled_k,
                max_iterations,
                similarity_cutoff,
                perturbation_rate,
                random_joins,
                update_threshold,
                random_seed,
                initial_sampler,
                source_node_labels,
                target_node_labels,
                progress_tracker,
            )?
            .0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_multi_with_stats(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_properties: &[KnnNodePropertySpec],
        k: usize,
        sampled_k: usize,
        max_iterations: usize,
        similarity_cutoff: f64,
        perturbation_rate: f64,
        random_joins: usize,
        update_threshold: u64,
        random_seed: Option<u64>,
        initial_sampler: KnnSamplerType,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<(Vec<FilteredKnnComputationResult>, KnnNnDescentStats), AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            if node_properties.is_empty() {
                return Err(AlgorithmError::InvalidGraph(
                    "Missing `node_properties`".to_string(),
                ));
            }

            let mut props: Vec<(String, Arc<dyn NodePropertyValues>, SimilarityMetric)> =
                Vec::with_capacity(node_properties.len());

            for spec in node_properties {
                let name = spec.name.trim();
                if name.is_empty() {
                    return Err(AlgorithmError::InvalidGraph(
                        "`node_properties` contains an empty property name".to_string(),
                    ));
                }
                let values = graph_store
                    .node_property_values(name)
                    .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;
                props.push((name.to_string(), values, spec.metric));
            }

            let similarity = <dyn SimilarityComputer>::of_properties(props)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let (source_allowed, target_allowed) =
                Self::build_filters(graph_store, source_node_labels, target_node_labels)?;

            let source_allowed = source_allowed.map(Arc::new);
            let target_allowed = target_allowed.map(Arc::new);

            let initial_neighbors = KnnStorageRuntime::build_initial_neighbors(
                graph_store,
                node_count,
                k,
                initial_sampler,
                random_seed.unwrap_or(0),
                source_allowed.clone(),
                target_allowed.clone(),
            );

            let cfg = KnnNnDescentConfig {
                k,
                sampled_k,
                max_iterations,
                similarity_cutoff,
                perturbation_rate,
                random_joins,
                update_threshold,
                random_seed: random_seed.unwrap_or(0),
            };

            Ok(self.with_thread_pool(|| {
                computation.compute_nn_descent_with_stats(
                    node_count,
                    initial_neighbors,
                    cfg,
                    similarity,
                    source_allowed,
                    target_allowed,
                )
            }))
        })();

        match result {
            Ok(value) => {
                progress_tracker.log_progress(node_count);
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn build_filters(
        graph_store: &impl GraphStore,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
    ) -> Result<(Option<Vec<bool>>, Option<Vec<bool>>), AlgorithmError> {
        let source = Self::build_label_filter(graph_store, source_node_labels)?;
        let target = Self::build_label_filter(graph_store, target_node_labels)?;
        Ok((source, target))
    }

    fn build_label_filter(
        graph_store: &impl GraphStore,
        labels: &[NodeLabel],
    ) -> Result<Option<Vec<bool>>, AlgorithmError> {
        if labels.is_empty() || labels.iter().any(|l| l.is_all_nodes()) {
            return Ok(None);
        }

        for label in labels {
            if label.is_all_nodes() {
                continue;
            }
            if !graph_store.has_node_label(label) {
                return Err(AlgorithmError::InvalidGraph(format!(
                    "Unknown node label `{}`",
                    label.name()
                )));
            }
        }

        let node_count = graph_store.node_count();
        let id_map = graph_store.nodes();
        let label_set: HashSet<NodeLabel> = labels.iter().cloned().collect();

        let mut allowed = vec![false; node_count];
        for (mapped, allowed_slot) in allowed.iter_mut().enumerate().take(node_count) {
            let mapped_i64 = mapped as i64;
            let mut ok = false;
            for label in &label_set {
                if id_map.has_label(mapped_i64, label) {
                    ok = true;
                    break;
                }
            }
            *allowed_slot = ok;
        }

        Ok(Some(allowed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn compute_single_respects_source_and_target_label_filters() {
        let config = RandomGraphConfig {
            node_count: 24,
            node_labels: vec!["A".to_string(), "B".to_string()],
            seed: Some(7),
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();

        let node_count = store.node_count();
        let id_map = store.nodes();
        let a = NodeLabel::of("A");
        let b = NodeLabel::of("B");

        let a_count = (0..node_count)
            .filter(|&i| id_map.has_label(i as i64, &a))
            .count();
        let b_count = (0..node_count)
            .filter(|&i| id_map.has_label(i as i64, &b))
            .count();
        assert!(a_count > 0, "seeded graph should contain label A");
        assert!(b_count > 0, "seeded graph should contain label B");

        let runtime = FilteredKnnStorageRuntime::new(4);
        let computation = FilteredKnnComputationRuntime::new();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("filteredknn".to_string(), node_count),
            4,
        );

        let rows = runtime
            .compute_single(
                &computation,
                &store,
                "random_score",
                3,
                2,
                2,
                0.0,
                SimilarityMetric::Default,
                0.0,
                0,
                0,
                Some(7),
                KnnSamplerType::Uniform,
                &[a.clone()],
                &[b.clone()],
                &mut progress_tracker,
            )
            .unwrap();

        for row in &rows {
            assert!(
                id_map.has_label(row.source as i64, &a),
                "source {} must have label A",
                row.source
            );
            assert!(
                id_map.has_label(row.target as i64, &b),
                "target {} must have label B",
                row.target
            );
        }
    }
}
