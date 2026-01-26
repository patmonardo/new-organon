use crate::algo::algorithms::similarity::build_similarity_relationship_store;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::similarity::node_similarity::{
    NodeSimilarityComputationRuntime, NodeSimilarityConfig, NodeSimilarityMetric,
    NodeSimilarityMutateResult, NodeSimilarityResult, NodeSimilarityResultBuilder,
    NodeSimilarityStats, NodeSimilarityStorageRuntime,
};
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Additional imports for progress tracking and similarity stats
use crate::core::utils::progress::TaskProgressTracker;

pub struct NodeSimilarityFacade {
    graph_store: Arc<DefaultGraphStore>,
    metric: NodeSimilarityMetric,
    similarity_cutoff: f64,
    top_k: usize,
    top_n: usize,
    concurrency: usize,
    weight_property: Option<String>,
}

impl NodeSimilarityFacade {
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
        ConfigValidator::in_range(self.top_n as f64, 0.0, 1_000_000.0, "top_n")?;
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
    fn compute_results(&self) -> Result<(Vec<NodeSimilarityResult>, std::time::Duration)> {
        self.validate()?;
        // We need to access the graph from the store.
        // Assuming Orientation::Natural for Similarity.
        // Empty set = all relationship types in the default graph view.

        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();

        let graph = if let Some(prop) = self.weight_property.as_ref() {
            // Provide an explicit selector for every relationship type so DefaultGraph
            // will NOT auto-select unrelated properties when the requested key is missing.
            let selectors = rel_types
                .iter()
                .cloned()
                .map(|t| (t, prop.clone()))
                .collect::<HashMap<_, _>>();

            self.graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Natural,
                )
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?
        } else {
            self.graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?
        };

        let node_count = graph.node_count();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("node_similarity".to_string(), node_count),
            self.concurrency,
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let config = self.build_config();
        let storage = NodeSimilarityStorageRuntime::new(config.concurrency);
        let computation = NodeSimilarityComputationRuntime::new();

        let start = std::time::Instant::now();
        let results = storage.compute(&computation, graph.as_ref(), &config);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        // Convert to public result type
        Ok((
            results
                .into_iter()
                .map(NodeSimilarityResult::from)
                .collect(),
            start.elapsed(),
        ))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = NodeSimilarityResult>>> {
        let (results, _elapsed) = self.compute_results()?;
        Ok(Box::new(results.into_iter()))
    }

    pub fn stats(self) -> Result<NodeSimilarityStats> {
        let (results, _elapsed) = self.compute_results()?;
        Ok(NodeSimilarityResultBuilder::new(&results).stats())
    }

    pub fn mutate(self, property: &str) -> Result<NodeSimilarityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        let graph_store = Arc::clone(&self.graph_store);
        let start = std::time::Instant::now();
        let (results, _elapsed) = self.compute_results()?;
        let builder = NodeSimilarityResultBuilder::new(&results);
        let stats = builder.stats();

        let pairs: Vec<(u64, u64, f64)> = results
            .iter()
            .map(|r| (r.source, r.target, r.similarity))
            .collect();

        let updated_store =
            build_similarity_relationship_store(graph_store.as_ref(), property, &pairs)?;

        let relationships_updated = graph_store.relationship_count() as u64;
        let summary = builder.mutation_summary(property, relationships_updated, start.elapsed());

        Ok(NodeSimilarityMutateResult {
            summary,
            stats,
            updated_store,
        })
    }

    pub fn write(self, property: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        let res = self.mutate(property)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            res.summary.property_name,
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Worst-case output is bounded by top_k per node.
        let pair_count = node_count.saturating_mul(self.top_k);

        // Results + scratch arrays for weights/accumulators.
        let results_memory = pair_count * 32;
        let per_node_scratch = node_count * 32;

        // Weight property access can add additional per-node scratch.
        let weight_memory = if self.weight_property.is_some() {
            node_count * 8
        } else {
            0
        };

        let total = results_memory + per_node_scratch + weight_memory;
        let overhead = total / 5;
        MemoryRange::of_range(total, total + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::GraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn mutate_adds_relationship_property() {
        let config = RandomGraphConfig {
            node_count: 12,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let result = NodeSimilarityFacade::new(Arc::clone(&store))
            .similarity_cutoff(0.0)
            .top_k(3)
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
