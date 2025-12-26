//! Configuration for streaming node properties from the graph store.
//!
//! Mirrors Java GraphStreamNodePropertiesConfig interface.
//! This is essentially a type alias for GraphExportNodePropertiesConfig since
//! streaming and exporting have the same configuration needs.

use crate::applications::graph_store_catalog::configs::graph_export_node_properties_config::GraphExportNodePropertiesConfig;

/// Configuration for streaming node properties.
/// In Java, this extends GraphExportNodePropertiesConfig.
/// In Rust, we use the same config struct since they have identical fields.
pub type GraphStreamNodePropertiesConfig = GraphExportNodePropertiesConfig;
