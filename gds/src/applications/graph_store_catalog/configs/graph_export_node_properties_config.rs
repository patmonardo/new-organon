//! Configuration for exporting node properties from the graph store.
//!
//! Mirrors Java GraphExportNodePropertiesConfig interface and integrates with the Rust config system.

use crate::applications::graph_store_catalog::configs::GraphNodePropertiesConfig;
use crate::config::validation::ConfigError;

#[derive(Debug, Clone, Default)]
pub struct GraphExportNodePropertiesConfig {
    /// Base node properties configuration
    pub node_config: GraphNodePropertiesConfig,
    /// List of node properties to export (required, non-empty)
    pub node_properties: Vec<String>,
    /// Whether to include node labels in the output
    pub list_node_labels: bool,
}

impl GraphExportNodePropertiesConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate base node properties config
        self.node_config.validate()?;

        // Additional validation for export-specific fields
        if self.node_properties.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "nodeProperties".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        for prop in &self.node_properties {
            if prop.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeProperties".to_string(),
                    reason: "must not contain empty strings".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Create a builder for this config
    pub fn builder() -> GraphExportNodePropertiesConfigBuilder {
        GraphExportNodePropertiesConfigBuilder::default()
    }

    /// Parse node properties from user input (string or list).
    /// Mirrors Java UserInputAsStringOrListOfString.parse().
    pub fn parse_node_properties(user_input: &serde_json::Value) -> Result<Vec<String>, String> {
        match user_input {
            serde_json::Value::String(s) => {
                if s.trim().is_empty() {
                    return Err("nodeProperties cannot be empty string".into());
                }
                Ok(vec![s.clone()])
            }
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    return Err("nodeProperties array cannot be empty".into());
                }
                let mut result = Vec::new();
                for item in arr {
                    match item {
                        serde_json::Value::String(s) => {
                            if s.trim().is_empty() {
                                return Err("nodeProperties cannot contain empty strings".into());
                            }
                            result.push(s.clone());
                        }
                        _ => return Err("nodeProperties array must contain only strings".into()),
                    }
                }
                Ok(result)
            }
            _ => Err("nodeProperties must be a string or array of strings".into()),
        }
    }

    /// Validates that the specified node properties exist for the given labels.
    /// In Java, this has complex validation logic checking GraphStore.
    pub fn validate_against_store<G: HasNodeProperties>(&self, store: &G) -> Result<(), String> {
        // Check that node properties exist
        for prop in &self.node_properties {
            if !store.has_node_property(prop) {
                let candidates = store.similar_node_properties(prop);
                let message = if !candidates.is_empty() {
                    format!("Did you mean: {}.", candidates.join(", "))
                } else {
                    format!(
                        "Available node properties: {}.",
                        store.node_property_keys().join(", ")
                    )
                };
                return Err(format!(
                    "Node property '{}' does not exist. {}",
                    prop, message
                ));
            }
        }

        // Check that node labels exist (if not wildcard)
        if !self.node_config.node_labels.contains(&"*".to_string()) {
            for label in &self.node_config.node_labels {
                if !store.has_node_label(label) {
                    let candidates = store.similar_node_labels(label);
                    let message = if !candidates.is_empty() {
                        format!("Did you mean: {}.", candidates.join(", "))
                    } else {
                        format!(
                            "Available node labels: {}.",
                            store.node_label_keys().join(", ")
                        )
                    };
                    return Err(format!(
                        "Node label '{}' does not exist. {}",
                        label, message
                    ));
                }
            }
        }

        Ok(())
    }

    /// Factory method to create config from components.
    pub fn of(
        graph_name: Option<String>,
        node_labels: Vec<String>,
        node_properties: Vec<String>,
        list_node_labels: bool,
    ) -> Result<Self, String> {
        let node_config =
            GraphNodePropertiesConfig::of(graph_name, node_labels).map_err(|e| e.to_string())?;

        let config = Self::builder()
            .node_config(node_config)
            .node_properties(node_properties)
            .list_node_labels(list_node_labels)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let node_config = GraphNodePropertiesConfig::from_json(json)?;

        let node_properties = json
            .get("nodeProperties")
            .ok_or("nodeProperties is required")?
            .as_array()
            .ok_or("nodeProperties must be an array")?
            .iter()
            .map(|v| v.as_str().ok_or("nodeProperties must contain strings"))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let list_node_labels = json
            .get("listNodeLabels")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Self::of(
            node_config.graph_name,
            node_config.node_labels,
            node_properties,
            list_node_labels,
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

impl crate::config::ValidatedConfig for GraphExportNodePropertiesConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphExportNodePropertiesConfig::validate(self)
    }
}

/// Builder for GraphExportNodePropertiesConfig
#[derive(Default)]
pub struct GraphExportNodePropertiesConfigBuilder {
    node_config: Option<GraphNodePropertiesConfig>,
    node_properties: Option<Vec<String>>,
    list_node_labels: Option<bool>,
}

impl GraphExportNodePropertiesConfigBuilder {
    pub fn node_config(mut self, node_config: GraphNodePropertiesConfig) -> Self {
        self.node_config = Some(node_config);
        self
    }

    pub fn node_properties(mut self, node_properties: Vec<String>) -> Self {
        self.node_properties = Some(node_properties);
        self
    }

    pub fn list_node_labels(mut self, list_node_labels: bool) -> Self {
        self.list_node_labels = Some(list_node_labels);
        self
    }

    pub fn build(self) -> Result<GraphExportNodePropertiesConfig, String> {
        let config = GraphExportNodePropertiesConfig {
            node_config: self.node_config.unwrap_or_default(),
            node_properties: self.node_properties.unwrap_or_default(),
            list_node_labels: self.list_node_labels.unwrap_or_default(),
        };
        config.validate()?;
        Ok(config)
    }
}

/// Trait for graph stores that support node property/label introspection (for validation)
pub trait HasNodeProperties {
    fn has_node_property(&self, key: &str) -> bool;
    fn node_property_keys(&self) -> Vec<String>;
    fn has_node_label(&self, label: &str) -> bool;
    fn node_label_keys(&self) -> Vec<String>;

    fn similar_node_properties(&self, key: &str) -> Vec<String> {
        let key_lower = key.to_lowercase();
        self.node_property_keys()
            .into_iter()
            .filter(|k| k.to_lowercase().contains(&key_lower))
            .collect()
    }

    fn similar_node_labels(&self, label: &str) -> Vec<String> {
        let label_lower = label.to_lowercase();
        self.node_label_keys()
            .into_iter()
            .filter(|k| k.to_lowercase().contains(&label_lower))
            .collect()
    }
}
