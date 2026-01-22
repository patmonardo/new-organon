//! Configuration for writing relationships to the database.
//!
//! Mirrors Java GraphWriteRelationshipConfig interface and integrates with the Rust config system.

use crate::config::validation::ConfigError;
use crate::define_config;

define_config!(
    pub struct GraphWriteRelationshipConfig {
        validate = |cfg: &GraphWriteRelationshipConfig| {
            if cfg.relationship_type.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "relationshipType".to_string(),
                    reason: "must not be empty".to_string(),
                });
            }
            Ok(())
        },
        /// The relationship type to write (required)
        relationship_type: String = String::new(),
        /// Optional relationship property to write
        relationship_property: Option<String> = None,
    }
);

impl GraphWriteRelationshipConfig {
    /// Factory method to create config from components.
    pub fn of(
        relationship_type: String,
        relationship_property: Option<String>,
    ) -> Result<Self, String> {
        let config = Self::builder()
            .relationship_type(relationship_type)
            .relationship_property(relationship_property)
            .build()?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let relationship_type = json
            .get("relationshipType")
            .ok_or("relationshipType is required")?
            .as_str()
            .ok_or("relationshipType must be a string")?
            .to_string();

        let relationship_property = json
            .get("relationshipProperty")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Self::of(relationship_type, relationship_property)
    }
}
