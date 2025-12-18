use crate::applications::services::logging::Log;
use crate::projection::PropertyMappings;
use crate::types::graph_store::{GraphName, GraphStore};

use crate::applications::graph_store_catalog::configs::GraphStreamRelationshipPropertiesConfig;
use crate::applications::graph_store_catalog::services::{
    DefaultExportLocation, ExportLocation, GraphNameValidationService, GraphStoreValidationService,
    PropertySpec, UserInputWriteProperties,
};

/// Configuration service for GraphStore catalog operations.
///
/// Mirrors Java CatalogConfigurationService class.
/// This handles parsing and validation of configuration objects.
pub struct CatalogConfigurationService {
    #[allow(dead_code)]
    log: Log,
    graph_name_validation_service: GraphNameValidationService,
    graph_store_validation_service: GraphStoreValidationService,
}

impl CatalogConfigurationService {
    /// Creates a new CatalogConfigurationService.
    pub fn new(log: Log) -> Self {
        Self {
            log: log.clone(),
            graph_name_validation_service: GraphNameValidationService::new(),
            graph_store_validation_service: GraphStoreValidationService::new(),
        }
    }

    /// Validates a graph name configuration.
    pub fn validate_graph_name(&self, graph_name: &str) -> Result<GraphName, String> {
        self.graph_name_validation_service.validate(graph_name)
    }

    /// Validates a graph name configuration with strict validation.
    pub fn validate_graph_name_strictly(&self, graph_name: &str) -> Result<GraphName, String> {
        self.graph_name_validation_service
            .validate_strictly(graph_name)
    }

    /// Validates a possible null graph name.
    pub fn validate_possible_null_graph_name(
        &self,
        graph_name: Option<&str>,
    ) -> Result<Option<GraphName>, String> {
        self.graph_name_validation_service
            .validate_possible_null(graph_name)
    }

    /// Validates a single graph name or list of graph names.
    pub fn validate_single_or_list_graph_name(
        &self,
        graph_name_or_list: &serde_json::Value,
    ) -> Result<Vec<GraphName>, String> {
        self.graph_name_validation_service
            .validate_single_or_list(graph_name_or_list)
    }

    /// Validates graph store properties.
    pub fn validate_graph_store_properties<G: GraphStore>(
        &self,
        graph_store: &G,
        properties: &[String],
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_node_properties_exist(graph_store, properties)
    }

    /// Validates relationship properties against relationship types.
    pub fn validate_relationship_properties(
        &self,
        graph_store: &impl GraphStore,
        relationship_type: &str,
        relationship_properties: &[String],
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_relationship_properties_match_relationship_type(
                graph_store,
                relationship_type,
                relationship_properties,
            )
    }

    /// Validates that relationships may be deleted.
    pub fn validate_relationships_may_be_deleted(
        &self,
        graph_store: &impl GraphStore,
        relationship_type: &str,
        graph_name: &GraphName,
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_relationships_may_be_deleted(graph_store, relationship_type, graph_name.value())
    }

    /// Validates graph property exists.
    pub fn validate_graph_property_exists(
        &self,
        graph_store: &impl GraphStore,
        graph_property: &str,
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_graph_property_exists(graph_store, graph_property)
    }

    /// Validates read access permissions.
    pub fn validate_read_access(
        &self,
        graph_store: &impl GraphStore,
        should_export_additional_node_properties: bool,
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_read_access(graph_store, should_export_additional_node_properties)
    }

    /// Parses user input write properties.
    pub fn parse_user_input_write_properties(
        &self,
        user_input: &serde_json::Value,
        configuration_key: &str,
    ) -> Result<Vec<PropertySpec>, String> {
        let value = user_input
            .as_str()
            .ok_or_else(|| format!("{} must be a string", configuration_key))?;
        UserInputWriteProperties::parse(value, configuration_key)
    }

    /// Creates an export location from configuration.
    pub fn create_export_location(&self, export_path: Option<&str>) -> Box<dyn ExportLocation> {
        let path = export_path.map(std::path::PathBuf::from);
        Box::new(DefaultExportLocation::new(path))
    }

    /// Validates node properties match node labels.
    pub fn validate_node_properties_match_node_labels(
        &self,
        graph_store: &impl GraphStore,
        node_labels: &[String],
        node_properties: &[String],
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_node_properties_match_node_labels(graph_store, node_labels, node_properties)
    }

    /// Validates relationship properties match relationship types.
    pub fn validate_relationship_properties_match_relationship_types(
        &self,
        graph_store: &impl GraphStore,
        configuration: &impl GraphStreamRelationshipPropertiesConfig,
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_relationship_properties_match_relationship_types(
                graph_store,
                &configuration.relationship_types(),
                &configuration.relationship_properties(),
            )
    }

    /// Validates relationship types are present.
    pub fn validate_relationship_types_present(
        &self,
        graph_store: &impl GraphStore,
        relationship_types: &[String],
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_relationship_types_present(graph_store, relationship_types)
    }

    /// Validates possible relationship property matches relationship type.
    pub fn validate_possible_relationship_property_matches_relationship_type(
        &self,
        graph_store: &impl GraphStore,
        relationship_type_as_string: &str,
        possible_relationship_property: Option<&str>,
    ) -> Result<(), String> {
        self.graph_store_validation_service
            .ensure_possible_relationship_property_matches_relationship_type(
                graph_store,
                relationship_type_as_string,
                possible_relationship_property,
            )
    }

    /// Filters existing node properties.
    pub fn filter_existing_node_properties(
        &self,
        graph_store: &impl GraphStore,
        node_properties: &[String],
    ) -> Vec<String> {
        self.graph_store_validation_service
            .filter_existing_node_properties(graph_store, node_properties)
    }

    /// Validates node properties do not exist.
    pub fn validate_node_properties_not_exist(
        &self,
        graph_store: &impl GraphStore,
        _additional_node_properties: &PropertyMappings,
    ) -> Result<(), String> {
        let additional: Vec<String> = Vec::new();
        self.graph_store_validation_service
            .ensure_node_properties_not_exist(graph_store, &additional)
    }
}
