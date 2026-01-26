//! GraphSAGE facade (builder API).

use crate::algo::algorithms::traits as facade;
use crate::algo::algorithms::ConfigValidator;
use crate::algo::embeddings::graphsage::spec::{GraphSageConfig, GraphSageResult};
use crate::algo::embeddings::graphsage::GraphSageStorageRuntime;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::types::DefaultGraphStore;
use crate::types::GraphStore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSageStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// GraphSAGE builder for inference/embeddings generation.
#[derive(Clone)]
pub struct GraphSageBuilder {
    graph_store: Arc<DefaultGraphStore>,
    model_user: String,
    model_name: String,
    batch_size: usize,
    concurrency: usize,
}

impl GraphSageBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            model_user: "anonymous".to_string(),
            model_name: "".to_string(),
            batch_size: 100,
            concurrency: num_cpus::get().max(1),
        }
    }

    pub fn model_user(mut self, user: impl Into<String>) -> Self {
        self.model_user = user.into();
        self
    }

    pub fn model_name(mut self, name: impl Into<String>) -> Self {
        self.model_name = name.into();
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn validate(&self) -> facade::Result<()> {
        if self.model_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "modelName must be specified".to_string(),
            ));
        }

        ConfigValidator::in_range(self.batch_size as f64, 1.0, 1_000_000_000.0, "batch_size")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    pub fn run(self) -> facade::Result<GraphSageResult> {
        self.validate()?;

        // Directly call the storage runtime
        let storage = GraphSageStorageRuntime::new();
        let rel_types = std::collections::HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(
                &rel_types,
                &std::collections::HashMap::new(),
                Orientation::Natural,
            )
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| {
                AlgorithmError::Graph(e.to_string())
            })?;

        let graphsage_config = GraphSageConfig {
            model_user: self.model_user,
            model_name: self.model_name,
            batch_size: self.batch_size,
            concurrency: self.concurrency,
        };

        Ok(storage.compute(graph.as_ref(), &graphsage_config))
    }

    pub fn stats(self) -> facade::Result<GraphSageStats> {
        let start = Instant::now();
        let result = self.run()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        Ok(GraphSageStats {
            node_count: result.node_count as u64,
            embedding_dimension: result.embedding_dimension as u64,
            compute_millis,
            success: true,
        })
    }
}
