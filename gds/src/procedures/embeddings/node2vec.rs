//! Node2Vec facade (builder API).
//!
//! **What is it?**: Node embeddings via biased random walks + skip-gram (Word2Vec-style)
//! **Why care?**: Produces a dense vector per node for similarity search, clustering, and ML
//! **Key knobs**: `return_factor` ($p$), `in_out_factor` ($q$), `window_size`, `negative_sampling_rate`
//!
//! ## How to think about it
//!
//! 1) **Walks create a corpus**: each walk is a sentence of node ids.
//! 2) **Skip-gram training**: nodes that co-occur in windows become close in vector space.
//! 3) **Bias controls signal**:
//!    - $q > 1$ tends to stay local (BFS-ish) → community similarity.
//!    - $q < 1$ explores outward (DFS-ish) → structural similarity.
//!
//! ## Example
//! ```rust,no_run
//! # use std::sync::Arc;
//! # use gds::types::graph_store::DefaultGraphStore;
//! # use gds::procedures::GraphFacade;
//! # let store = Arc::new(DefaultGraphStore::empty());
//! let graph = Graph::new(store);
//! let rows: Vec<_> = graph
//!     .node2vec()
//!     .walks_per_node(10)
//!     .walk_length(80)
//!     .return_factor(1.0)
//!     .in_out_factor(1.0)
//!     .embedding_dimension(128)
//!     .iterations(1)
//!     .random_seed(Some(42))
//!     .stream()?
//!     .collect();
//! # Ok::<(), gds::projection::eval::algorithm::AlgorithmError>(())
//! ```

use crate::algo::algorithms::ConfigValidator;
use crate::algo::algorithms::Result;
use crate::algo::embeddings::node2vec::{
    EmbeddingInitializerConfig, Node2VecComputationRuntime, Node2VecConfig, Node2VecResult,
    Node2VecStorageRuntime,
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
pub struct Node2VecStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "iterations")]
    pub iterations: u64,
    #[serde(rename = "lossPerIteration")]
    pub loss_per_iteration: Vec<f64>,
    #[serde(rename = "lastLoss")]
    pub last_loss: Option<f64>,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// Stream row: `(node_id, embedding)`.
#[derive(Debug, Clone, PartialEq)]
pub struct Node2VecRow {
    pub node_id: u64,
    pub embedding: Vec<f32>,
}

/// Node2Vec builder.
#[derive(Clone)]
pub struct Node2VecBuilder {
    graph_store: Arc<DefaultGraphStore>,
    config: Node2VecConfig,
}

impl Node2VecBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: Node2VecConfig::default(),
        }
    }

    pub fn walks_per_node(mut self, walks_per_node: usize) -> Self {
        self.config.walks_per_node = walks_per_node;
        self
    }

    pub fn walk_length(mut self, walk_length: usize) -> Self {
        self.config.walk_length = walk_length;
        self
    }

    pub fn return_factor(mut self, return_factor: f64) -> Self {
        self.config.return_factor = return_factor;
        self
    }

    pub fn in_out_factor(mut self, in_out_factor: f64) -> Self {
        self.config.in_out_factor = in_out_factor;
        self
    }

    pub fn positive_sampling_factor(mut self, positive_sampling_factor: f64) -> Self {
        self.config.positive_sampling_factor = positive_sampling_factor;
        self
    }

    pub fn negative_sampling_exponent(mut self, negative_sampling_exponent: f64) -> Self {
        self.config.negative_sampling_exponent = negative_sampling_exponent;
        self
    }

    pub fn initial_learning_rate(mut self, initial_learning_rate: f64) -> Self {
        self.config.initial_learning_rate = initial_learning_rate;
        self
    }

    pub fn min_learning_rate(mut self, min_learning_rate: f64) -> Self {
        self.config.min_learning_rate = min_learning_rate;
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.config.iterations = iterations;
        self
    }

    pub fn window_size(mut self, window_size: usize) -> Self {
        self.config.window_size = window_size;
        self
    }

    pub fn negative_sampling_rate(mut self, negative_sampling_rate: usize) -> Self {
        self.config.negative_sampling_rate = negative_sampling_rate;
        self
    }

    pub fn embedding_dimension(mut self, embedding_dimension: usize) -> Self {
        self.config.embedding_dimension = embedding_dimension;
        self
    }

    pub fn embedding_initializer(
        mut self,
        embedding_initializer: EmbeddingInitializerConfig,
    ) -> Self {
        self.config.embedding_initializer = embedding_initializer;
        self
    }

    pub fn source_nodes(mut self, source_nodes: Vec<i64>) -> Self {
        self.config.source_nodes = source_nodes;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn walk_buffer_size(mut self, walk_buffer_size: usize) -> Self {
        self.config.walk_buffer_size = walk_buffer_size;
        self
    }

    pub fn random_seed(mut self, random_seed: Option<u64>) -> Self {
        self.config.random_seed = random_seed;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.config.walks_per_node == 0 {
            return Err(AlgorithmError::Execution(
                "walks_per_node must be > 0".into(),
            ));
        }
        if self.config.walk_length == 0 {
            return Err(AlgorithmError::Execution("walk_length must be > 0".into()));
        }
        if self.config.embedding_dimension == 0 {
            return Err(AlgorithmError::Execution(
                "embedding_dimension must be > 0".into(),
            ));
        }
        if self.config.window_size == 0 {
            return Err(AlgorithmError::Execution("window_size must be > 0".into()));
        }
        if self.config.iterations == 0 {
            return Err(AlgorithmError::Execution("iterations must be > 0".into()));
        }

        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        ConfigValidator::in_range(
            self.config.walk_buffer_size as f64,
            1.0,
            1_000_000_000.0,
            "walk_buffer_size",
        )?;

        Ok(())
    }

    fn compute(&self) -> Result<Node2VecResult> {
        self.validate()?;

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = Node2VecStorageRuntime::new();
        Node2VecComputationRuntime::run(graph_view, &self.config, &storage)
    }

    /// Stream mode: yields `(node_id, embedding)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = Node2VecRow>>> {
        let result = self.compute()?;
        let iter = result
            .embeddings
            .into_iter()
            .enumerate()
            .map(|(node_id, embedding)| Node2VecRow {
                node_id: node_id as u64,
                embedding,
            });
        Ok(Box::new(iter))
    }

    /// Full result: returns all embeddings + loss history.
    pub fn run(&self) -> Result<Node2VecResult> {
        self.compute()
    }

    pub fn stats(&self) -> Result<Node2VecStats> {
        let start = Instant::now();
        let result = self.compute()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        let node_count = result.embeddings.len() as u64;
        let embedding_dimension = result
            .embeddings
            .first()
            .map(|v| v.len() as u64)
            .unwrap_or(0);
        let last_loss = result.loss_per_iteration.last().copied();

        Ok(Node2VecStats {
            node_count,
            embedding_dimension,
            iterations: result.loss_per_iteration.len() as u64,
            loss_per_iteration: result.loss_per_iteration,
            last_loss,
            compute_millis,
            success: true,
        })
    }

    /// Full result + a canonical print envelope (summary) emitted at the boundary.
    ///
    /// This keeps the kernel output structurally inspectable without stuffing large
    /// embedding matrices into the print payload.
    pub fn run_with_print(
        &self,
        run_id: Option<String>,
    ) -> Result<(Node2VecResult, PrintEnvelope)> {
        let result = self.compute()?;

        let node_count = result.embeddings.len();
        let embedding_dimension = result.embeddings.first().map(|v| v.len()).unwrap_or(0);
        let last_loss = result.loss_per_iteration.last().copied();

        let print = PrintEnvelope::new(
            PrintKind::Ml,
            PrintProvenance {
                source: "gds::node2vec".to_string(),
                run_id,
                kernel_version: None,
            },
            serde_json::json!({
                "algo": "node2vec",
                "node_count": node_count,
                "embedding_dimension": embedding_dimension,
                "loss_per_iteration": result.loss_per_iteration,
                "last_loss": last_loss,
                "config": {
                    "walks_per_node": self.config.walks_per_node,
                    "walk_length": self.config.walk_length,
                    "return_factor": self.config.return_factor,
                    "in_out_factor": self.config.in_out_factor,
                    "positive_sampling_factor": self.config.positive_sampling_factor,
                    "negative_sampling_exponent": self.config.negative_sampling_exponent,
                    "initial_learning_rate": self.config.initial_learning_rate,
                    "min_learning_rate": self.config.min_learning_rate,
                    "iterations": self.config.iterations,
                    "window_size": self.config.window_size,
                    "negative_sampling_rate": self.config.negative_sampling_rate,
                    "concurrency": self.config.concurrency,
                    "walk_buffer_size": self.config.walk_buffer_size,
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
            seed: Some(7),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.6)],
            directed: true,
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn facade_run_returns_expected_shape() {
        let graph = GraphFacade::new(store());

        let result = graph
            .node2vec()
            .walks_per_node(2)
            .walk_length(6)
            .iterations(1)
            .window_size(3)
            .negative_sampling_rate(1)
            .embedding_dimension(8)
            .concurrency(1)
            .random_seed(Some(42))
            .run()
            .unwrap();

        assert_eq!(result.loss_per_iteration.len(), 1);
        assert_eq!(result.embeddings.len(), 10);
        assert_eq!(result.embeddings[0].len(), 8);
    }

    #[test]
    fn facade_stream_yields_one_row_per_node() {
        let graph = GraphFacade::new(store());

        let rows: Vec<_> = graph
            .node2vec()
            .walks_per_node(1)
            .walk_length(5)
            .iterations(1)
            .window_size(2)
            .negative_sampling_rate(1)
            .embedding_dimension(4)
            .concurrency(1)
            .random_seed(Some(1))
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 10);
        assert_eq!(rows[0].embedding.len(), 4);
    }

    #[test]
    fn facade_run_with_print_emits_summary_without_embeddings() {
        let graph = GraphFacade::new(store());

        let (result, print) = graph
            .node2vec()
            .walks_per_node(1)
            .walk_length(5)
            .iterations(1)
            .window_size(2)
            .negative_sampling_rate(1)
            .embedding_dimension(4)
            .concurrency(1)
            .random_seed(Some(1))
            .run_with_print(Some("run-test-1".to_string()))
            .unwrap();

        assert_eq!(result.embeddings.len(), 10);
        assert_eq!(result.embeddings[0].len(), 4);

        let value = serde_json::to_value(&print).expect("serialize print");
        assert_eq!(value["kind"], "ml");
        assert_eq!(value["provenance"]["source"], "gds::node2vec");
        assert_eq!(value["provenance"]["run_id"], "run-test-1");
        assert_eq!(value["payload"]["algo"], "node2vec");
        assert_eq!(value["payload"]["embedding_dimension"], 4);
        assert!(value["payload"].get("embeddings").is_none());
    }
}
