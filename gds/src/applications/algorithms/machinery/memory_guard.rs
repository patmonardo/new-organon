use std::sync::Arc;

use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use crate::applications::algorithms::machinery::{AlgorithmLabel, DimensionTransformer};
use crate::config::base_types::Config;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::core::utils::progress::JobId;
use crate::mem::{MemoryEstimation, MemoryTracker};
use crate::applications::services::logging::Log;
use crate::core::GraphDimensions;

/// Memory Guard - memory protection and validation
/// This is just memory guarding. Do not conflate with UI concerns.
pub trait MemoryGuard {
    /// This could be handy for tests
    fn assert_algorithm_can_run(
        &self,
        username: &str,
        estimation_factory: &dyn Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &Arc<DefaultGraphStore>,
        configuration: &dyn Config,
        label: &AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
    ) -> Result<(), MemoryGuardError>;
}

/// Memory Guard Error
#[derive(Debug)]
pub enum MemoryGuardError {
    InsufficientMemory { required: u64, available: u64 },
    EstimationNotImplemented,
    Other(String),
}

impl std::fmt::Display for MemoryGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryGuardError::InsufficientMemory { required, available } => {
                write!(f, "Memory required ({}) exceeds available memory ({})", required, available)
            }
            MemoryGuardError::EstimationNotImplemented => {
                write!(f, "Memory estimation not implemented")
            }
            MemoryGuardError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MemoryGuardError {}

/// Default Memory Guard implementation
pub struct DefaultMemoryGuard {
    log: Log,
    use_max_memory_estimation: bool,
    memory_tracker: MemoryTracker,
}

impl DefaultMemoryGuard {
    pub fn new(
        log: Log,
        use_max_memory_estimation: bool,
        memory_tracker: MemoryTracker,
    ) -> Self {
        Self {
            log,
            use_max_memory_estimation,
            memory_tracker,
        }
    }

    pub fn create(
        log: Log,
        use_max_memory_estimation: bool,
        memory_tracker: MemoryTracker,
    ) -> Self {
        Self::new(log, use_max_memory_estimation, memory_tracker)
    }
}

impl MemoryGuard for DefaultMemoryGuard {
    fn assert_algorithm_can_run(
        &self,
        username: &str,
        estimation_factory: &dyn Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &Arc<DefaultGraphStore>,
        configuration: &dyn Config,
        label: &AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
    ) -> Result<(), MemoryGuardError> {
        match MemoryRequirement::create(
            estimation_factory,
            graph_store,
            dimension_transformer,
            configuration,
            self.use_max_memory_estimation,
        ) {
            Ok(memory_requirement) => {
                let bytes_to_reserve = memory_requirement.required_memory();

                // Java parity placeholder: we don't yet have job-id on the shared Config traits.
                // Use the empty job id for now.
                let job_id = JobId::EMPTY;
                self.memory_tracker
                    .try_to_track(username, label.as_string(), &job_id, bytes_to_reserve)
                    .map_err(|e| MemoryGuardError::InsufficientMemory {
                        required: e.bytes_required(),
                        available: e.bytes_available(),
                    })
            }
            Err(MemoryGuardError::EstimationNotImplemented) => {
                self.log.info(&format!("Memory usage estimate not available for {}, skipping guard", label.as_string()));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

/// Memory Requirement - represents memory requirements for an algorithm
pub struct MemoryRequirement {
    pub required_memory: u64,
}

impl MemoryRequirement {
    pub fn new(required_memory: u64) -> Self {
        Self { required_memory }
    }

    pub fn required_memory(&self) -> u64 {
        self.required_memory
    }

    pub fn create(
        estimation_factory: &dyn Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &Arc<DefaultGraphStore>,
        dimension_transformer: Box<dyn DimensionTransformer>,
        configuration: &dyn Config,
        use_max_memory_estimation: bool,
    ) -> Result<Self, MemoryGuardError> {
        let memory_estimation = estimation_factory();

        // Minimal, correct dimensions source for new Applications: use GraphStore counts.
        // (This keeps us out of placeholder-land and matches the mem subsystem expectations.)
        let graph_dimensions: Box<dyn GraphDimensions> = Box::new(ConcreteGraphDimensions::of(
            graph_store.node_count(),
            graph_store.relationship_count(),
        ));
        let transformed_graph_dimensions = dimension_transformer.transform(graph_dimensions);

        // Until Config exposes concurrency in a shared trait, default to 1.
        let concurrency = 1usize;
        let memory_tree = memory_estimation.estimate(transformed_graph_dimensions.as_ref(), concurrency);
        let memory_range = memory_tree.memory_usage();

        let bytes_required = if use_max_memory_estimation {
            memory_range.max() as u64
        } else {
            memory_range.min() as u64
        };

        let _ = configuration;
        Ok(Self::new(bytes_required))
    }
}
