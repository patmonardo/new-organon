use crate::algo::algorithms::similarity::build_similarity_relationship_store;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::similarity::filtered_knn::{
    FilteredKnnComputationRuntime, FilteredKnnConfig, FilteredKnnResultRow,
    FilteredKnnStorageRuntime,
};
use crate::algo::similarity::filtered_knn::{
    FilteredKnnMutateResult, FilteredKnnResultBuilder, FilteredKnnStats,
};
use crate::algo::similarity::knn::metrics::{KnnNodePropertySpec, SimilarityMetric};
use crate::algo::similarity::knn::storage::KnnSamplerType;
use crate::algo::similarity::knn::KnnNnDescentStats;
use crate::core::utils::progress::Tasks;
use crate::mem::MemoryRange;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

// Additional imports for progress tracking
use crate::core::utils::progress::TaskProgressTracker;

pub struct FilteredKnnFacade {
    graph_store: Arc<DefaultGraphStore>,
    node_property: String,
    node_properties: Vec<KnnNodePropertySpec>,
    k: usize,
    metric: SimilarityMetric,
    similarity_cutoff: f64,
    concurrency: usize,
    sampled_k: Option<usize>,
    max_iterations: usize,
    initial_sampler: KnnSamplerType,
    random_seed: Option<u64>,
    perturbation_rate: f64,
    random_joins: usize,
    update_threshold: u64,
    source_node_labels: Vec<NodeLabel>,
    target_node_labels: Vec<NodeLabel>,
}

impl FilteredKnnFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>, node_property: impl Into<String>) -> Self {
        Self {
            graph_store,
            node_property: node_property.into(),
            node_properties: Vec::new(),
            k: 10,
            metric: SimilarityMetric::Default,
            similarity_cutoff: 0.0,
            concurrency: 4,
            sampled_k: None,
            max_iterations: 10,
            initial_sampler: KnnSamplerType::default(),
            random_seed: None,
            perturbation_rate: 0.0,
            random_joins: 0,
            update_threshold: 0,
            source_node_labels: Vec::new(),
            target_node_labels: Vec::new(),
        }
    }

    pub fn add_property(
        mut self,
        node_property: impl Into<String>,
        metric: SimilarityMetric,
    ) -> Self {
        self.node_properties
            .push(KnnNodePropertySpec::new(node_property, metric));
        self
    }

    pub fn properties(mut self, node_properties: Vec<KnnNodePropertySpec>) -> Self {
        self.node_properties = node_properties;
        self
    }

    pub fn k(mut self, k: usize) -> Self {
        self.k = k;
        self
    }

    pub fn metric(mut self, metric: SimilarityMetric) -> Self {
        self.metric = metric;
        self
    }

    pub fn similarity_cutoff(mut self, cutoff: f64) -> Self {
        self.similarity_cutoff = cutoff;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn sampled_k(mut self, sampled_k: usize) -> Self {
        self.sampled_k = Some(sampled_k);
        self
    }

    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn initial_sampler(mut self, sampler: KnnSamplerType) -> Self {
        self.initial_sampler = sampler;
        self
    }

    pub fn random_seed(mut self, seed: Option<u64>) -> Self {
        self.random_seed = seed;
        self
    }

    pub fn perturbation_rate(mut self, rate: f64) -> Self {
        self.perturbation_rate = rate;
        self
    }

    pub fn random_joins(mut self, random_joins: usize) -> Self {
        self.random_joins = random_joins;
        self
    }

    pub fn update_threshold(mut self, update_threshold: u64) -> Self {
        self.update_threshold = update_threshold;
        self
    }

    pub fn source_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.source_node_labels = labels;
        self
    }

    pub fn target_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.target_node_labels = labels;
        self
    }

    fn build_config(&self) -> FilteredKnnConfig {
        FilteredKnnConfig {
            node_property: self.node_property.clone(),
            node_properties: self.node_properties.clone(),
            k: self.k,
            sampled_k: self.sampled_k,
            max_iterations: self.max_iterations,
            initial_sampler: self.initial_sampler,
            random_seed: self.random_seed,
            perturbation_rate: self.perturbation_rate,
            random_joins: self.random_joins,
            update_threshold: self.update_threshold,
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            concurrency: self.concurrency,
            source_node_labels: self.source_node_labels.clone(),
            target_node_labels: self.target_node_labels.clone(),
        }
    }

    fn compute_rows_and_nn_stats(self) -> Result<(Vec<FilteredKnnResultRow>, KnnNnDescentStats)> {
        let config = self.build_config();
        let computation = FilteredKnnComputationRuntime::new();
        let storage = FilteredKnnStorageRuntime::new(config.concurrency);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("filteredknn".to_string(), self.graph_store.node_count()),
            config.concurrency,
        );

        let (results, nn_stats) = if config.node_properties.is_empty() {
            storage.compute_single_with_stats(
                &computation,
                self.graph_store.as_ref(),
                &config.node_property,
                config.k,
                config
                    .sampled_k
                    .unwrap_or_else(|| (config.k + 1) / 2)
                    .min(config.k),
                config.max_iterations,
                config.similarity_cutoff,
                config.similarity_metric,
                config.perturbation_rate,
                config.random_joins,
                config.update_threshold,
                config.random_seed,
                config.initial_sampler,
                &config.source_node_labels,
                &config.target_node_labels,
                &mut progress_tracker,
            )?
        } else {
            // Multi-property mode should include the primary property passed to `new(...)`.
            let mut combined: Vec<KnnNodePropertySpec> =
                Vec::with_capacity(config.node_properties.len() + 1);

            let primary = config.node_property.trim();
            if !primary.is_empty() && !config.node_properties.iter().any(|p| p.name == primary) {
                combined.push(KnnNodePropertySpec::new(primary, config.similarity_metric));
            }
            combined.extend(config.node_properties.iter().cloned());

            storage.compute_multi_with_stats(
                &computation,
                self.graph_store.as_ref(),
                &combined,
                config.k,
                config
                    .sampled_k
                    .unwrap_or_else(|| (config.k + 1) / 2)
                    .min(config.k),
                config.max_iterations,
                config.similarity_cutoff,
                config.perturbation_rate,
                config.random_joins,
                config.update_threshold,
                config.random_seed,
                config.initial_sampler,
                &config.source_node_labels,
                &config.target_node_labels,
                &mut progress_tracker,
            )?
        };

        Ok((
            results
                .into_iter()
                .map(FilteredKnnResultRow::from)
                .collect(),
            nn_stats,
        ))
    }

    fn compute_rows(self) -> Result<Vec<FilteredKnnResultRow>> {
        Ok(self.compute_rows_and_nn_stats()?.0)
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = FilteredKnnResultRow>>> {
        let rows = self.compute_rows()?;
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(self) -> Result<FilteredKnnStats> {
        let (rows, nn_stats) = self.compute_rows_and_nn_stats()?;
        Ok(FilteredKnnResultBuilder::new(&rows, &nn_stats).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<FilteredKnnMutateResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let graph_store = Arc::clone(&self.graph_store);
        let start = std::time::Instant::now();
        let (rows, nn_stats) = self.compute_rows_and_nn_stats()?;
        let builder = FilteredKnnResultBuilder::new(&rows, &nn_stats);
        let stats = builder.stats();

        let pairs: Vec<(u64, u64, f64)> = rows
            .iter()
            .map(|r| (r.source, r.target, r.similarity))
            .collect();

        let updated_store =
            build_similarity_relationship_store(graph_store.as_ref(), property_name, &pairs)?;

        let relationships_updated = graph_store.relationship_count() as u64;
        let summary =
            builder.mutation_summary(property_name, relationships_updated, start.elapsed());

        Ok(FilteredKnnMutateResult {
            summary,
            stats,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            res.summary.property_name,
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        let property_count = if self.node_properties.is_empty() {
            1
        } else {
            self.node_properties.len() + 1
        };

        // Filtering by labels usually reduces work, but memory estimate remains conservative.
        let pair_count = node_count.saturating_mul(self.k);

        let results_memory = pair_count * 32;
        let per_node_scratch = node_count * 24;
        let per_property_scratch = property_count * node_count * 8;

        // Label filter sets/vectors
        let label_filter_memory =
            (self.source_node_labels.len() + self.target_node_labels.len()) * 64;

        let total = results_memory + per_node_scratch + per_property_scratch + label_filter_memory;
        let overhead = total / 5;
        MemoryRange::of_range(total, total + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::prelude::GraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn multi_property_mode_includes_primary_property() {
        let config = RandomGraphConfig {
            node_count: 12,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let err = FilteredKnnFacade::new(Arc::clone(&store), "does_not_exist")
            .add_property("random_score", SimilarityMetric::Default)
            .k(1)
            .stream()
            .err();

        assert!(err.is_some());
    }

    #[test]
    fn stats_exposes_nn_descent_metadata() {
        let config = RandomGraphConfig {
            node_count: 25,
            seed: Some(7),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let max_iterations = 3;
        let stats = FilteredKnnFacade::new(Arc::clone(&store), "random_score")
            .k(5)
            .max_iterations(max_iterations)
            .stats()
            .unwrap();

        assert!(stats.ran_iterations <= max_iterations as u64);
        // Non-negative by construction (u64); ensure it is wired through.
        let _ = stats.node_pairs_considered;
    }

    #[test]
    fn mutate_adds_relationship_property() {
        let config = RandomGraphConfig {
            node_count: 12,
            seed: Some(21),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let result = FilteredKnnFacade::new(Arc::clone(&store), "random_score")
            .k(3)
            .similarity_cutoff(0.0)
            .concurrency(1)
            .mutate("sim_score")
            .unwrap();

        let rel_type = result
            .updated_store
            .relationship_types()
            .into_iter()
            .next()
            .expect("expected at least one relationship type");
        let _ = result
            .updated_store
            .relationship_property_values(&rel_type, "sim_score")
            .unwrap();
    }
}
