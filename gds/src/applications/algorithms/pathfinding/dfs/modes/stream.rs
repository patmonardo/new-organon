use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dfs::request::DfsRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::traits::PathResult;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DfsRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("DFS::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<PathResult>>, String> {
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
        Ok(Some(rows))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources, rows: Option<Vec<PathResult>>, timings| {
        let rows = rows.unwrap_or_default();
        json!({
            "ok": true,
            "op": op,
            "mode": "stream",
            "data": rows,
            "timings": timings_json(timings)
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, builder) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DFS stream failed: {e}")),
    }
}
