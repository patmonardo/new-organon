use std::sync::Arc;

use crate::procedures::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::applications::algorithms::machinery::{
    AlgorithmLabel, MemoryGuard, DimensionTransformer,
};
use crate::config::base_types::Config;
use crate::mem::MemoryEstimation;
use crate::core::loading::GraphResources;
use crate::applications::services::logging::Log;

/// The framework hook for all algorithm computations.
/// This is the lowest common denominator of things algorithm computations need.
pub trait Computation<RESULT> {
    /// The lowest common denominator of things algorithm computations need
    ///
    /// # Arguments
    /// * `graph` - most algorithms need this
    /// * `graph_store` - very few algorithms need this
    fn compute(&self, graph: Graph, graph_store: Arc<DefaultGraphStore>) -> RESULT;
}

/// Computation Service - encapsulates computing stuff with memory guard and metrics.
/// This encapsulates computing stuff with memory guard and metrics.
/// _Could_ be algorithms, could be something else.
pub struct ComputationService {
    _log: Log,
    memory_guard: Box<dyn MemoryGuard>,
    algorithm_metrics_service: Box<dyn AlgorithmMetricsService>,
    username: String,
}

impl ComputationService {
    pub fn new(
        username: String,
        log: Log,
        memory_guard: Box<dyn MemoryGuard>,
        algorithm_metrics_service: Box<dyn AlgorithmMetricsService>,
    ) -> Self {
        Self {
            _log: log,
            memory_guard,
            algorithm_metrics_service,
            username,
        }
    }

    pub fn compute_algorithm<ConfigT: Config, ResultFromAlgorithm>(
        &self,
        configuration: ConfigT,
        graph_resources: GraphResources,
        label: AlgorithmLabel,
        estimation_supplier: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: Box<dyn Computation<ResultFromAlgorithm>>,
        dimension_transformer: Box<dyn DimensionTransformer>,
    ) -> ResultFromAlgorithm {
        let _ = self.memory_guard.assert_algorithm_can_run(
            &self.username,
            &estimation_supplier,
            &graph_resources.graph_store,
            &configuration,
            &label,
            dimension_transformer,
        );

        self.compute_with_metrics(graph_resources, label, computation)
    }

    fn compute_with_metrics<ResultFromAlgorithm>(
        &self,
        graph_resources: GraphResources,
        label: AlgorithmLabel,
        computation: Box<dyn Computation<ResultFromAlgorithm>>,
    ) -> ResultFromAlgorithm {
        let _execution_metric = self.algorithm_metrics_service.create(label.as_string());

        // TODO: Implement proper metrics tracking
        // try (executionMetric) {
        //     executionMetric.start();
        //     return computation.compute(graphResources.graph(), graphResources.graphStore());
        // } catch (RuntimeException e) {
        //     log.warn("computation failed, halting metrics gathering", e);
        //     executionMetric.failed(e);
        //     throw e;
        // }

        computation.compute(graph_resources.graph, graph_resources.graph_store)
    }
}

// Placeholder for AlgorithmMetricsService
pub trait AlgorithmMetricsService {
    fn create(&self, name: &str) -> Box<dyn ExecutionMetric>;
}

// Placeholder for ExecutionMetric
pub trait ExecutionMetric {
    fn start(&mut self);
    fn failed(&mut self, error: Box<dyn std::error::Error>);
}
