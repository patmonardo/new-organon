use crate::mem::MemoryRange;
use crate::procedures::traits::Result;
pub use crate::algo::similarity::knn::{KnnNodePropertySpec, SimilarityMetric};
use crate::algo::similarity::knn::{KnnConfig, KnnResultRow};
use crate::core::utils::progress::Tasks;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnStats {
    #[serde(rename = "nodesCompared")]
    pub nodes_compared: u64,
    #[serde(rename = "similarityPairs")]
    pub similarity_pairs: u64,
    #[serde(rename = "similarityDistribution")]
    pub similarity_distribution: HashMap<String, f64>,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

pub struct KnnBuilder {
    graph_store: Arc<DefaultGraphStore>,
    node_property: String,
    node_properties: Vec<KnnNodePropertySpec>,
    k: usize,
    metric: SimilarityMetric,
    similarity_cutoff: f64,
    concurrency: usize,
}

impl KnnBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>, node_property: impl Into<String>) -> Self {
        Self {
            graph_store,
            node_property: node_property.into(),
            node_properties: Vec::new(),
            k: 10,
            metric: SimilarityMetric::Default,
            similarity_cutoff: 0.0,
            concurrency: 4,
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

    fn build_config(&self) -> KnnConfig {
        KnnConfig {
            node_property: self.node_property.clone(),
            node_properties: self.node_properties.clone(),
            k: self.k,
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            concurrency: self.concurrency,
        }
    }

    fn compute_rows(self) -> Result<Vec<KnnResultRow>> {
        let config = self.build_config();
        let computation = crate::algo::similarity::knn::KnnComputationRuntime::new();
        let storage =
            crate::algo::similarity::knn::KnnStorageRuntime::new(config.concurrency);

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("knn".to_string(), self.graph_store.node_count()),
            config.concurrency,
        );

        let results = if config.node_properties.is_empty() {
            storage.compute_single(
                &computation,
                self.graph_store.as_ref(),
                &config.node_property,
                config.k,
                config.similarity_cutoff,
                config.similarity_metric,
                &mut progress_tracker,
            )?
        } else {
            // Multi-property mode should include the primary property passed to `new(...)`.
            // The `node_properties` list represents additional per-property metric specs.
            let mut combined: Vec<KnnNodePropertySpec> = Vec::with_capacity(
                config.node_properties.len() + 1,
            );

            let primary = config.node_property.trim();
            if !primary.is_empty()
                && !config.node_properties.iter().any(|p| p.name == primary)
            {
                combined.push(KnnNodePropertySpec::new(
                    primary,
                    config.similarity_metric,
                ));
            }
            combined.extend(config.node_properties.iter().cloned());

            storage.compute_multi(
                &computation,
                self.graph_store.as_ref(),
                &combined,
                config.k,
                config.similarity_cutoff,
                &mut progress_tracker,
            )?
        };

        Ok(results.into_iter().map(KnnResultRow::from).collect())
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = KnnResultRow>>> {
        let rows = self.compute_rows()?;
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(self) -> Result<KnnStats> {
        let rows = self.compute_rows()?;

        let mut sources = HashSet::new();
        let tuples: Vec<(u64, u64, f64)> = rows
            .iter()
            .map(|r| {
                sources.insert(r.source);
                (r.source, r.target, r.similarity)
            })
            .collect();

        let stats = crate::algo::core::result::similarity::similarity_stats(
            || tuples.into_iter(),
            true,
        );

        Ok(KnnStats {
            nodes_compared: sources.len() as u64,
            similarity_pairs: rows.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        })
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        let property_count = if self.node_properties.is_empty() {
            1
        } else {
            // multi-property mode includes primary property
            self.node_properties.len() + 1
        };

        let pair_count = node_count.saturating_mul(self.k);

        // Rough accounting:
        // - results: (source, target, similarity) + Vec overhead
        // - per-node scratch (indices, counters)
        // - per-property scratch
        let results_memory = pair_count * 32;
        let per_node_scratch = node_count * 24;
        let per_property_scratch = property_count * node_count * 8;

        let total = results_memory + per_node_scratch + per_property_scratch;
        let overhead = total / 5;
        MemoryRange::of_range(total, total + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn multi_property_mode_includes_primary_property() {
        let config = RandomGraphConfig {
            node_count: 12,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        // If the primary property were (incorrectly) ignored in multi-property mode,
        // this would succeed because `random_score` exists.
        // We expect it to fail because the primary property is missing.
        let err = KnnBuilder::new(Arc::clone(&store), "does_not_exist")
            .add_property("random_score", SimilarityMetric::Default)
            .k(1)
            .stream()
            .err();

        assert!(err.is_some());
    }
}
