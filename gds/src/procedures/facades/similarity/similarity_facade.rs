use crate::procedures::facades::traits::Result;
use crate::procedures::similarity::{
    NodeSimilarityConfig, NodeSimilarityMetric, NodeSimilarityResult,
};
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;

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
        // We need to access the graph from the store.
        // Assuming Orientation::Natural for Similarity.
        // And we include all relationship types (empty HashSet conventionally means all, or we need to know types).
        // Since NodeSimilarity usually operates on specific projection or all, we'll assume ALL for now or usage default.

        let rel_types = HashSet::new(); // Empty usually means "all" in GDS or "none"?
                                        // In GDS projections, usually you specify types. If running on anonymous facade graph, it takes what's there.
                                        // Let's assume we want the default view.

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

    pub fn stats(self) -> Result<()> {
        // TODO: Implement stats
        Ok(())
    }

    pub fn mutate(self, _property: &str) -> Result<()> {
        // TODO: Implement mutate
        Ok(())
    }

    pub fn write(self, _property: &str) -> Result<()> {
        // TODO: Implement write
        Ok(())
    }
}
