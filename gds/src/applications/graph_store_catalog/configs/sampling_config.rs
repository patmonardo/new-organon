use crate::config::validation::ConfigError;
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

impl SamplingConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

impl crate::config::ValidatedConfig for SamplingConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        SamplingConfig::validate(self)
    }
}
