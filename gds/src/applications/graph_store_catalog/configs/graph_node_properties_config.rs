//! Base configuration for accessing node properties from the graph store.
//!
//! Mirrors Java GraphNodePropertiesConfig interface and integrates with the Rust config system.
//! This is the base trait that other node property configs extend.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct GraphNodePropertiesConfig {
        validate = |cfg: &GraphNodePropertiesConfig| {
            // Node labels can be empty (defaults to ["*"]) or contain valid strings
            for label in &cfg.node_labels {
                if label.trim().is_empty() && label != "*" {
                    return Err(ConfigError::InvalidParameter {
                        parameter: "nodeLabels".to_string(),
                        reason: "must not contain empty strings (except '*' wildcard)".to_string(),
                    });
                }
            }
            Ok(())
        },
        /// Optional graph name (if None, must be provided elsewhere)
        graph_name: Option<String> = None,
        /// Node labels to filter by (defaults to ["*"] for all labels)
        node_labels: Vec<String> = vec!["*".to_string()],
    }
);

impl GraphNodePropertiesConfig {
    /// Parse node labels from user input (string or list).
    /// Mirrors Java UserInputAsStringOrListOfString.parse().
    pub fn parse_node_labels(user_input: &serde_json::Value) -> Result<Vec<String>, String> {
        match user_input {
            serde_json::Value::String(s) => {
                if s.trim().is_empty() {
                    return Err("nodeLabels cannot be empty string".into());
                }
                Ok(vec![s.clone()])
            }
            serde_json::Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    match item {
                        serde_json::Value::String(s) => {
                            if s.trim().is_empty() {
                                return Err("nodeLabels cannot contain empty strings".into());
                            }
                            result.push(s.clone());
                        }
                        _ => return Err("nodeLabels array must contain only strings".into()),
                    }
                }
                // Default to ["*"] if empty array
                if result.is_empty() {
                    Ok(vec!["*".to_string()])
                } else {
                    Ok(result)
                }
            }
            _ => Err("nodeLabels must be a string or array of strings".into()),
        }
    }

    /// Factory method to create config from components.
    pub fn of(graph_name: Option<String>, node_labels: Vec<String>) -> Result<Self, String> {
        let config = Self::builder()
            .graph_name(graph_name)
            .node_labels(node_labels)
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

        let node_labels = json
            .get("nodeLabels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| vec!["*".to_string()]);

        Self::of(graph_name, node_labels)
    }
}
