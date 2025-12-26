//! Configuration for writing relationship properties to the database.
//!
//! Mirrors Java WriteRelationshipPropertiesConfig interface and integrates with the Rust config system.

use crate::define_config;

define_config!(
    pub struct WriteRelationshipPropertiesConfig {
        // No specific configuration parameters - marker config similar to Java
    }
);

impl WriteRelationshipPropertiesConfig {
    /// Factory method to create config from components.
    pub fn of() -> Result<Self, String> {
        Ok(Self::builder().build()?)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(_json: &serde_json::Value) -> Result<Self, String> {
        Self::of()
    }
}
