use super::computation::{FilteredKnnComputationResult, FilteredKnnComputationRuntime};
use super::storage::FilteredKnnStorageRuntime;
use crate::define_algorithm_spec;
use crate::procedures::similarity::knn::metrics::{KnnNodePropertySpec, SimilarityMetric};
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::NodeLabel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredKnnConfig {
    /// Single-property mode (backwards-compatible).
    pub node_property: String,

    /// Multi-property mode: if non-empty, this takes precedence over `node_property`.
    #[serde(default)]
    pub node_properties: Vec<KnnNodePropertySpec>,

    #[serde(default = "default_k")]
    pub k: usize,

    #[serde(default)]
    pub similarity_metric: SimilarityMetric,

    #[serde(default = "default_cutoff")]
    pub similarity_cutoff: f64,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// If empty, all nodes are eligible sources. Otherwise, a node is a source if it has ANY of these labels.
    #[serde(default)]
    pub source_node_labels: Vec<NodeLabel>,

    /// If empty, all nodes are eligible targets. Otherwise, a node is a target if it has ANY of these labels.
    #[serde(default)]
    pub target_node_labels: Vec<NodeLabel>,
}

fn default_k() -> usize {
    10
}
fn default_cutoff() -> f64 {
    0.0
}
fn default_concurrency() -> usize {
    4
}

impl Default for FilteredKnnConfig {
    fn default() -> Self {
        Self {
            node_property: String::new(),
            node_properties: Vec::new(),
            k: default_k(),
            similarity_metric: SimilarityMetric::Default,
            similarity_cutoff: default_cutoff(),
            concurrency: default_concurrency(),
            source_node_labels: Vec::new(),
            target_node_labels: Vec::new(),
        }
    }
}

impl FilteredKnnConfig {
    fn validate(&self) -> Result<(), AlgorithmError> {
        if self.node_properties.is_empty() {
            if self.node_property.is_empty() {
                return Err(AlgorithmError::InvalidGraph(
                    "Missing `node_property` (or provide `node_properties`)".to_string(),
                ));
            }
        } else if self
            .node_properties
            .iter()
            .any(|p| p.name.trim().is_empty())
        {
            return Err(AlgorithmError::InvalidGraph(
                "`node_properties` contains an empty property name".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredKnnResultRow {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

impl From<FilteredKnnComputationResult> for FilteredKnnResultRow {
    fn from(r: FilteredKnnComputationResult) -> Self {
        Self {
            source: r.source,
            target: r.target,
            similarity: r.similarity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredKnnAlgorithmResult {
    pub rows: Vec<FilteredKnnResultRow>,
}

impl FilteredKnnAlgorithmResult {
    pub fn new(rows: Vec<FilteredKnnResultRow>) -> Self {
        Self { rows }
    }
}

define_algorithm_spec! {
    name: "filteredknn",
    output_type: FilteredKnnAlgorithmResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    execute: |_self, graph_store, config_input, _context| {
        let parsed: FilteredKnnConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {e}")))?;

        parsed.validate()?;

        let storage = FilteredKnnStorageRuntime::new(parsed.concurrency);
        let computation = FilteredKnnComputationRuntime::new();

        let results = if parsed.node_properties.is_empty() {
            storage.compute_single(
                &computation,
                graph_store,
                &parsed.node_property,
                parsed.k,
                parsed.similarity_cutoff,
                parsed.similarity_metric,
                &parsed.source_node_labels,
                &parsed.target_node_labels,
            )?
        } else {
            storage.compute_multi(
                &computation,
                graph_store,
                &parsed.node_properties,
                parsed.k,
                parsed.similarity_cutoff,
                &parsed.source_node_labels,
                &parsed.target_node_labels,
            )?
        };

        let rows: Vec<FilteredKnnResultRow> = results.into_iter().map(FilteredKnnResultRow::from).collect();
        Ok(FilteredKnnAlgorithmResult::new(rows))
    }
}

pub type FilteredKnnAlgorithmSpec = FILTEREDKNNAlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validation_requires_some_property() {
        let cfg = FilteredKnnConfig::default();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validation_allows_multi_properties() {
        let cfg = FilteredKnnConfig {
            node_property: "".to_string(),
            node_properties: vec![KnnNodePropertySpec::new("a", SimilarityMetric::Cosine)],
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }
}
