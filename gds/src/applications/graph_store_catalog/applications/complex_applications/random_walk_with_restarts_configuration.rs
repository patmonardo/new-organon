use serde::{Deserialize, Serialize};

/// Configuration for random-walk sampling variants.
///
/// Java parity: maps the procedure configuration used by RWR/CNARW sampling.
/// Rust pass-1: supports deterministic induced-subgraph sampling; extra fields are accepted
/// for forward compatibility but may be ignored by the current implementation.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomWalkWithRestartsConfiguration {
    pub graph_name: String,
    pub from_graph_name: String,

    pub sample_node_count: Option<usize>,
    pub sample_ratio: Option<f64>,
    pub seed: Option<u64>,

    pub restart_probability: Option<f64>,
    pub walk_length: Option<usize>,
    pub concurrency: Option<usize>,
}
