//! FastRP facade (builder API).

use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::algo::embeddings::fastrp::{
    FastRPComputationRuntime, FastRPConfig, FastRPResult, FastRPStorageRuntime,
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
pub struct FastRPStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// Stream row: `(node_id, embedding)`.
#[derive(Debug, Clone, PartialEq)]
pub struct FastRPRow {
    pub node_id: u64,
    pub embedding: Vec<f32>,
}

/// FastRP builder.
#[derive(Clone)]
pub struct FastRPBuilder {
    graph_store: Arc<DefaultGraphStore>,
    config: FastRPConfig,
}

impl FastRPBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: FastRPConfig::default(),
        }
    }

    pub fn embedding_dimension(mut self, embedding_dimension: usize) -> Self {
        self.config.embedding_dimension = embedding_dimension;
        self
    }

    pub fn property_dimension(mut self, property_dimension: usize) -> Self {
        self.config.property_dimension = property_dimension;
        self
    }

    pub fn iteration_weights(mut self, iteration_weights: Vec<f32>) -> Self {
        self.config.iteration_weights = iteration_weights;
        self
    }

    pub fn feature_properties(mut self, feature_properties: Vec<String>) -> Self {
        self.config.feature_properties = feature_properties;
        self
    }

    pub fn relationship_weight_property(
        mut self,
        relationship_weight_property: Option<String>,
    ) -> Self {
        self.config.relationship_weight_property = relationship_weight_property;
        self
    }

    pub fn normalization_strength(mut self, normalization_strength: f32) -> Self {
        self.config.normalization_strength = normalization_strength;
        self
    }

    pub fn node_self_influence(mut self, node_self_influence: f32) -> Self {
        self.config.node_self_influence = node_self_influence;
        self
    }

    pub fn random_seed(mut self, random_seed: Option<u64>) -> Self {
        self.config.random_seed = random_seed;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.config.min_batch_size = min_batch_size;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.config.embedding_dimension == 0 {
            return Err(AlgorithmError::Execution(
                "embedding_dimension must be > 0".into(),
            ));
        }
        if self.config.property_dimension > self.config.embedding_dimension {
            return Err(AlgorithmError::Execution(
                "property_dimension must be <= embedding_dimension".into(),
            ));
        }
        if self.config.iteration_weights.is_empty() {
            return Err(AlgorithmError::Execution(
                "iteration_weights must be non-empty".into(),
            ));
        }
        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        Ok(())
    }

    fn compute(&self) -> Result<FastRPResult> {
        self.validate()?;

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = FastRPStorageRuntime::new();
        let extractors = storage
            .feature_extractors(graph_view.as_ref(), &self.config.feature_properties)
            .map_err(AlgorithmError::Execution)?;

        FastRPComputationRuntime::run(Arc::clone(&graph_view), &self.config, extractors)
    }

    /// Stream mode: yields `(node_id, embedding)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = FastRPRow>>> {
        let result = self.compute()?;
        let iter = result
            .embeddings
            .into_iter()
            .enumerate()
            .map(|(node_id, embedding)| FastRPRow {
                node_id: node_id as u64,
                embedding,
            });
        Ok(Box::new(iter))
    }

    /// Full result: returns all embeddings.
    pub fn run(&self) -> Result<FastRPResult> {
        self.compute()
    }

    pub fn stats(&self) -> Result<FastRPStats> {
        let start = Instant::now();
        let result = self.compute()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        let node_count = result.embeddings.len() as u64;
        let embedding_dimension = result
            .embeddings
            .first()
            .map(|v| v.len() as u64)
            .unwrap_or(0);

        Ok(FastRPStats {
            node_count,
            embedding_dimension,
            compute_millis,
            success: true,
        })
    }

    /// Full result + a canonical print envelope (summary) emitted at the boundary.
    ///
    /// The print payload is intentionally summary-only (no embedding matrix).
    pub fn run_with_print(&self, run_id: Option<String>) -> Result<(FastRPResult, PrintEnvelope)> {
        let result = self.compute()?;

        let node_count = result.embeddings.len();
        let embedding_dimension = result.embeddings.first().map(|v| v.len()).unwrap_or(0);

        let print = PrintEnvelope::new(
            PrintKind::Ml,
            PrintProvenance {
                source: "gds::fastrp".to_string(),
                run_id,
                kernel_version: None,
            },
            serde_json::json!({
                "algo": "fastrp",
                "node_count": node_count,
                "embedding_dimension": embedding_dimension,
                "config": {
                    "feature_properties": self.config.feature_properties,
                    "iteration_weights": self.config.iteration_weights,
                    "embedding_dimension": self.config.embedding_dimension,
                    "property_dimension": self.config.property_dimension,
                    "relationship_weight_property": self.config.relationship_weight_property,
                    "normalization_strength": self.config.normalization_strength,
                    "node_self_influence": self.config.node_self_influence,
                    "concurrency": self.config.concurrency,
                    "min_batch_size": self.config.min_batch_size,
                    "random_seed": self.config.random_seed,
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
            seed: Some(9),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.6)],
            directed: true,
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn facade_run_with_print_emits_summary_without_embeddings() {
        let graph = GraphFacade::new(store());

        let (result, print) = graph
            .fast_rp()
            .embedding_dimension(8)
            .property_dimension(0)
            .iteration_weights(vec![1.0, 1.0])
            .concurrency(1)
            .random_seed(Some(1))
            .run_with_print(Some("run-test-2".to_string()))
            .unwrap();

        assert_eq!(result.embeddings.len(), 10);
        assert_eq!(result.embeddings[0].len(), 8);

        let value = serde_json::to_value(&print).expect("serialize print");
        assert_eq!(value["kind"], "ml");
        assert_eq!(value["provenance"]["source"], "gds::fastrp");
        assert_eq!(value["provenance"]["run_id"], "run-test-2");
        assert_eq!(value["payload"]["algo"], "fastrp");
        assert_eq!(value["payload"]["embedding_dimension"], 8);
        assert!(value["payload"].get("embeddings").is_none());
    }
}
