//! HashGNN facade (builder API).

use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::algo::embeddings::hashgnn::algo::{
    HashGNNComputationRuntime, HashGNNConfig, HashGNNStorageRuntime,
};
pub use crate::algo::embeddings::hashgnn::spec::{
    BinarizeFeaturesConfig, GenerateFeaturesConfig, HashGNNEmbeddings, HashGNNResult,
};
use crate::prints::{PrintEnvelope, PrintKind, PrintProvenance};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashGNNStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "outputMode")]
    pub output_mode: String,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// HashGNN builder.
#[derive(Clone)]
pub struct HashGNNBuilder {
    graph_store: Arc<DefaultGraphStore>,
    config: HashGNNConfig,
}

impl HashGNNBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: HashGNNConfig::default(),
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.config.iterations = iterations;
        self
    }

    pub fn embedding_density(mut self, embedding_density: usize) -> Self {
        self.config.embedding_density = embedding_density;
        self
    }

    pub fn neighbor_influence(mut self, neighbor_influence: f64) -> Self {
        self.config.neighbor_influence = neighbor_influence;
        self
    }

    pub fn feature_properties(mut self, feature_properties: Vec<String>) -> Self {
        self.config.feature_properties = feature_properties;
        self
    }

    pub fn heterogeneous(mut self, heterogeneous: bool) -> Self {
        self.config.heterogeneous = heterogeneous;
        self
    }

    pub fn output_dimension(mut self, output_dimension: Option<usize>) -> Self {
        self.config.output_dimension = output_dimension;
        self
    }

    pub fn binarize_features(mut self, binarize_features: Option<BinarizeFeaturesConfig>) -> Self {
        self.config.binarize_features = binarize_features;
        self
    }

    pub fn generate_features(mut self, generate_features: Option<GenerateFeaturesConfig>) -> Self {
        self.config.generate_features = generate_features;
        self
    }

    pub fn random_seed(mut self, random_seed: Option<u64>) -> Self {
        self.config.random_seed = random_seed;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.config.iterations == 0 {
            return Err(AlgorithmError::Execution("iterations must be > 0".into()));
        }
        if self.config.embedding_density == 0 {
            return Err(AlgorithmError::Execution(
                "embedding_density must be > 0".into(),
            ));
        }
        ConfigValidator::in_range(
            self.config.neighbor_influence,
            0.0,
            1_000_000.0,
            "neighbor_influence",
        )?;

        if self.config.feature_properties.is_empty() && self.config.generate_features.is_none() {
            return Err(AlgorithmError::Execution(
                "HashGNN requires either feature_properties or generate_features".into(),
            ));
        }

        if let Some(cfg) = &self.config.generate_features {
            if cfg.dimension == 0 {
                return Err(AlgorithmError::Execution(
                    "generate_features.dimension must be > 0".into(),
                ));
            }
            if cfg.density_level == 0 {
                return Err(AlgorithmError::Execution(
                    "generate_features.density_level must be > 0".into(),
                ));
            }
        }

        if let Some(cfg) = &self.config.binarize_features {
            if cfg.dimension == 0 {
                return Err(AlgorithmError::Execution(
                    "binarize_features.dimension must be > 0".into(),
                ));
            }
        }

        Ok(())
    }

    fn compute(&self) -> Result<HashGNNResult> {
        self.validate()?;

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = HashGNNStorageRuntime::new();
        HashGNNComputationRuntime::run(graph_view, &self.config, &storage)
    }

    /// Full result: returns embeddings.
    pub fn run(&self) -> Result<HashGNNResult> {
        self.compute()
    }

    pub fn stats(&self) -> Result<HashGNNStats> {
        let start = Instant::now();
        let result = self.compute()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        let (output_mode, embedding_dimension, node_count) = match &result.embeddings {
            HashGNNEmbeddings::BinaryIndices {
                embedding_dimension,
                embeddings,
            } => (
                "binary".to_string(),
                *embedding_dimension as u64,
                embeddings.len() as u64,
            ),
            HashGNNEmbeddings::Dense { embeddings } => (
                "dense".to_string(),
                embeddings.first().map(|v| v.len() as u64).unwrap_or(0),
                embeddings.len() as u64,
            ),
        };

        Ok(HashGNNStats {
            node_count,
            embedding_dimension,
            output_mode,
            compute_millis,
            success: true,
        })
    }

    /// Full result + a canonical print envelope (summary) emitted at the boundary.
    pub fn run_with_print(&self, run_id: Option<String>) -> Result<(HashGNNResult, PrintEnvelope)> {
        let result = self.compute()?;

        let (output_mode, embedding_dimension, node_count) = match &result.embeddings {
            HashGNNEmbeddings::BinaryIndices {
                embedding_dimension,
                embeddings,
            } => ("binary", *embedding_dimension, embeddings.len()),
            HashGNNEmbeddings::Dense { embeddings } => (
                "dense",
                embeddings.first().map(|v| v.len()).unwrap_or(0),
                embeddings.len(),
            ),
        };

        let print = PrintEnvelope::new(
            PrintKind::Ml,
            PrintProvenance {
                source: "gds::hashgnn".to_string(),
                run_id,
                kernel_version: None,
            },
            serde_json::json!({
                "algo": "hashgnn",
                "node_count": node_count,
                "output_mode": output_mode,
                "embedding_dimension": embedding_dimension,
                "config": {
                    "iterations": self.config.iterations,
                    "embedding_density": self.config.embedding_density,
                    "neighbor_influence": self.config.neighbor_influence,
                    "feature_properties": self.config.feature_properties.clone(),
                    "heterogeneous": self.config.heterogeneous,
                    "output_dimension": self.config.output_dimension,
                    "binarize_features": self.config.binarize_features.as_ref().map(|cfg| {
                        serde_json::json!({
                            "dimension": cfg.dimension,
                            "threshold": cfg.threshold,
                        })
                    }),
                    "generate_features": self.config.generate_features.as_ref().map(|cfg| {
                        serde_json::json!({
                            "dimension": cfg.dimension,
                            "density_level": cfg.density_level,
                        })
                    }),
                    "random_seed": self.config.random_seed,
                    "concurrency": self.config.concurrency,
                }
            }),
        );

        Ok((result, print))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 20,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.3)],
            directed: true,
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn facade_run_with_print_emits_summary_without_embeddings() {
        let graph = GraphFacade::new(store());

        let (result, print) = graph
            .hash_gnn()
            .iterations(2)
            .embedding_density(4)
            .neighbor_influence(1.0)
            .generate_features(Some(GenerateFeaturesConfig {
                dimension: 64,
                density_level: 3,
            }))
            .concurrency(1)
            .random_seed(Some(7))
            .run_with_print(Some("run-test-hashgnn".to_string()))
            .unwrap();

        match result.embeddings {
            HashGNNEmbeddings::BinaryIndices {
                embeddings,
                embedding_dimension,
            } => {
                assert_eq!(embeddings.len(), 20);
                assert_eq!(embedding_dimension, 64);
            }
            _ => panic!("expected binary output"),
        }

        let value = serde_json::to_value(&print).expect("serialize print");
        assert_eq!(value["kind"], "ml");
        assert_eq!(value["provenance"]["source"], "gds::hashgnn");
        assert_eq!(value["provenance"]["run_id"], "run-test-hashgnn");
        assert_eq!(value["payload"]["algo"], "hashgnn");
        assert_eq!(value["payload"]["output_mode"], "binary");
        assert_eq!(value["payload"]["embedding_dimension"], 64);
        assert!(value["payload"].get("embeddings").is_none());
    }
}
