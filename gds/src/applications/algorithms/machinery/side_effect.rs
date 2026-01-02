use crate::core::loading::GraphResources;
use crate::applications::algorithms::machinery::{MutateStep, WriteStep};
use crate::core::utils::progress::JobId;

/// Side Effect - represents side effects that can be applied after algorithm execution
pub trait SideEffect<ResultFromAlgorithm, Metadata> {
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
    ) -> Option<Metadata>;
}

/// Side Effect Executor - reusable boilerplate for executing side effects
pub struct SideEffectExecutor;

impl SideEffectExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Reusable boilerplate for executing side effects
    pub fn execute_side_effect<ResultFromAlgorithm, Metadata>(
        &self,
        result: Option<ResultFromAlgorithm>,
        side_effect: impl Fn(ResultFromAlgorithm) -> Metadata,
    ) -> Option<Metadata> {
        if result.is_none() {
            return None;
        }

        Some(side_effect(result.unwrap()))
    }
}

impl Default for SideEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutate Side Effect - wraps a MutateStep as a SideEffect
pub struct MutateSideEffect<ResultFromAlgorithm, MutateMetadata> {
    side_effect_executor: SideEffectExecutor,
    mutate_step: Box<dyn MutateStep<ResultFromAlgorithm, MutateMetadata>>,
}

impl<ResultFromAlgorithm, MutateMetadata> MutateSideEffect<ResultFromAlgorithm, MutateMetadata> {
    pub fn new(mutate_step: Box<dyn MutateStep<ResultFromAlgorithm, MutateMetadata>>) -> Self {
        Self {
            side_effect_executor: SideEffectExecutor::new(),
            mutate_step,
        }
    }
}

impl<ResultFromAlgorithm, MutateMetadata> SideEffect<ResultFromAlgorithm, MutateMetadata>
    for MutateSideEffect<ResultFromAlgorithm, MutateMetadata>
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
    ) -> Option<MutateMetadata> {
        self.side_effect_executor.execute_side_effect(result, |r| {
            self.mutate_step.execute(
                graph_resources.graph.clone(),
                graph_resources.graph_store.clone(),
                r,
            )
        })
    }
}

/// Write Side Effect - wraps a WriteStep as a SideEffect
pub struct WriteSideEffect<ResultFromAlgorithm, WriteMetadata> {
    side_effect_executor: SideEffectExecutor,
    job_id: JobId,
    write_step: Box<dyn WriteStep<ResultFromAlgorithm, WriteMetadata>>,
}

impl<ResultFromAlgorithm, WriteMetadata> WriteSideEffect<ResultFromAlgorithm, WriteMetadata> {
    pub fn new(
        job_id: JobId,
        write_step: Box<dyn WriteStep<ResultFromAlgorithm, WriteMetadata>>,
    ) -> Self {
        Self {
            side_effect_executor: SideEffectExecutor::new(),
            job_id,
            write_step,
        }
    }
}

impl<ResultFromAlgorithm, WriteMetadata> SideEffect<ResultFromAlgorithm, WriteMetadata>
    for WriteSideEffect<ResultFromAlgorithm, WriteMetadata>
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
    ) -> Option<WriteMetadata> {
        self.side_effect_executor.execute_side_effect(result, |r| {
            self.write_step.execute(
                graph_resources.graph.clone(),
                graph_resources.graph_store.clone(),
                graph_resources
                    .result_store
                    .as_ref()
                    .map(|rs| rs.as_ref()),
                r,
                self.job_id.clone(),
            )
        })
    }
}
