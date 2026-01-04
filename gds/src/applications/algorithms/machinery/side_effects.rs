//! Concrete side-effect wrappers (Java parity).

use crate::core::loading::GraphResources;
use crate::core::utils::progress::JobId;

use super::{MutateStep, SideEffect, SideEffectExecutor, WriteStep};

pub struct MutateSideEffect<S>(pub S);

impl<ResultFromAlgorithm, Metadata, S> SideEffect<ResultFromAlgorithm, Metadata>
    for MutateSideEffect<S>
where
    S: Fn(&GraphResources, &ResultFromAlgorithm) -> Metadata + Send + Sync,
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<&ResultFromAlgorithm>,
    ) -> Option<Metadata> {
        let executor = SideEffectExecutor;
        executor.execute_side_effect(result, |r| (self.0)(graph_resources, r))
    }
}

pub struct WriteSideEffect<S>(pub S);

impl<ResultFromAlgorithm, Metadata, S> SideEffect<ResultFromAlgorithm, Metadata>
    for WriteSideEffect<S>
where
    S: Fn(&GraphResources, &ResultFromAlgorithm) -> Metadata + Send + Sync,
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<&ResultFromAlgorithm>,
    ) -> Option<Metadata> {
        let executor = SideEffectExecutor;
        executor.execute_side_effect(result, |r| (self.0)(graph_resources, r))
    }
}

/// Java-parity: a side effect backed by a `MutateStep` object.
pub struct MutateStepSideEffect<S>(pub S);

impl<ResultFromAlgorithm, Metadata, S> SideEffect<ResultFromAlgorithm, Metadata>
    for MutateStepSideEffect<S>
where
    S: MutateStep<ResultFromAlgorithm, Metadata> + Send + Sync,
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<&ResultFromAlgorithm>,
    ) -> Option<Metadata> {
        let executor = SideEffectExecutor;
        executor.execute_side_effect(result, |r| self.0.execute(graph_resources, r))
    }
}

/// Java-parity: a side effect backed by a `WriteStep` object.
pub struct WriteStepSideEffect<S> {
    pub job_id: JobId,
    pub write_step: S,
}

impl<S> WriteStepSideEffect<S> {
    pub fn new(job_id: JobId, write_step: S) -> Self {
        Self { job_id, write_step }
    }
}

impl<ResultFromAlgorithm, Metadata, S> SideEffect<ResultFromAlgorithm, Metadata>
    for WriteStepSideEffect<S>
where
    S: WriteStep<ResultFromAlgorithm, Metadata> + Send + Sync,
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<&ResultFromAlgorithm>,
    ) -> Option<Metadata> {
        let executor = SideEffectExecutor;
        executor.execute_side_effect(result, |r| {
            let result_store = graph_resources.result_store.as_deref();
            self.write_step.execute(
                graph_resources.facade(),
                graph_resources.store().as_ref(),
                result_store,
                r,
                &self.job_id,
            )
        })
    }
}
