//! MutateStep / WriteStep (Java parity).

use crate::applications::graph_store_catalog::loaders::ResultStore;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::JobId;
use crate::procedures::GraphFacade;
use crate::types::graph_store::DefaultGraphStore;

pub trait MutateStep<ResultFromAlgorithm, Metadata> {
    fn execute(&self, graph_resources: &GraphResources, result: &ResultFromAlgorithm) -> Metadata;
}

pub trait WriteStep<ResultFromAlgorithm, Metadata> {
    /// Java parity: timings belong on the outside.
    fn execute(
        &self,
        graph: &GraphFacade,
        graph_store: &DefaultGraphStore,
        result_store: Option<&dyn ResultStore>,
        result: &ResultFromAlgorithm,
        job_id: &JobId,
    ) -> Metadata;
}
