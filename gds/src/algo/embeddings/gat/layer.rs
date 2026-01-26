use super::attention::AttentionMechanism;
use super::config::GATConfig;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::HashMap;

pub struct GATLayer {
    pub config: GATConfig,
    pub attention: AttentionMechanism,
}

impl GATLayer {
    pub fn new(config: GATConfig) -> Self {
        Self {
            config,
            attention: AttentionMechanism::new(),
        }
    }

    pub fn forward(&mut self, graph: &dyn Graph, features: &mut HashMap<NodeId, Vec<f64>>) {
        // For each node, compute attention and aggregate
        let nodes: Vec<NodeId> = graph.iter().collect();

        for &node in &nodes {
            let neighbors: Vec<NodeId> = graph
                .stream_relationships(node, 0.0)
                .map(|cursor| cursor.target_id())
                .collect();
            if neighbors.is_empty() {
                continue;
            }

            self.attention
                .compute_attention(node, &neighbors, features, &self.config);

            if let Some(weights) = self.attention.get_weights(node) {
                let mut new_embedding = vec![0.0; self.config.embedding_dimension];

                // Weighted sum of neighbor features
                for (i, &neighbor) in neighbors.iter().enumerate() {
                    let neigh_feat = &features[&neighbor];
                    let weight = weights[i];
                    for j in 0..new_embedding.len() {
                        new_embedding[j] += weight * neigh_feat[j];
                    }
                }

                // Concatenate with self (simplified multi-head)
                let self_feat = &features[&node];
                new_embedding.extend_from_slice(self_feat);

                // Update features (in-place for simplicity)
                features.insert(node, new_embedding);
            }
        }
    }
}
