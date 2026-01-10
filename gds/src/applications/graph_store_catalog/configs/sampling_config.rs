use serde::{Deserialize, Serialize};

/// Sampling parameters for GraphSamplingApplication.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingConfig {
    pub sample_node_count: Option<usize>,
    pub sample_ratio: Option<f64>,
    pub sampled_graph_name: Option<String>,
    pub seed: Option<u64>,
}
