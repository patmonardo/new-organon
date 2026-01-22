//! Configuration for accessing graph properties from the graph store.
//!
//! Mirrors Java GraphAccessGraphPropertiesConfig interface and integrates with the Rust config system.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct GraphAccessGraphPropertiesConfig {
        validate = |cfg: &GraphAccessGraphPropertiesConfig| {
            if cfg.graph_property.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "graphProperty".to_string(),
                    reason: "must not be empty".to_string(),
                });
            }
            Ok(())
        },
        /// Optional graph name (if None, must be provided elsewhere)
        graph_name: Option<String> = None,
        /// The graph property key to access (required)
        graph_property: String = String::new(),
    }
);

impl GraphAccessGraphPropertiesConfig {
    /// Factory method to create config from components.
    pub fn of(graph_name: Option<String>, graph_property: String) -> Result<Self, String> {
        let config = Self::builder()
            .graph_name(graph_name)
            .graph_property(graph_property)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let graph_name = json
            .get("graphName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let graph_property = json
            .get("graphProperty")
            .ok_or("graphProperty is required")?
            .as_str()
            .ok_or("graphProperty must be a string")?
            .to_string();

        Self::of(graph_name, graph_property)
    }

    /// Validate against a GraphStore (optional, for runtime checks)
    pub fn validate_against_store<G: HasGraphPropertyKeys>(&self, store: &G) -> Result<(), String> {
        if !store.has_graph_property(&self.graph_property) {
            let candidates = store.similar_graph_properties(&self.graph_property);
            let message = if !candidates.is_empty() {
                format!("Did you mean: {}.", candidates.join(", "))
            } else {
                format!(
                    "The following properties exist in the graph: {}.",
                    store.graph_property_keys().join(", ")
                )
            };
            return Err(format!(
                "The specified graph property '{}' does not exist. {}",
                self.graph_property, message
            ));
        }
        Ok(())
    }
}

/// Trait for graph stores that support property key introspection (for validation)
pub trait HasGraphPropertyKeys {
    fn has_graph_property(&self, key: &str) -> bool;
    fn graph_property_keys(&self) -> Vec<String>;
    fn similar_graph_properties(&self, key: &str) -> Vec<String> {
        // Simple case-insensitive similarity (could use Levenshtein, etc.)
        let key_lower = key.to_lowercase();
        self.graph_property_keys()
            .into_iter()
            .filter(|k| k.to_lowercase().contains(&key_lower))
            .collect()
    }
}
