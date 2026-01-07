use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::astar::request::AStarRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::AStarStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &AStarRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("AStar::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<AStarStats>, String> {
        // Use facade instead of direct algo calls
        use crate::procedures::pathfinding::Heuristic;
        let mut builder = gr
            .facade()
            .astar()
            .source(request.source)
            .target(request.target)
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .heuristic(Heuristic::Haversine) // Use Haversine for geographic routing
            .concurrency(request.common.concurrency.value());

        if !request.relationship_types.is_empty() {
            builder = builder.relationship_types(request.relationship_types.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let result_builder = FnStatsResultBuilder(
        |_gr: &GraphResources,
         stats: Option<AStarStats>,
         timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
            json!({
                "nodes_visited": stats.as_ref().map(|s| s.nodes_visited).unwrap_or(0),
                "final_queue_size": stats.as_ref().map(|s| s.final_queue_size).unwrap_or(0),
                "max_queue_size": stats.as_ref().map(|s| s.max_queue_size).unwrap_or(0),
                "execution_time_ms": stats.as_ref().map(|s| s.execution_time_ms).unwrap_or(0),
                "targets_found": stats.as_ref().map(|s| s.targets_found).unwrap_or(0),
                "all_targets_reached": stats.as_ref().map(|s| s.all_targets_reached).unwrap_or(false),
                "heuristic_accuracy": stats.as_ref().map(|s| s.heuristic_accuracy).unwrap_or(1.0),
                "heuristic_evaluations": stats.as_ref().map(|s| s.heuristic_evaluations).unwrap_or(0),
                "pre_processing_time_ms": timings.pre_processing_millis,
                "post_processing_time_ms": timings.side_effect_millis,
            })
        },
    );

    match convenience.process_stats(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        result_builder,
    ) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("A* stats failed: {e}")),
    }
}
