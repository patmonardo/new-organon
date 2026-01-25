use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};

/// Configuration for synthetic graph generation.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphGenerationRelationshipConfig {
    pub relationship_type: String,
    pub probability: f64,
}

/// Configuration for GenerateGraphApplication.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphGenerationConfig {
    pub graph_name: Option<String>,
    pub node_count: Option<u64>,
    pub node_labels: Vec<String>,
    pub relationships: Vec<GraphGenerationRelationshipConfig>,
    pub directed: Option<bool>,
    pub inverse_indexed: Option<bool>,
    pub seed: Option<u64>,
}

impl GraphGenerationConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(n) = self.node_count {
            if n == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeCount".to_string(),
                    reason: "nodeCount must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for GraphGenerationConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphGenerationConfig::validate(self)
    }
}
