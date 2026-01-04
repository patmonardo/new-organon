//! SideEffect (Java parity).

use crate::core::loading::GraphResources;

pub trait SideEffect<ResultFromAlgorithm, Metadata> {
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<&ResultFromAlgorithm>,
    ) -> Option<Metadata>;
}

pub struct SideEffectExecutor;

impl SideEffectExecutor {
    pub fn execute_side_effect<ResultFromAlgorithm, Metadata>(
        &self,
        result: Option<&ResultFromAlgorithm>,
        side_effect: impl FnOnce(&ResultFromAlgorithm) -> Metadata,
    ) -> Option<Metadata> {
        result.map(side_effect)
    }
}
