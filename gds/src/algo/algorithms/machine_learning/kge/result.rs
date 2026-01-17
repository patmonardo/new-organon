//! Result containers for KGE predictions.

use serde::Serialize;
use std::collections::HashMap;

/// A single predicted link with score.
#[derive(Debug, Clone, Serialize)]
pub struct KgePrediction {
    /// Source node id.
    pub source_node_id: i64,
    /// Target node id.
    pub target_node_id: i64,
    /// Predicted score.
    pub score: f64,
}

/// Aggregated predictions keyed by source node.
#[derive(Debug, Clone)]
pub struct KgePredictResult {
    by_source: HashMap<i64, Vec<KgePrediction>>,
    links_considered: u64,
}

impl KgePredictResult {
    /// Construct a new result container.
    pub fn new(by_source: HashMap<i64, Vec<KgePrediction>>, links_considered: u64) -> Self {
        Self {
            by_source,
            links_considered,
        }
    }

    /// Returns predictions grouped by source node.
    pub fn by_source(&self) -> &HashMap<i64, Vec<KgePrediction>> {
        &self.by_source
    }

    /// Returns the number of candidate links evaluated.
    pub fn links_considered(&self) -> u64 {
        self.links_considered
    }

    /// Returns the total number of predicted links.
    pub fn relationship_count(&self) -> u64 {
        self.by_source.values().map(|v| v.len() as u64).sum()
    }

    /// Returns an iterator over all predictions.
    pub fn iter(&self) -> impl Iterator<Item = &KgePrediction> {
        self.by_source.values().flat_map(|v| v.iter())
    }
}
