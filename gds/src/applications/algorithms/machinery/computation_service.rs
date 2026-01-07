//! ComputationService (Java parity, simplified).
//!
//! Java reference: `ComputationService`.
//! Responsibilities:
//! - run `MemoryGuard.assertAlgorithmCanRun(...)` before compute
//! - execute computation (optionally with metrics; simplified here)

use crate::applications::services::logging::Log;
use crate::core::loading::GraphResources;
use crate::errors::MemoryEstimationError;
use crate::mem::MemoryTreeWithDimensions;

use super::{
    AlgoBaseConfigLike, Computation, DimensionTransformer, Label, MemoryGuard, MemoryGuardError,
};

pub struct ComputationService<G> {
    pub username: String,
    pub log: Log,
    pub memory_guard: G,
}

impl<G> ComputationService<G>
where
    G: MemoryGuard,
{
    pub fn new(username: String, log: Log, memory_guard: G) -> Self {
        Self {
            username,
            log,
            memory_guard,
        }
    }

    pub fn compute_algorithm<Configuration, ResultFromAlgorithm>(
        &self,
        configuration: &Configuration,
        graph_resources: &GraphResources,
        label: &dyn Label,
        estimation_supplier: impl FnOnce(
            &GraphResources,
            &Configuration,
        )
            -> Result<MemoryTreeWithDimensions, MemoryEstimationError>,
        computation: &dyn Computation<ResultFromAlgorithm>,
        dimension_transformer: &dyn DimensionTransformer,
    ) -> Result<ResultFromAlgorithm, MemoryGuardError>
    where
        Configuration: AlgoBaseConfigLike,
    {
        self.memory_guard.assert_algorithm_can_run(
            &self.username,
            estimation_supplier,
            graph_resources,
            configuration,
            label,
            dimension_transformer,
        )?;

        // Metrics gathering is intentionally simplified to keep this folder self-contained.
        // The key parity point is that guard runs before compute.
        let graph_store = graph_resources.store();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            computation.compute(graph_resources.facade(), graph_store.as_ref())
        }));

        match result {
            Ok(value) => Ok(value),
            Err(_panic) => {
                self.log
                    .warn("computation failed, halting metrics gathering");
                std::panic::resume_unwind(_panic)
            }
        }
    }
}
