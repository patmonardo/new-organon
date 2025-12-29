use super::property_mapping::PropertyMapping;
use crate::projection::{PropertyMappings, PropertyMappingsBuilder};
use std::collections::HashMap;

/// Wildcard symbol to project all properties.
pub const PROJECT_ALL: &str = "*";

/// Key used for properties in configuration objects.
pub const PROPERTIES_KEY: &str = "properties";

/// Base trait for projections of graph elements (nodes or relationships).
///
/// Defines the interface for element projections with property mappings.
pub trait ElementProjection: Send + Sync {
    /// Returns the property mappings for this projection.
    fn properties(&self) -> &PropertyMappings;

    /// Creates a new projection with additional property mappings.
    fn with_additional_property_mappings(
        &self,
        mappings: PropertyMappings,
    ) -> Box<dyn ElementProjection>;

    /// Checks if this projection includes all properties.
    fn project_all(&self) -> bool;

    /// Checks if aggregation should be included in serialized output.
    fn include_aggregation(&self) -> bool;

    /// Converts this projection to a configuration map.
    fn to_config(&self) -> HashMap<String, serde_json::Value>;
}

/// Trait for types that support inline property addition.
///
/// Mirrors the Java GDS inline projection API while delegating to the shared PropertyMappings builder.
pub trait InlineProperties: Sized {
    /// Gets the inline properties builder.
    fn inline_builder(&mut self) -> &mut InlinePropertiesBuilder;

    /// Adds a property mapping.
    fn add_property_mapping(mut self, mapping: PropertyMapping) -> Self {
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        self
    }

    /// Adds a simple property by key.
    fn add_property(mut self, property_key: impl Into<String>) -> Result<Self, String> {
        let mapping = PropertyMapping::of(property_key)?;
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        Ok(self)
    }

    /// Adds a property with source name.
    fn add_property_with_source(
        mut self,
        property_key: impl Into<String>,
        neo_property_key: impl Into<String>,
    ) -> Result<Self, String> {
        let mapping = PropertyMapping::with_source(property_key, neo_property_key)?;
        self.inline_builder()
            .properties_builder()
            .add_mapping_ref(mapping);
        Ok(self)
    }

    /// Adds multiple property mappings.
    fn add_all_properties(mut self, mappings: Vec<PropertyMapping>) -> Self {
        for mapping in mappings {
            self.inline_builder()
                .properties_builder()
                .add_mapping_ref(mapping);
        }
        self
    }

    /// Finalizes the property building process.
    fn build_properties(&mut self) {
        self.inline_builder().build();
    }
}

/// Helper for building properties inline during projection construction.
#[derive(Debug)]
pub struct InlinePropertiesBuilder {
    properties_builder: Option<PropertyMappingsBuilder>,
    properties: Option<PropertyMappings>,
}

impl InlinePropertiesBuilder {
    /// Creates a new InlinePropertiesBuilder.
    pub fn new() -> Self {
        InlinePropertiesBuilder {
            properties_builder: None,
            properties: None,
        }
    }

    /// Creates a builder with existing properties.
    pub fn with_properties(properties: PropertyMappings) -> Self {
        InlinePropertiesBuilder {
            properties_builder: None,
            properties: Some(properties),
        }
    }

    /// Gets or creates the properties builder.
    pub fn properties_builder(&mut self) -> &mut PropertyMappingsBuilder {
        if self.properties_builder.is_none() {
            let builder = if let Some(ref properties) = self.properties {
                PropertyMappingsBuilder::new().from(properties)
            } else {
                PropertyMappingsBuilder::new()
            };
            self.properties = None;
            self.properties_builder = Some(builder);
        }
        self.properties_builder.as_mut().unwrap()
    }

    /// Builds the final PropertyMappings.
    pub fn build(&mut self) -> Option<PropertyMappings> {
        if let Some(builder) = self.properties_builder.take() {
            if self.properties.is_some() {
                panic!(
                    "Cannot have both complete mapping from `properties` \
                     and other properties from `add_property`"
                );
            }
            Some(builder.build())
        } else {
            self.properties.take()
        }
    }

    /// Gets the current properties (if not being built).
    pub fn get_properties(&self) -> Option<&PropertyMappings> {
        self.properties.as_ref()
    }

    /// Sets the properties directly.
    pub fn set_properties(&mut self, properties: PropertyMappings) {
        self.properties = Some(properties);
        self.properties_builder = None;
    }
}

impl Default for InlinePropertiesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_mappings_empty() {
        let mappings = PropertyMappings::empty();
        assert!(mappings.is_empty());
        assert_eq!(mappings.size(), 0);
    }

    #[test]
    fn test_property_mappings_builder() {
        let mappings = PropertyMappings::builder()
            .add_mapping(PropertyMapping::of("age").unwrap())
            .add_mapping(PropertyMapping::with_source("score", "user_score").unwrap())
            .build();

        assert_eq!(mappings.size(), 2);
        let keys = mappings.property_keys();
        assert!(keys.contains("age"));
        assert!(keys.contains("score"));
    }

    #[test]
    fn test_property_mappings_merge() {
        let mappings1 = PropertyMappings::builder()
            .add_mapping(PropertyMapping::of("age").unwrap())
            .build();

        let mappings2 = PropertyMappings::builder()
            .add_mapping(PropertyMapping::of("score").unwrap())
            .build();

        let merged = mappings1.merge_with(&mappings2);
        assert_eq!(merged.size(), 2);
        let keys = merged.property_keys();
        assert!(keys.contains("age"));
        assert!(keys.contains("score"));
    }

    #[test]
    fn test_inline_properties_builder() {
        let mut builder = InlinePropertiesBuilder::new();
        builder
            .properties_builder()
            .add_mapping_ref(PropertyMapping::of("age").unwrap());

        let properties = builder.build().unwrap();
        assert_eq!(properties.size(), 1);
        assert!(properties.property_keys().contains("age"));
    }
}
