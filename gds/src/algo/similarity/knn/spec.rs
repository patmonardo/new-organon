use super::metrics::{KnnNodePropertySpec, SimilarityMetric};
use super::KnnSamplerType;
use super::KnnStorageRuntime;
use super::{KnnComputationResult, KnnComputationRuntime, KnnNnDescentStats};
use crate::algo::algorithms::similarity::similarity_stats;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnConfig {
    /// Single-property mode (backwards-compatible).
    pub node_property: String,

    /// Multi-property mode: if non-empty, this takes precedence over `node_property`.
    #[serde(default)]
    pub node_properties: Vec<KnnNodePropertySpec>,
    #[serde(default = "default_k")]
    pub k: usize,

    /// NN-Descent sampledK (how many "new" neighbors to explore per iteration).
    ///
    /// If omitted, defaults to `ceil(k / 2)`.
    #[serde(default)]
    pub sampled_k: Option<usize>,

    /// Max iterations for NN-Descent.
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Initial sampling strategy.
    #[serde(default)]
    pub initial_sampler: KnnSamplerType,

    /// Optional random seed (used for sampling and random joins).
    #[serde(default)]
    pub random_seed: Option<u64>,

    /// Perturbation rate for accepting non-improving neighbors.
    #[serde(default = "default_perturbation_rate")]
    pub perturbation_rate: f64,

    /// Random joins per node per iteration.
    #[serde(default = "default_random_joins")]
    pub random_joins: usize,

    /// Convergence update threshold (stop when updates per-iteration <= threshold).
    #[serde(default = "default_update_threshold")]
    pub update_threshold: u64,
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
fn default_max_iterations() -> usize {
    10
}
fn default_perturbation_rate() -> f64 {
    0.0
}
fn default_random_joins() -> usize {
    0
}
fn default_update_threshold() -> u64 {
    0
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
            sampled_k: None,
            max_iterations: default_max_iterations(),
            initial_sampler: KnnSamplerType::default(),
            random_seed: None,
            perturbation_rate: default_perturbation_rate(),
            random_joins: default_random_joins(),
            update_threshold: default_update_threshold(),
            similarity_metric: SimilarityMetric::Default,
            similarity_cutoff: default_cutoff(),
            concurrency: default_concurrency(),
        }
    }
}

impl KnnConfig {
    fn validate(&self) -> Result<(), AlgorithmError> {
        if self.k == 0 {
            return Err(AlgorithmError::InvalidGraph("`k` must be > 0".to_string()));
        }
        if !(0.0..=1.0).contains(&self.perturbation_rate) {
            return Err(AlgorithmError::InvalidGraph(
                "`perturbation_rate` must be within [0.0, 1.0]".to_string(),
            ));
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnStats {
    #[serde(rename = "nodesCompared")]
    pub nodes_compared: u64,
    #[serde(rename = "ranIterations")]
    pub ran_iterations: u64,
    #[serde(rename = "didConverge")]
    pub did_converge: bool,
    #[serde(rename = "nodePairsConsidered")]
    pub node_pairs_considered: u64,
    #[serde(rename = "similarityPairs")]
    pub similarity_pairs: u64,
    #[serde(rename = "similarityDistribution")]
    pub similarity_distribution: HashMap<String, f64>,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnnWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for KNN: summary + stats + updated store
#[derive(Debug, Clone)]
pub struct KnnMutateResult {
    pub summary: KnnMutationSummary,
    pub stats: KnnStats,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// KNN result builder (facade adapter).
pub struct KnnResultBuilder<'a> {
    rows: &'a [KnnResultRow],
    nn_stats: &'a KnnNnDescentStats,
}

impl<'a> KnnResultBuilder<'a> {
    pub fn new(rows: &'a [KnnResultRow], nn_stats: &'a KnnNnDescentStats) -> Self {
        Self { rows, nn_stats }
    }

    pub fn stats(&self) -> KnnStats {
        let mut sources = HashSet::new();
        let tuples: Vec<(u64, u64, f64)> = self
            .rows
            .iter()
            .map(|r| {
                sources.insert(r.source);
                (r.source, r.target, r.similarity)
            })
            .collect();

        let stats = similarity_stats(|| tuples.into_iter(), true);

        KnnStats {
            nodes_compared: sources.len() as u64,
            ran_iterations: self.nn_stats.ran_iterations as u64,
            did_converge: self.nn_stats.did_converge,
            node_pairs_considered: self.nn_stats.node_pairs_considered,
            similarity_pairs: self.rows.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        }
    }

    pub fn mutation_summary(
        &self,
        property_name: &str,
        nodes_updated: u64,
        execution_time: Duration,
    ) -> KnnMutationSummary {
        KnnMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
        execution_time: Duration,
    ) -> KnnWriteSummary {
        KnnWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        }
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

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("knn".to_string(), graph_store.node_count()),
            parsed.concurrency,
        );

        let results = if parsed.node_properties.is_empty() {
            storage.compute_single(
                &computation,
                graph_store,
                &parsed.node_property,
                parsed.k,
                parsed
                    .sampled_k
                    .unwrap_or_else(|| (parsed.k + 1) / 2)
                    .min(parsed.k),
                parsed.max_iterations,
                parsed.similarity_cutoff,
                parsed.similarity_metric,
                parsed.perturbation_rate,
                parsed.random_joins,
                parsed.update_threshold,
                parsed.random_seed,
                parsed.initial_sampler,
                &mut progress_tracker,
            )?
        } else {
            storage.compute_multi(
                &computation,
                graph_store,
                &parsed.node_properties,
                parsed.k,
                parsed
                    .sampled_k
                    .unwrap_or_else(|| (parsed.k + 1) / 2)
                    .min(parsed.k),
                parsed.max_iterations,
                parsed.similarity_cutoff,
                parsed.perturbation_rate,
                parsed.random_joins,
                parsed.update_threshold,
                parsed.random_seed,
                parsed.initial_sampler,
                &mut progress_tracker,
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
