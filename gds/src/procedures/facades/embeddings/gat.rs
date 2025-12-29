use crate::prelude::DefaultGraphStore;
use crate::prelude::GraphStore;
use crate::procedures::embeddings::gat::storage::GATStorageRuntime;
use crate::procedures::embeddings::GATConfig;
use crate::procedures::embeddings::GATResult;
use crate::procedures::facades::traits as facade;
use std::collections::HashMap;
use std::sync::Arc;

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

    pub fn run(self) -> facade::Result<GATResult> {
        let storage = GATStorageRuntime::new();
        // For now, assume natural orientation, empty rel types
        let rel_types = std::collections::HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(
                &rel_types,
                &HashMap::new(),
                crate::projection::orientation::Orientation::Natural,
            )
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;
        Ok(storage.compute(graph.as_ref(), &self.config))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::facades::Graph;
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
    fn facade_run_produces_embeddings() {
        let graph = Graph::new(store());

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
