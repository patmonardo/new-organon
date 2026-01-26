//! IndirectExposure procedure facade.
//!
//! Validates config, builds the storage/computation runtimes, and returns
//! exposure metrics computed via Pregel (no graph mutation).

use crate::algo::algorithms::Result;
use crate::algo::indirect_exposure::{
    IndirectExposureConfig, IndirectExposureResult, IndirectExposureStorageRuntime,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct IndirectExposureFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: IndirectExposureConfig,
}

impl IndirectExposureFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: IndirectExposureConfig::default(),
        }
    }

    pub fn sanctioned_property(mut self, property: impl Into<String>) -> Self {
        self.config.sanctioned_property = property.into();
        self
    }

    pub fn relationship_weight_property(mut self, property: Option<impl Into<String>>) -> Self {
        self.config.relationship_weight_property = property.map(|p| p.into());
        self
    }

    pub fn max_iterations(mut self, iterations: usize) -> Self {
        self.config.max_iterations = iterations;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    fn validate_config(&self, config: &IndirectExposureConfig) -> Result<()> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    /// Execute the indirect exposure computation and return metrics per node.
    pub fn stats(&self) -> Result<IndirectExposureResult> {
        self.validate_config(&self.config)?;
        let storage = IndirectExposureStorageRuntime::with_default_projection(
            self.graph_store.as_ref(),
            self.config.relationship_weight_property.clone(),
        )
        .map_err(AlgorithmError::Execution)?;

        storage.run(&self.config).map_err(AlgorithmError::Execution)
    }
}
