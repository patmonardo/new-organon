use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for native projections (catalog-backed stores).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeProjectionConfig {
    pub graph_name: String,
    pub source_graph_name: Option<String>,
    pub relationship_types: Vec<String>,
    pub node_labels: Vec<String>,
    pub node_properties: Vec<String>,
    pub relationship_properties: Vec<String>,
    pub relationship_property_selectors: HashMap<String, String>,
    pub weight_property: Option<String>,
    pub fictitious_loading: bool,
}

/// Configuration for generic projections (cypher-like).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericProjectionConfig {
    pub graph_name: String,
    pub source_graph_name: Option<String>,
    pub relationship_types: Vec<String>,
    pub node_labels: Vec<String>,
    pub node_properties: Vec<String>,
    pub relationship_properties: Vec<String>,
    pub relationship_property_selectors: HashMap<String, String>,
    pub weight_property: Option<String>,
    pub fictitious_loading: bool,
}
