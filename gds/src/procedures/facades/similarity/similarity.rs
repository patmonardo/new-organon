use crate::procedures::facades::builder_base::ConfigValidator;
use crate::procedures::facades::traits::Result;
use crate::procedures::similarity::{
    NodeSimilarityConfig, NodeSimilarityMetric, NodeSimilarityResult,
};
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityStats {
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

pub struct SimilarityBuilder {
    graph_store: Arc<DefaultGraphStore>,
    metric: NodeSimilarityMetric,
    similarity_cutoff: f64,
    top_k: usize,
    top_n: usize,
    concurrency: usize,
    weight_property: Option<String>,
}

impl SimilarityBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            metric: NodeSimilarityMetric::Jaccard, // Default
            similarity_cutoff: 0.1,                // Default from GDS
            top_k: 10,
            top_n: 0,
            concurrency: 4,
            weight_property: None,
        }
    }

    pub fn metric(mut self, metric: NodeSimilarityMetric) -> Self {
        self.metric = metric;
        self
    }

    pub fn similarity_cutoff(mut self, cutoff: f64) -> Self {
        self.similarity_cutoff = cutoff;
        self
    }

    pub fn top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn top_n(mut self, n: usize) -> Self {
        self.top_n = n;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn weight_property(mut self, property: String) -> Self {
        self.weight_property = Some(property);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.similarity_cutoff, 0.0, 1.0, "similarity_cutoff")?;
        ConfigValidator::in_range(self.top_k as f64, 1.0, 1_000_000.0, "top_k")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        if let Some(prop) = &self.weight_property {
            ConfigValidator::non_empty_string(prop, "weight_property")?;
        }
        Ok(())
    }

    fn build_config(&self) -> NodeSimilarityConfig {
        NodeSimilarityConfig {
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            top_k: self.top_k,
            top_n: self.top_n,
            concurrency: self.concurrency,
            weight_property: self.weight_property.clone(),
        }
    }

    // Computation helper
    fn compute_results(&self) -> Result<Vec<NodeSimilarityResult>> {
        self.validate()?;
        // We need to access the graph from the store.
        // Assuming Orientation::Natural for Similarity.
        // Empty set = all relationship types in the default graph view.

        let rel_types = HashSet::new();

        let graph = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

        let config = self.build_config();
        let storage =
            crate::procedures::similarity::NodeSimilarityStorageRuntime::new(config.concurrency);
        let mut computation =
            crate::procedures::similarity::NodeSimilarityComputationRuntime::new();

        let results = storage.compute(&mut computation, graph.as_ref(), &config);

        // Convert to public result type
        Ok(results
            .into_iter()
            .map(NodeSimilarityResult::from)
            .collect())
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = NodeSimilarityResult>>> {
        let results = self.compute_results()?;
        Ok(Box::new(results.into_iter()))
    }

    pub fn stats(self) -> Result<NodeSimilarityStats> {
        let results = self.compute_results()?;

        let mut sources = HashSet::new();
        let tuples: Vec<(u64, u64, f64)> = results
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

        Ok(NodeSimilarityStats {
            nodes_compared: sources.len() as u64,
            similarity_pairs: results.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        })
    }

    pub fn mutate(self, property: &str) -> Result<()> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        Err(AlgorithmError::Execution(
            "Node Similarity mutate/write is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, property: &str) -> Result<()> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        Err(AlgorithmError::Execution(
            "Node Similarity mutate/write is not implemented yet".to_string(),
        ))
    }
}
