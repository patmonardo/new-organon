//! Configuration for writing node properties to the database.
//!
//! Mirrors Java GraphWriteNodePropertiesConfig interface and integrates with the Rust config system.

use crate::applications::graph_store_catalog::configs::GraphNodePropertiesConfig;
use crate::config::validation::ConfigError;

#[derive(Debug, Clone, Default)]
pub struct GraphWriteNodePropertiesConfig {
    /// Base node properties configuration
    pub node_config: GraphNodePropertiesConfig,
    /// List of node property specifications for writing
    pub node_properties: Vec<NodePropertySpec>,
}

impl GraphWriteNodePropertiesConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate base node properties config
        self.node_config.validate()?;

        // Additional validation for write-specific fields
        if self.node_properties.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "nodeProperties".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        for prop in &self.node_properties {
            if prop.node_property_name.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeProperties".to_string(),
                    reason: "must not contain empty property names".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Create a builder for this config
    pub fn builder() -> GraphWriteNodePropertiesConfigBuilder {
        GraphWriteNodePropertiesConfigBuilder::default()
    }

    /// Factory method to create config from components.
    pub fn of(
        graph_name: Option<String>,
        node_labels: Vec<String>,
        node_properties: Vec<NodePropertySpec>,
    ) -> Result<Self, String> {
        let node_config =
            GraphNodePropertiesConfig::of(graph_name, node_labels).map_err(|e| e.to_string())?;

        let config = Self::builder()
            .node_config(node_config)
            .node_properties(node_properties)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let node_config = GraphNodePropertiesConfig::from_json(json)?;

        // Parse node properties - this would need more complex parsing in real implementation
        // For now, assume it's an array of strings
        let node_properties = json
            .get("nodeProperties")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        v.as_str()
                            .map(|s| NodePropertySpec::new(s.to_string(), None))
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self::of(
            node_config.graph_name,
            node_config.node_labels,
            node_properties,
        )
    }

    // Delegate methods to embedded configs
    pub fn graph_name(&self) -> Option<String> {
        self.node_config.graph_name.clone()
    }

    pub fn node_labels(&self) -> Vec<String> {
        self.node_config.node_labels.clone()
    }
}

impl crate::config::ValidatedConfig for GraphWriteNodePropertiesConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphWriteNodePropertiesConfig::validate(self)
    }
}

/// Builder for GraphWriteNodePropertiesConfig
#[derive(Default)]
pub struct GraphWriteNodePropertiesConfigBuilder {
    node_config: Option<GraphNodePropertiesConfig>,
    node_properties: Option<Vec<NodePropertySpec>>,
}

impl GraphWriteNodePropertiesConfigBuilder {
    pub fn node_config(mut self, node_config: GraphNodePropertiesConfig) -> Self {
        self.node_config = Some(node_config);
        self
    }

    pub fn node_properties(mut self, node_properties: Vec<NodePropertySpec>) -> Self {
        self.node_properties = Some(node_properties);
        self
    }

    pub fn build(self) -> Result<GraphWriteNodePropertiesConfig, String> {
        let config = GraphWriteNodePropertiesConfig {
            node_config: self.node_config.unwrap_or_default(),
            node_properties: self.node_properties.unwrap_or_default(),
        };
        config.validate()?;
        Ok(config)
    }
}

/// Specification for a node property to be written.
///
/// Mirrors Java UserInputWriteProperties.PropertySpec.
#[derive(Clone, Debug)]
pub struct NodePropertySpec {
    node_property_name: String,
    renamed_node_property: Option<String>,
}

impl NodePropertySpec {
    /// Creates a new NodePropertySpec.
    pub fn new(node_property_name: String, renamed_node_property: Option<String>) -> Self {
        Self {
            node_property_name,
            renamed_node_property,
        }
    }

    /// Returns the property name to write to the database.
    pub fn write_property(&self) -> String {
        self.renamed_node_property
            .clone()
            .unwrap_or_else(|| self.node_property_name.clone())
    }

    /// Returns the original node property name.
    pub fn node_property(&self) -> String {
        self.node_property_name.clone()
    }
}
