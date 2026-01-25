use super::{
    KnnComputationResult, KnnComputationRuntime, KnnNnDescentConfig, KnnNnDescentStats,
};
use super::metrics::{KnnNodePropertySpec, SimilarityComputer, SimilarityMetric};
use crate::core::utils::progress::ProgressTracker;
use crate::ml::core::samplers::RandomWalkSampler;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph_store::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub enum KnnSamplerType {
    #[serde(rename = "UNIFORM")]
    #[default]
    Uniform,
    #[serde(rename = "RANDOMWALK")]
    RandomWalk,
}

pub struct KnnStorageRuntime {
    concurrency: usize,
}

impl KnnStorageRuntime {
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

    pub fn compute_single(
        &self,
        computation: &KnnComputationRuntime,
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
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<Vec<KnnComputationResult>, AlgorithmError> {
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
                progress_tracker,
            )?
            .0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_single_with_stats(
        &self,
        computation: &KnnComputationRuntime,
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
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<(Vec<KnnComputationResult>, KnnNnDescentStats), AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            let values = graph_store
                .node_property_values(node_property)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let similarity =
                <dyn SimilarityComputer>::of_property_values(node_property, values, metric)
                    .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

            let initial_neighbors = Self::build_initial_neighbors(
                graph_store,
                node_count,
                k,
                initial_sampler,
                random_seed.unwrap_or(0),
                None,
                None,
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
                computation.compute_nn_descent(
                    node_count,
                    initial_neighbors,
                    cfg,
                    similarity,
                    None,
                    None,
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

    pub fn compute_multi(
        &self,
        computation: &KnnComputationRuntime,
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
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<Vec<KnnComputationResult>, AlgorithmError> {
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
                progress_tracker,
            )?
            .0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_multi_with_stats(
        &self,
        computation: &KnnComputationRuntime,
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
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<(Vec<KnnComputationResult>, KnnNnDescentStats), AlgorithmError> {
        let node_count = graph_store.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            if node_properties.is_empty() {
                return Err(AlgorithmError::InvalidGraph(
                    "Missing `node_properties`".to_string(),
                ));
            }

            let mut props: Vec<(
                String,
                Arc<dyn NodePropertyValues>,
                SimilarityMetric,
            )> = Vec::with_capacity(node_properties.len());

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

            let initial_neighbors = Self::build_initial_neighbors(
                graph_store,
                node_count,
                k,
                initial_sampler,
                random_seed.unwrap_or(0),
                None,
                None,
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
                computation.compute_nn_descent(
                    node_count,
                    initial_neighbors,
                    cfg,
                    similarity,
                    None,
                    None,
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

    pub(crate) fn build_initial_neighbors(
        graph_store: &impl GraphStore,
        node_count: usize,
        k: usize,
        sampler: KnnSamplerType,
        random_seed: u64,
        source_allowed: Option<Arc<Vec<bool>>>,
        target_allowed: Option<Arc<Vec<bool>>>,
    ) -> Vec<Vec<u64>> {
        let bounded_k = k.min(node_count.saturating_sub(1));
        if bounded_k == 0 {
            return vec![Vec::new(); node_count];
        }

        match sampler {
            KnnSamplerType::Uniform => (0..node_count)
                .map(|source| {
                    if let Some(allowed) = source_allowed.as_ref() {
                        if !allowed[source] {
                            return Vec::new();
                        }
                    }
                    sample_uniform_neighbors(node_count, source as u64, bounded_k, random_seed)
                        .into_iter()
                        .filter(|&t| {
                            if let Some(allowed) = target_allowed.as_ref() {
                                allowed.get(t as usize).copied().unwrap_or(true)
                            } else {
                                true
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
            KnnSamplerType::RandomWalk => {
                let graph = graph_store.get_graph();
                let mut walker = RandomWalkSampler::create(
                    Arc::clone(&graph),
                    |node| graph.degree(node as i64) as f64,
                    // Java uses 3*k walk length multiplier.
                    3 * bounded_k,
                    // Heuristic defaults inspired by the Java implementation.
                    0.4,
                    0.6,
                    random_seed,
                );

                (0..node_count)
                    .map(|source| {
                        if let Some(allowed) = source_allowed.as_ref() {
                            if !allowed[source] {
                                return Vec::new();
                            }
                        }

                        walker.prepare_for_new_node(source as u64);
                        let walk = walker.walk(source as u64);

                        let mut out: Vec<u64> = Vec::with_capacity(bounded_k);
                        let mut seen = std::collections::HashSet::new();
                        for v in walk {
                            if v == source as u64 {
                                continue;
                            }
                            if let Some(allowed) = target_allowed.as_ref() {
                                if !allowed.get(v as usize).copied().unwrap_or(true) {
                                    continue;
                                }
                            }
                            if seen.insert(v) {
                                out.push(v);
                                if out.len() == bounded_k {
                                    break;
                                }
                            }
                        }

                        if out.len() < bounded_k {
                            // Fill from uniform samples if the walk was too short / had duplicates.
                            for t in sample_uniform_neighbors(
                                node_count,
                                source as u64,
                                bounded_k * 2,
                                random_seed.wrapping_add(source as u64),
                            ) {
                                if out.len() == bounded_k {
                                    break;
                                }
                                if seen.insert(t) {
                                    if let Some(allowed) = target_allowed.as_ref() {
                                        if !allowed.get(t as usize).copied().unwrap_or(true) {
                                            continue;
                                        }
                                    }
                                    out.push(t);
                                }
                            }
                        }

                        out
                    })
                    .collect()
            }
        }
    }
}

pub(crate) fn sample_uniform_neighbors(
    node_count: usize,
    source: u64,
    desired: usize,
    seed: u64,
) -> Vec<u64> {
    if node_count <= 1 || desired == 0 {
        return Vec::new();
    }

    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ (source.wrapping_mul(0x9E37_79B9_7F4A_7C15)));
    let mut out = Vec::with_capacity(desired.min(node_count.saturating_sub(1)));
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();

    let max_tries = desired.saturating_mul(50).max(100);
    for _ in 0..max_tries {
        if out.len() == desired {
            break;
        }
        let mut candidate = (rng.next_u32() as usize % node_count) as u64;
        if candidate == source {
            candidate = (candidate + 1) % (node_count as u64);
        }
        if seen.insert(candidate) {
            out.push(candidate);
        }
    }

    out
}
