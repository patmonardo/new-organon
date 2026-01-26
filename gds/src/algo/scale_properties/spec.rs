//! ScaleProperties specification and configuration.
//! Mirrors the Java `ScalePropertiesBaseConfig` family.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported scaler variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScalePropertiesScaler {
    #[serde(rename = "minMax")]
    MinMax,
    #[serde(rename = "stdScore")]
    StdScore,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "none")]
    None,
}

impl Default for ScalePropertiesScaler {
    fn default() -> Self {
        Self::MinMax
    }
}

/// Configuration for scaling node properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalePropertiesConfig {
    /// Node property names to scale. Array properties are flattened.
    #[serde(default)]
    pub node_properties: Vec<String>,
    /// Scaler variant to apply.
    #[serde(default)]
    pub scaler: ScalePropertiesScaler,
    /// Concurrency hint for stats computation and scaling.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for ScalePropertiesConfig {
    fn default() -> Self {
        Self {
            node_properties: Vec::new(),
            scaler: ScalePropertiesScaler::default(),
            concurrency: default_concurrency(),
        }
    }
}

/// Result of ScaleProperties computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePropertiesResult {
    /// Per-node scaled values. Each row length equals the sum of input property dimensions.
    pub scaled_properties: Vec<Vec<f64>>,
    /// Per-property scaler statistics keyed by property name, then statistic name.
    pub scaler_statistics: HashMap<String, HashMap<String, Vec<f64>>>,
}

/// Stream row for ScaleProperties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePropertiesStreamRow {
    pub node_id: u64,
    pub values: Vec<f64>,
}

/// Statistics for ScaleProperties computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePropertiesStats {
    pub scaler: String,
    pub stats: HashMap<String, HashMap<String, Vec<f64>>>,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePropertiesMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePropertiesWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Catalog marker for executor wiring.
pub struct ScalePropertiesAlgorithmSpec {
    graph_name: String,
}

impl ScalePropertiesAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
