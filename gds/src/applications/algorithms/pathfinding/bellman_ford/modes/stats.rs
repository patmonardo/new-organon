use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::bellman_ford::request::BellmanFordRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::BellmanFordStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &BellmanFordRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("BellmanFord::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<BellmanFordStats>, String> {
        // Use facade instead of direct algo calls
        let mut builder = gr
            .facade()
            .bellman_ford()
            .source(request.source)
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .track_negative_cycles(request.track_negative_cycles)
            .track_paths(request.track_paths)
            .concurrency(request.common.concurrency.value());

        if !request.relationship_types.is_empty() {
            builder = builder.relationship_types(request.relationship_types.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let result_builder = FnStatsResultBuilder(
        |_gr: &GraphResources,
         stats: Option<BellmanFordStats>,
         timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
            json!({
                "paths_found": stats.as_ref().map(|s| s.paths_found).unwrap_or(0),
                "negative_cycles_found": stats.as_ref().map(|s| s.negative_cycles_found).unwrap_or(0),
                "contains_negative_cycle": stats.as_ref().map(|s| s.contains_negative_cycle).unwrap_or(false),
                "execution_time_ms": stats.as_ref().map(|s| s.execution_time_ms).unwrap_or(0),
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
        Err(e) => err(
            op,
            "EXECUTION_ERROR",
            &format!("Bellman-Ford stats failed: {e}"),
        ),
    }
}
