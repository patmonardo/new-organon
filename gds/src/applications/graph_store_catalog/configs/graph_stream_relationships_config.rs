//! Configuration for streaming relationships from the graph store.
//!
//! Mirrors Java GraphStreamRelationshipsConfig interface and integrates with the Rust config system.
//! Similar pattern to GraphNodePropertiesConfig but for relationships.

use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    pub struct GraphStreamRelationshipsConfig {
        validate = |cfg: &GraphStreamRelationshipsConfig| {
            // Relationship types can be empty (defaults to ["*"]) or contain valid strings
            for rel_type in &cfg.relationship_types {
                if rel_type.trim().is_empty() && rel_type != "*" {
                    return Err(ConfigError::InvalidParameter {
                        parameter: "relationshipTypes".to_string(),
                        reason: "must not contain empty strings (except '*' wildcard)".to_string(),
                    });
                }
            }
            Ok(())
        },
        /// Optional graph name (if None, must be provided elsewhere)
        graph_name: Option<String> = None,
        /// Relationship types to filter by (defaults to ["*"] for all types)
        relationship_types: Vec<String> = vec!["*".to_string()],
    }
);

impl GraphStreamRelationshipsConfig {
    /// Parse relationship types from user input (string or list).
    /// Mirrors Java UserInputAsStringOrListOfString.parse().
    pub fn parse_relationship_types(user_input: &serde_json::Value) -> Result<Vec<String>, String> {
        match user_input {
            serde_json::Value::String(s) => {
                if s.trim().is_empty() {
                    return Err("relationshipTypes cannot be empty string".into());
                }
                Ok(vec![s.clone()])
            }
            serde_json::Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    match item {
                        serde_json::Value::String(s) => {
                            if s.trim().is_empty() {
                                return Err("relationshipTypes cannot contain empty strings".into());
                            }
                            result.push(s.clone());
                        }
                        _ => return Err("relationshipTypes array must contain only strings".into()),
                    }
                }
                // Default to ["*"] if empty array
                if result.is_empty() {
                    Ok(vec!["*".to_string()])
                } else {
                    Ok(result)
                }
            }
            _ => Err("relationshipTypes must be a string or array of strings".into()),
        }
    }

    /// Factory method to create config from components.
    pub fn of(graph_name: Option<String>, relationship_types: Vec<String>) -> Result<Self, String> {
        let config = Self::builder()
            .graph_name(graph_name)
            .relationship_types(relationship_types)
            .build()?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let graph_name = json
            .get("graphName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let relationship_types = json
            .get("relationshipTypes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| vec!["*".to_string()]);

        Self::of(graph_name, relationship_types)
    }
}
