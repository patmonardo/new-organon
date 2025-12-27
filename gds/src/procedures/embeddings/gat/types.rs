use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GATResult {
    pub node_embeddings: HashMap<i64, Vec<f64>>, // node_id -> embedding vector
    pub embedding_dimension: usize,
    pub num_nodes: usize,
}

#[derive(Clone, Debug)]
pub struct AttentionWeights {
    pub weights: Vec<f64>, // attention coefficients for neighbors
}
