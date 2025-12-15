use crate::procedures::similarity::similarity_metric::{NodeSimilarityMetric, SimilarityMetric};
use crate::procedures::similarity::vector_computer::{
    UnweightedVectorComputer, VectorComputer, WeightedVectorComputer,
};
use crate::projection::orientation::Orientation;
use crate::types::graph::Graph;
use rayon::prelude::*;

pub struct NodeSimilarityConfig {
    pub similarity_metric: NodeSimilarityMetric,
    pub similarity_cutoff: f64,
    pub top_k: usize,
    pub top_n: usize,
    pub concurrency: usize,
    pub weight_property: Option<String>,
}

pub struct NodeSimilarityResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

pub struct NodeSimilarity {
    config: NodeSimilarityConfig,
}

impl NodeSimilarity {
    pub fn new(config: NodeSimilarityConfig) -> Self {
        Self { config }
    }

    pub fn compute(&self, graph: &dyn Graph) -> Vec<NodeSimilarityResult> {
        let metric = self
            .config
            .similarity_metric
            .create(self.config.similarity_cutoff);
        let vector_computer: Box<dyn VectorComputer> =
            if let Some(prop) = &self.config.weight_property {
                Box::new(WeightedVectorComputer::new(
                    graph,
                    prop.clone(),
                    Orientation::Natural,
                ))
            } else {
                Box::new(UnweightedVectorComputer::new(
                    graph,
                    None,
                    Orientation::Natural,
                ))
            };

        // Naive parallel implementation: for each node, compare with all other nodes (higher ID).
        // This is O(N^2) in worst case.
        // Optimizations like TopN/TopK and locality sensitive hashing would go here.
        // Java GDS limits comparisons via Degree Cutoff and other heuristics.

        let nodes: Vec<u64> = (0..graph.node_count()).map(|i| i as u64).collect();

        // Use rayon to parallelize outer loop
        nodes
            .par_iter()
            .flat_map(|&source| {
                let source_vector = vector_computer.vector(source);
                let source_weights = vector_computer.weights(source);

                // Only compare with nodes > source to avoid duplicates and self-loops
                // and symmetric calculations
                nodes
                    .iter()
                    .filter(move |&&target| target > source)
                    .filter_map(move |&target| {
                        let target_vector = vector_computer.vector(target); // This might be expensive re-fetching

                        let similarity = if self.config.weight_property.is_some() {
                            let target_weights = vector_computer.weights(target);
                            metric.compute_weighted_similarity(
                                &source_vector,
                                &target_vector,
                                &source_weights,
                                &target_weights,
                            )
                        } else {
                            metric.compute_similarity(&source_vector, &target_vector)
                        };

                        if !similarity.is_nan() {
                            Some(NodeSimilarityResult {
                                source,
                                target,
                                similarity,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
