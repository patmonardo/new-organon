use crate::algo::algorithms::traits as facade;
use crate::algo::algorithms::ConfigValidator;
use crate::algo::embeddings::gat::storage::GATStorageRuntime;
use crate::algo::embeddings::gat::GATConfig;
use crate::algo::embeddings::gat::GATResult;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::types::DefaultGraphStore;
use crate::types::GraphStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GATStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

#[derive(Clone)]
pub struct GATBuilder {
    graph_store: Arc<DefaultGraphStore>,
    config: GATConfig,
}

impl GATBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: GATConfig::default(),
        }
    }

    pub fn embedding_dimension(mut self, dim: usize) -> Self {
        self.config.embedding_dimension = dim;
        self
    }

    pub fn num_heads(mut self, heads: usize) -> Self {
        self.config.num_heads = heads;
        self
    }

    pub fn num_layers(mut self, layers: usize) -> Self {
        self.config.num_layers = layers;
        self
    }

    pub fn epochs(mut self, epochs: usize) -> Self {
        self.config.epochs = epochs;
        self
    }

    pub fn dropout(mut self, dropout: f64) -> Self {
        self.config.dropout = dropout;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.config.alpha = alpha;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = Some(seed);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    fn validate(&self) -> facade::Result<()> {
        if self.config.embedding_dimension == 0 {
            return Err(AlgorithmError::Execution(
                "embedding_dimension must be > 0".into(),
            ));
        }
        if self.config.num_heads == 0 {
            return Err(AlgorithmError::Execution("num_heads must be > 0".into()));
        }
        if self.config.num_layers == 0 {
            return Err(AlgorithmError::Execution("num_layers must be > 0".into()));
        }
        if self.config.epochs == 0 {
            return Err(AlgorithmError::Execution("epochs must be > 0".into()));
        }
        ConfigValidator::in_range(self.config.dropout, 0.0, 1.0, "dropout")?;
        ConfigValidator::in_range(self.config.alpha, 0.0, 1_000_000.0, "alpha")?;
        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        Ok(())
    }

    fn compute(&self) -> facade::Result<GATResult> {
        self.validate()?;

        let storage = GATStorageRuntime::new();
        // For now, assume natural orientation, empty rel types
        let rel_types = std::collections::HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(
                &rel_types,
                &HashMap::new(),
                Orientation::Natural,
            )
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| {
                AlgorithmError::Graph(e.to_string())
            })?;
        Ok(storage.compute(graph.as_ref(), &self.config))
    }

    pub fn run(self) -> facade::Result<GATResult> {
        self.compute()
    }

    pub fn stats(self) -> facade::Result<GATStats> {
        let start = Instant::now();
        let result = self.compute()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        Ok(GATStats {
            node_count: result.num_nodes as u64,
            embedding_dimension: result.embedding_dimension as u64,
            compute_millis,
            success: true,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    use crate::procedures::GraphFacade;

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
    fn facade_run_produces_embeddings() {
        let graph = GraphFacade::new(store());

        let result = graph
            .gat()
            .embedding_dimension(32)
            .num_heads(4)
            .num_layers(2)
            .random_seed(42)
            .run()
            .unwrap();

        assert_eq!(result.node_embeddings.len(), 20);
        assert_eq!(result.embedding_dimension, 32);
        assert_eq!(result.num_nodes, 20);
    }
}
