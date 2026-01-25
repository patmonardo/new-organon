//! Configuration for streaming relationship properties from the graph store.
//!
//! Mirrors Java GraphStreamRelationshipPropertiesConfig interface and integrates with the Rust config system.

use crate::applications::graph_store_catalog::configs::GraphStreamRelationshipsConfig;
use crate::config::validation::ConfigError;

#[derive(Debug, Clone, Default)]
pub struct GraphStreamRelationshipPropertiesConfig {
    /// Base relationship streaming configuration
    pub relationships_config: GraphStreamRelationshipsConfig,
    /// List of relationship properties to stream
    pub relationship_properties: Vec<String>,
}

impl GraphStreamRelationshipPropertiesConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate base relationships config
        self.relationships_config.validate()?;

        // Additional validation for relationship properties
        if self.relationship_properties.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "relationshipProperties".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        for prop in &self.relationship_properties {
            if prop.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "relationshipProperties".to_string(),
                    reason: "must not contain empty strings".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Create a builder for this config
    pub fn builder() -> GraphStreamRelationshipPropertiesConfigBuilder {
        GraphStreamRelationshipPropertiesConfigBuilder::default()
    }

    /// Factory method to create config from components.
    pub fn of(
        graph_name: Option<String>,
        relationship_types: Vec<String>,
        relationship_properties: Vec<String>,
    ) -> Result<Self, String> {
        let relationships_config =
            GraphStreamRelationshipsConfig::of(graph_name, relationship_types)?;

        let config = Self::builder()
            .relationships_config(relationships_config)
            .relationship_properties(relationship_properties)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(config)
    }

    /// Create from JSON value (for wire protocol deserialization)
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        let relationships_config = GraphStreamRelationshipsConfig::from_json(json)?;

        let relationship_properties = json
            .get("relationshipProperties")
            .ok_or("relationshipProperties is required")?
            .as_array()
            .ok_or("relationshipProperties must be an array")?
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or("relationshipProperties must contain strings")
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        Self::of(
            relationships_config.graph_name,
            relationships_config.relationship_types,
            relationship_properties,
        )
    }

    // Delegate methods to embedded configs
    pub fn graph_name(&self) -> Option<String> {
        self.relationships_config.graph_name.clone()
    }

    pub fn relationship_types(&self) -> Vec<String> {
        self.relationships_config.relationship_types.clone()
    }
}

impl crate::config::ValidatedConfig for GraphStreamRelationshipPropertiesConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphStreamRelationshipPropertiesConfig::validate(self)
    }
}

/// Builder for GraphStreamRelationshipPropertiesConfig
#[derive(Default)]
pub struct GraphStreamRelationshipPropertiesConfigBuilder {
    relationships_config: Option<GraphStreamRelationshipsConfig>,
    relationship_properties: Option<Vec<String>>,
}

impl GraphStreamRelationshipPropertiesConfigBuilder {
    pub fn relationships_config(
        mut self,
        relationships_config: GraphStreamRelationshipsConfig,
    ) -> Self {
        self.relationships_config = Some(relationships_config);
        self
    }

    pub fn relationship_properties(mut self, relationship_properties: Vec<String>) -> Self {
        self.relationship_properties = Some(relationship_properties);
        self
    }

    pub fn build(self) -> Result<GraphStreamRelationshipPropertiesConfig, String> {
        let config = GraphStreamRelationshipPropertiesConfig {
            relationships_config: self.relationships_config.unwrap_or_default(),
            relationship_properties: self.relationship_properties.unwrap_or_default(),
        };
        config.validate()?;
        Ok(config)
    }
}

/// Trait for graph stores that support relationship property/type introspection (for validation)
pub trait HasRelationshipProperties {
    fn has_relationship_property(&self, rel_type: &str, key: &str) -> bool;
    fn relationship_property_keys(&self, rel_type: &str) -> Vec<String>;
    fn has_relationship_type(&self, rel_type: &str) -> bool;
    fn relationship_types(&self) -> Vec<String>;

    fn similar_relationship_properties(&self, rel_type: &str, key: &str) -> Vec<String> {
        let key_lower = key.to_lowercase();
        self.relationship_property_keys(rel_type)
            .into_iter()
            .filter(|k| k.to_lowercase().contains(&key_lower))
            .collect()
    }

    fn similar_relationship_types(&self, rel_type: &str) -> Vec<String> {
        let type_lower = rel_type.to_lowercase();
        self.relationship_types()
            .into_iter()
            .filter(|t| t.to_lowercase().contains(&type_lower))
            .collect()
    }
}
