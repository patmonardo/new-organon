use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::procedures::pathfinding::DfsStats;
use crate::applications::algorithms::pathfinding::dfs::request::DfsRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
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

    let task = Tasks::leaf("DFS::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<DfsStats>, String> {
        let mut builder = gr.facade().dfs()
            .source(request.source)
            .track_paths(request.track_paths)
            .concurrency(request.common.concurrency.value());

        if let Some(max_depth) = request.max_depth {
            builder = builder.max_depth(max_depth);
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let result_builder = FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<DfsStats>, timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
        json!({
            "nodes_visited": stats.as_ref().map(|s| s.nodes_visited).unwrap_or(0),
            "max_depth_reached": stats.as_ref().map(|s| s.max_depth_reached).unwrap_or(0),
            "execution_time_ms": stats.as_ref().map(|s| s.execution_time_ms).unwrap_or(0),
            "targets_found": stats.as_ref().map(|s| s.targets_found).unwrap_or(0),
            "all_targets_reached": stats.as_ref().map(|s| s.all_targets_reached).unwrap_or(false),
            "backtrack_operations": stats.as_ref().map(|s| s.backtrack_operations).unwrap_or(0),
            "avg_branch_depth": stats.as_ref().map(|s| s.avg_branch_depth).unwrap_or(0.0),
            "pre_processing_time_ms": timings.pre_processing_millis,
            "post_processing_time_ms": timings.side_effect_millis,
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, result_builder) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DFS stats failed: {e}")),
    }
}
