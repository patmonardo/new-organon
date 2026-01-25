//! Configuration for streaming graph properties from the graph store.
//!
//! Mirrors Java GraphStreamGraphPropertiesConfig interface and integrates with the Rust config system.

use crate::applications::graph_store_catalog::configs::GraphAccessGraphPropertiesConfig;
use crate::config::validation::ConfigError;

#[derive(Debug, Clone, Default)]
pub struct GraphStreamGraphPropertiesConfig {
    /// Base graph access configuration
    pub access_config: GraphAccessGraphPropertiesConfig,
}

impl GraphStreamGraphPropertiesConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.access_config.validate()
    }

    /// Create a builder for this config
    pub fn builder() -> GraphStreamGraphPropertiesConfigBuilder {
        GraphStreamGraphPropertiesConfigBuilder::default()
    }

    /// Factory method to create config from components.
    pub fn of(graph_name: Option<String>, graph_property: String) -> Result<Self, String> {
        let access_config = GraphAccessGraphPropertiesConfig::of(graph_name, graph_property)?;

        let config = Self::builder()
            .access_config(access_config)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let access_config = GraphAccessGraphPropertiesConfig::from_json(json)?;
        Self::of(access_config.graph_name, access_config.graph_property)
    }

    // Delegate methods to embedded configs
    pub fn graph_name(&self) -> Option<String> {
        self.access_config.graph_name.clone()
    }

    pub fn graph_property(&self) -> String {
        self.access_config.graph_property.clone()
    }
}

impl crate::config::ValidatedConfig for GraphStreamGraphPropertiesConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphStreamGraphPropertiesConfig::validate(self)
    }
}

/// Builder for GraphStreamGraphPropertiesConfig
#[derive(Default)]
pub struct GraphStreamGraphPropertiesConfigBuilder {
    access_config: Option<GraphAccessGraphPropertiesConfig>,
}

impl GraphStreamGraphPropertiesConfigBuilder {
    pub fn access_config(mut self, access_config: GraphAccessGraphPropertiesConfig) -> Self {
        self.access_config = Some(access_config);
        self
    }

    pub fn build(self) -> Result<GraphStreamGraphPropertiesConfig, String> {
        let config = GraphStreamGraphPropertiesConfig {
            access_config: self.access_config.unwrap_or_default(),
        };
        config.validate()?;
        Ok(config)
    }
}
