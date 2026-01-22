//! Configuration for mutating node labels in the graph store.
//!
//! Mirrors Java MutateLabelConfig interface and integrates with the Rust config system.

use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    pub struct MutateLabelConfig {
        validate = |cfg: &MutateLabelConfig| {
            if cfg.node_filter.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "nodeFilter".to_string(),
                    reason: "must not be empty".to_string(),
                });
            }
            Ok(())
        },
        /// The node filter expression as a string (required)
        node_filter: String = String::new(),
    }
);

impl MutateLabelConfig {
    /// Factory method to create config from components.
    pub fn of(node_filter: String) -> Result<Self, String> {
        let config = Self::builder().node_filter(node_filter).build()?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let node_filter = json
            .get("nodeFilter")
            .ok_or("nodeFilter is required")?
            .as_str()
            .ok_or("nodeFilter must be a string")?
            .to_string();

        Self::of(node_filter)
    }
}
