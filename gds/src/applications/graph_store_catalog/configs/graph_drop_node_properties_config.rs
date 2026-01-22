//! Configuration for dropping node properties from the graph store.
//!
//! Mirrors Java GraphDropNodePropertiesConfig interface and integrates with the Rust config system.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::config::base_types::ConcurrencyConfig;
use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct GraphDropNodePropertiesConfig {
        validate = |cfg: &GraphDropNodePropertiesConfig| {
            if cfg.node_properties.is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeProperties".to_string(),
                    reason: "must not be empty".to_string(),
                });
            }
            for prop in &cfg.node_properties {
                if prop.trim().is_empty() {
                    return Err(ConfigError::InvalidParameter {
                        parameter: "nodeProperties".to_string(),
                        reason: "must not contain empty strings".to_string(),
                    });
                }
            }
            Ok(())
        },
        /// Optional graph name (if None, must be provided elsewhere)
        graph_name: Option<String> = None,
        /// List of node properties to drop (required, non-empty)
        node_properties: Vec<String> = Vec::new(),
        /// Whether to fail if properties are missing (default: true)
        fail_if_missing: bool = true,
        /// Concurrency level for the operation
        concurrency: usize = 4,
    }
);

impl ConcurrencyConfig for GraphDropNodePropertiesConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl GraphDropNodePropertiesConfig {
    /// Parse node properties from user input (string or list).
    /// Mirrors Java UserInputAsStringOrListOfString.parse().
    pub fn parse_node_properties(user_input: &serde_json::Value) -> Result<Vec<String>, String> {
        match user_input {
            serde_json::Value::String(s) => {
                // Single string - treat as one property
                if s.trim().is_empty() {
                    return Err("nodeProperties cannot be empty string".into());
                }
                Ok(vec![s.clone()])
            }
            serde_json::Value::Array(arr) => {
                // Array of strings
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

    /// Factory method to create config from components.
    /// Mirrors Java static of() method.
    pub fn of(
        graph_name: Option<String>,
        node_properties: Vec<String>,
        fail_if_missing: bool,
        concurrency: usize,
    ) -> Result<Self, String> {
        let config = Self::builder()
            .graph_name(graph_name)
            .node_properties(node_properties)
            .fail_if_missing(fail_if_missing)
            .concurrency(concurrency)
            .build()?;

        config.validate()?;
        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let graph_name = json
            .get("graphName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

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

        let fail_if_missing = json
            .get("failIfMissing")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let concurrency = json
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(4) as usize;

        Self::of(graph_name, node_properties, fail_if_missing, concurrency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = GraphDropNodePropertiesConfig::builder()
            .node_properties(vec!["prop1".to_string(), "prop2".to_string()])
            .build();
        assert!(config.is_ok());
    }

    #[test]
    fn test_empty_node_properties_fails() {
        let config = GraphDropNodePropertiesConfig::builder()
            .node_properties(vec![])
            .build();
        assert!(config.is_err());
        let err = config.unwrap_err();
        match err {
            ConfigError::InvalidParameter { parameter, .. } => {
                assert_eq!(parameter, "nodeProperties");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_from_json_malformed() {
        // Test with missing required nodeProperties
        let json = serde_json::json!({
            "graphName": "test_graph"
        });
        let config = GraphDropNodePropertiesConfig::from_json(&json);
        assert!(config.is_err());

        // Test with empty nodeProperties array
        let json = serde_json::json!({
            "graphName": "test_graph",
            "nodeProperties": []
        });
        let config = GraphDropNodePropertiesConfig::from_json(&json);
        assert!(config.is_err());

        // Test with non-string in nodeProperties
        let json = serde_json::json!({
            "graphName": "test_graph",
            "nodeProperties": ["prop1", 123]
        });
        let config = GraphDropNodePropertiesConfig::from_json(&json);
        assert!(config.is_err());
    }
}
