//! GraphSAGE facade (builder API).

use crate::prelude::DefaultGraphStore;
use crate::prelude::GraphStore;
use crate::procedures::facades::traits as facade;
use crate::projection::eval::procedure::AlgorithmError;
use std::sync::Arc;

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
        Ok(())
    }

    pub fn run(self) -> facade::Result<crate::procedures::embeddings::GraphSageResult> {
        self.validate()?;

        // Directly call the storage runtime
        let storage =
            crate::procedures::embeddings::graphsage::algo::storage::GraphSageStorageRuntime::new();
        let rel_types = std::collections::HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(
                &rel_types,
                &std::collections::HashMap::new(),
                crate::projection::orientation::Orientation::Natural,
            )
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| {
                AlgorithmError::Graph(e.to_string())
            })?;

        let graphsage_config = crate::procedures::embeddings::GraphSageConfig {
            model_user: self.model_user,
            model_name: self.model_name,
            batch_size: self.batch_size,
            concurrency: self.concurrency,
        };

        Ok(storage.compute(graph.as_ref(), &graphsage_config))
    }
}
