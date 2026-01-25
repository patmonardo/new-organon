use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", rename = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(
        default = "default_include_intermediate_communities",
        rename = "includeIntermediateCommunities"
    )]
    pub include_intermediate_communities: bool,

    #[serde(default)]
    pub seed_property: Option<String>,

    #[serde(default = "default_gamma")]
    pub gamma: f64,

    #[serde(default = "default_theta")]
    pub theta: f64,
}

fn default_max_iterations() -> usize {
    10
}

fn default_tolerance() -> f64 {
    0.0001
}

fn default_include_intermediate_communities() -> bool {
    false
}

fn default_gamma() -> f64 {
    1.0
}

fn default_theta() -> f64 {
    0.01
}

fn default_concurrency() -> usize {
    4
}

impl Default for LouvainConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            include_intermediate_communities: default_include_intermediate_communities(),
            seed_property: None,
            gamma: default_gamma(),
            theta: default_theta(),
        }
    }
}

impl crate::config::ValidatedConfig for LouvainConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_positive(self.concurrency as f64, "concurrency")?;
        crate::config::validate_positive(self.max_iterations as f64, "maxIterations")?;
        crate::config::validate_positive(self.tolerance, "tolerance")?;
        crate::config::validate_range(self.gamma, 0.0, 10.0, "gamma")?;
        crate::config::validate_range(self.theta, 0.0, 1.0, "theta")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainResult {
    pub data: Vec<u64>,
}

pub struct LouvainAlgorithmSpec {
    graph_name: String,
}

impl LouvainAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
