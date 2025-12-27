use crate::procedures::facades::traits::Result;
use crate::procedures::similarity::filteredknn::{
    FilteredKnnComputationRuntime, FilteredKnnConfig, FilteredKnnResultRow, FilteredKnnStorageRuntime,
};
use crate::procedures::similarity::knn::metrics::{KnnNodePropertySpec, SimilarityMetric};
use crate::projection::NodeLabel;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

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

    pub fn add_property(mut self, node_property: impl Into<String>, metric: SimilarityMetric) -> Self {
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

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = FilteredKnnResultRow>>> {
        let config = self.build_config();
        let computation = FilteredKnnComputationRuntime::new();
        let storage = FilteredKnnStorageRuntime::new(config.concurrency);

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
            )?
        } else {
            storage.compute_multi(
                &computation,
                self.graph_store.as_ref(),
                &config.node_properties,
                config.k,
                config.similarity_cutoff,
                &config.source_node_labels,
                &config.target_node_labels,
            )?
        };

        let rows: Vec<FilteredKnnResultRow> = results.into_iter().map(FilteredKnnResultRow::from).collect();
        Ok(Box::new(rows.into_iter()))
    }
}
