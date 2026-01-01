use crate::procedures::facades::traits::Result;
use crate::procedures::similarity::knn::{
    KnnConfig, KnnNodePropertySpec, KnnResultRow, SimilarityMetric,
};
use crate::types::prelude::DefaultGraphStore;
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
        let computation = crate::procedures::similarity::knn::KnnComputationRuntime::new();
        let storage =
            crate::procedures::similarity::knn::KnnStorageRuntime::new(config.concurrency);

        let results = if config.node_properties.is_empty() {
            storage.compute_single(
                &computation,
                self.graph_store.as_ref(),
                &config.node_property,
                config.k,
                config.similarity_cutoff,
                config.similarity_metric,
            )?
        } else {
            storage.compute_multi(
                &computation,
                self.graph_store.as_ref(),
                &config.node_properties,
                config.k,
                config.similarity_cutoff,
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

        let stats = crate::procedures::core::result::similarity::similarity_stats(
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
}
