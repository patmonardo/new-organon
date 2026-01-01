use super::computation::{KnnComputationResult, KnnComputationRuntime};
use super::metrics::{KnnNodePropertySpec, SimilarityMetric};
use super::storage::KnnStorageRuntime;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnConfig {
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

impl Default for KnnConfig {
    fn default() -> Self {
        Self {
            node_property: String::new(),
            node_properties: Vec::new(),
            k: default_k(),
            similarity_metric: SimilarityMetric::Default,
            similarity_cutoff: default_cutoff(),
            concurrency: default_concurrency(),
        }
    }
}

impl KnnConfig {
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
pub struct KnnResultRow {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

impl From<KnnComputationResult> for KnnResultRow {
    fn from(r: KnnComputationResult) -> Self {
        Self {
            source: r.source,
            target: r.target,
            similarity: r.similarity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnAlgorithmResult {
    pub rows: Vec<KnnResultRow>,
}

impl KnnAlgorithmResult {
    pub fn new(rows: Vec<KnnResultRow>) -> Self {
        Self { rows }
    }
}

define_algorithm_spec! {
    name: "knn",
    output_type: KnnAlgorithmResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    execute: |_self, graph_store, config_input, _context| {
        let parsed: KnnConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {e}")))?;

        parsed.validate()?;

        let storage = KnnStorageRuntime::new(parsed.concurrency);
        let computation = KnnComputationRuntime::new();

        let results = if parsed.node_properties.is_empty() {
            storage.compute_single(
                &computation,
                graph_store,
                &parsed.node_property,
                parsed.k,
                parsed.similarity_cutoff,
                parsed.similarity_metric,
            )?
        } else {
            storage.compute_multi(
                &computation,
                graph_store,
                &parsed.node_properties,
                parsed.k,
                parsed.similarity_cutoff,
            )?
        };

        let rows: Vec<KnnResultRow> = results.into_iter().map(KnnResultRow::from).collect();
        Ok(KnnAlgorithmResult::new(rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validation_allows_single_property() {
        let cfg = KnnConfig {
            node_property: "foo".to_string(),
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn config_validation_requires_some_property() {
        let cfg = KnnConfig::default();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validation_allows_multi_properties() {
        let cfg = KnnConfig {
            node_property: "".to_string(),
            node_properties: vec![KnnNodePropertySpec::new("a", SimilarityMetric::Cosine)],
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }
}

pub type KnnAlgorithmSpec = KNNAlgorithmSpec;
