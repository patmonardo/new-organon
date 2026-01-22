use super::aggregator::GATAggregator;
use super::config::GATConfig;
use super::types::GATResult;
use crate::types::graph::Graph;
use std::collections::HashMap;

pub struct GATComputationRuntime;

impl GATComputationRuntime {
    pub fn run(graph: &dyn Graph, config: &GATConfig) -> GATResult {
        // Initialize features randomly or from properties (simplified: random)
        let mut features = HashMap::new();
        for node in graph.iter() {
            let feat: Vec<f64> = (0..config.embedding_dimension)
                .map(|_| rand::random::<f64>() * 2.0 - 1.0) // random in [-1,1]
                .collect();
            features.insert(node, feat);
        }

        let mut aggregator = GATAggregator::new(config.clone());
        let final_features = aggregator.aggregate(graph, features);

        GATResult {
            node_embeddings: final_features,
            embedding_dimension: config.embedding_dimension,
            num_nodes: graph.node_count(),
        }
    }
}
