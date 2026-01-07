use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::bfs::request::BfsRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use serde_json::{json, Value};

pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("BFS::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<crate::procedures::pathfinding::BfsStats>, String> {
        let mut builder = gr
            .facade()
            .bfs()
            .source(request.source)
            .track_paths(request.track_paths)
            .concurrency(request.common.concurrency.value());

        if let Some(max_depth) = request.max_depth {
            builder = builder.max_depth(max_depth);
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, stats: Option<crate::procedures::pathfinding::BfsStats>, timings| {
            json!({
                "ok": true,
                "op": op,
                "mode": "stats",
                "data": stats,
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stats failed: {e}")),
    }
}
