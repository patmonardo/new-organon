use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::procedures::pathfinding::DijkstraStats;
use crate::applications::algorithms::pathfinding::dijkstra::request::DijkstraRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
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
        // Use facade instead of direct algo calls
        let mut builder = gr.facade().dijkstra()
            .source(request.source)
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .track_relationships(request.track_relationships)
            .concurrency(request.common.concurrency.value());

        if !request.targets.is_empty() {
            builder = builder.targets(request.targets.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let result_builder = FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<DijkstraStats>, timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
        json!({
            "paths_found": stats.as_ref().map(|s| s.paths_found).unwrap_or(0),
            "execution_time_ms": stats.as_ref().map(|s| s.execution_time_ms).unwrap_or(0),
            "nodes_expanded": stats.as_ref().map(|s| s.nodes_expanded).unwrap_or(0),
            "edges_considered": stats.as_ref().map(|s| s.edges_considered).unwrap_or(0),
            "max_queue_size": stats.as_ref().map(|s| s.max_queue_size).unwrap_or(0),
            "target_reached": stats.as_ref().map(|s| s.target_reached).unwrap_or(false),
            "pre_processing_time_ms": timings.pre_processing_millis,
            "post_processing_time_ms": timings.side_effect_millis,
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, result_builder) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Dijkstra stats failed: {e}")),
    }
}
