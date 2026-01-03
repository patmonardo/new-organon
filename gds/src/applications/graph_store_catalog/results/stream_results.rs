// Stream result types - direct translation from Java

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result for streaming node properties.
/// Mirrors Java GraphStreamNodePropertyResult class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphStreamNodePropertyResult {
    pub node_id: i64,
    pub property_value: Value,
    pub node_labels: Vec<String>,
}

impl GraphStreamNodePropertyResult {
    pub fn new(node_id: i64, property_value: Value, node_labels: Vec<String>) -> Self {
        Self {
            node_id,
            property_value,
            node_labels,
        }
    }
}

/// Result for streaming multiple node properties.
/// Mirrors Java GraphStreamNodePropertiesResult class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphStreamNodePropertiesResult {
    pub node_id: i64,
    pub node_property: String,
    pub property_value: Value,
    pub node_labels: Vec<String>,
}

impl GraphStreamNodePropertiesResult {
    pub fn new(
        node_id: i64,
        node_property: String,
        property_value: Value,
        node_labels: Vec<String>,
    ) -> Self {
        Self {
            node_id,
            node_property,
            property_value,
            node_labels,
        }
    }
}

/// Result for streaming relationship properties.
/// Mirrors Java GraphStreamRelationshipPropertyResult class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphStreamRelationshipPropertyResult {
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub relationship_type: String,
    pub property_value: Value,
}

impl GraphStreamRelationshipPropertyResult {
    pub fn new(
        source_node_id: i64,
        target_node_id: i64,
        relationship_type: String,
        property_value: Value,
    ) -> Self {
        Self {
            source_node_id,
            target_node_id,
            relationship_type,
            property_value,
        }
    }
}

/// Result for streaming multiple relationship properties.
/// Mirrors Java GraphStreamRelationshipPropertiesResult class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphStreamRelationshipPropertiesResult {
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub relationship_type: String,
    pub relationship_property: String,
    pub property_value: Value,
}

/// Result for streaming graph properties.
/// Mirrors Java GraphStreamGraphPropertiesResult class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphStreamGraphPropertiesResult {
    pub graph_property: String,
    pub property_value: Value,
}

impl GraphStreamGraphPropertiesResult {
    pub fn new(graph_property: String, property_value: Value) -> Self {
        Self {
            graph_property,
            property_value,
        }
    }
}

impl GraphStreamRelationshipPropertiesResult {
    pub fn new(
        source_node_id: i64,
        target_node_id: i64,
        relationship_type: String,
        relationship_property: String,
        property_value: Value,
    ) -> Self {
        Self {
            source_node_id,
            target_node_id,
            relationship_type,
            relationship_property,
            property_value,
        }
    }
}
