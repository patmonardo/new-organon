use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dijkstra::request::DijkstraRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::DijkstraStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DijkstraRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("Dijkstra::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<DijkstraStats>, String> {
        let mut builder = gr.facade().dijkstra().source(request.source);
        if !request.targets.is_empty() {
            builder = builder.targets(request.targets.clone());
        }
        builder = builder
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .track_relationships(request.track_relationships)
            .concurrency(request.common.concurrency.value());

        let stats: DijkstraStats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<DijkstraStats>, timings| {
        json!({
            "ok": true,
            "op": op,
            "mode": "stats",
            "data": stats,
            "timings": timings_json(timings)
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, builder)
    {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Dijkstra stats failed: {e}")),
    }
}
