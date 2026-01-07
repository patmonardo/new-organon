use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dfs::request::DfsRequest;
use crate::applications::algorithms::pathfinding::dfs::steps::mutate_step::{
    DfsMutateOutcome, DfsMutateStep,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::traits::PathResult;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(
    op: &str,
    request: &DfsRequest,
    catalog: Arc<dyn GraphCatalog>,
    graph_resources: &GraphResources,
) -> Value {
    let property_name = request
        .property_name
        .clone()
        .unwrap_or_else(|| "dfsDepth".to_string());

    let output_graph_name = request
        .output_graph_name
        .clone()
        .unwrap_or_else(|| request.common.graph_name.clone());

    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("DFS::mutate".to_string()).base().clone();

    let compute = move |gr: &GraphResources,
                        _tracker: &mut dyn ProgressTracker,
                        _termination: &TerminationFlag|
          -> Result<Option<DfsMutateOutcome>, String> {
        let mut builder = gr.facade().dfs().source(request.source);
        if !request.targets.is_empty() {
            builder = builder.targets(request.targets.clone());
        }
        if let Some(max_depth) = request.max_depth {
            builder = builder.max_depth(max_depth);
        }
        builder = builder.track_paths(request.track_paths);
        builder = builder.concurrency(request.common.concurrency.value());

        let iter = builder.stream().map_err(|e| e.to_string())?;
        let rows: Vec<PathResult> = iter.collect();

        let step = DfsMutateStep {
            catalog,
            output_graph_name,
            property_name,
        };
        let outcome = step.execute(gr, &rows)?;
        Ok(Some(outcome))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, outcome: Option<DfsMutateOutcome>, timings| {
            json!({
                "ok": true,
                "op": op,
                "mode": "mutate",
                "data": outcome,
                "timings": timings_json(timings)
            })
        },
    );

    match convenience.process_stats(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        builder,
    ) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DFS mutate failed: {e}")),
    }
}
