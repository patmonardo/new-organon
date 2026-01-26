use crate::types::graph::NodeId;
use std::collections::HashMap;

pub struct AttentionMechanism {
    pub attention_weights: HashMap<NodeId, Vec<f64>>, // node -> weights to neighbors
}

impl Default for AttentionMechanism {
    fn default() -> Self {
        Self::new()
    }
}

impl AttentionMechanism {
    pub fn new() -> Self {
        Self {
            attention_weights: HashMap::new(),
        }
    }

    pub fn compute_attention(
        &mut self,
        node: NodeId,
        neighbors: &[NodeId],
        features: &HashMap<NodeId, Vec<f64>>,
        config: &super::config::GATConfig,
    ) {
        // Simplified attention: dot product with leaky relu
        let node_feat = &features[&node];
        let mut weights = Vec::new();

        for &neighbor in neighbors {
            let neigh_feat = &features[&neighbor];
            let mut score = 0.0;
            for i in 0..node_feat.len() {
                score += node_feat[i] * neigh_feat[i];
            }
            // LeakyReLU
            score = if score > 0.0 {
                score
            } else {
                config.alpha * score
            };
            weights.push(score);
        }

        // Softmax
        let max_score = weights.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_weights: Vec<f64> = weights.iter().map(|w| (w - max_score).exp()).collect();
        let sum_exp: f64 = exp_weights.iter().sum();
        let softmax_weights: Vec<f64> = exp_weights.iter().map(|e| e / sum_exp).collect();

        self.attention_weights.insert(node, softmax_weights);
    }

    pub fn get_weights(&self, node: NodeId) -> Option<&Vec<f64>> {
        self.attention_weights.get(&node)
    }
}
