use crate::procedures::traits::Result;
use crate::algo::similarity::filteredknn::{
    FilteredKnnComputationRuntime, FilteredKnnConfig, FilteredKnnResultRow,
    FilteredKnnStorageRuntime,
};
use crate::algo::similarity::knn::metrics::{KnnNodePropertySpec, SimilarityMetric};
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredKnnStats {
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

pub struct FilteredKnnBuilder {
    graph_store: Arc<DefaultGraphStore>,
    node_property: String,
    node_properties: Vec<KnnNodePropertySpec>,
    k: usize,
    metric: SimilarityMetric,
    similarity_cutoff: f64,
    concurrency: usize,
    source_node_labels: Vec<NodeLabel>,
    target_node_labels: Vec<NodeLabel>,
}

impl FilteredKnnBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>, node_property: impl Into<String>) -> Self {
        Self {
            graph_store,
            node_property: node_property.into(),
            node_properties: Vec::new(),
            k: 10,
            metric: SimilarityMetric::Default,
            similarity_cutoff: 0.0,
            concurrency: 4,
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
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            concurrency: self.concurrency,
            source_node_labels: self.source_node_labels.clone(),
            target_node_labels: self.target_node_labels.clone(),
        }
    }

    fn compute_rows(self) -> Result<Vec<FilteredKnnResultRow>> {
        let config = self.build_config();
        let computation = FilteredKnnComputationRuntime::new();
        let storage = FilteredKnnStorageRuntime::new(config.concurrency);

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("filteredknn".to_string(), self.graph_store.node_count()),
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
                &config.source_node_labels,
                &config.target_node_labels,
                &mut progress_tracker,
            )?
        } else {
            // Multi-property mode should include the primary property passed to `new(...)`.
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
                &config.source_node_labels,
                &config.target_node_labels,
                &mut progress_tracker,
            )?
        };

        Ok(results
            .into_iter()
            .map(FilteredKnnResultRow::from)
            .collect())
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = FilteredKnnResultRow>>> {
        let rows = self.compute_rows()?;
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(self) -> Result<FilteredKnnStats> {
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

        Ok(FilteredKnnStats {
            nodes_compared: sources.len() as u64,
            similarity_pairs: rows.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        })
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

        let err = FilteredKnnBuilder::new(Arc::clone(&store), "does_not_exist")
            .add_property("random_score", SimilarityMetric::Default)
            .k(1)
            .stream()
            .err();

        assert!(err.is_some());
    }
}
