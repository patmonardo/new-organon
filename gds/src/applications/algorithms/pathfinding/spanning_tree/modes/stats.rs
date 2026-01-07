use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::applications::algorithms::pathfinding::spanning_tree::request::SpanningTreeRequest;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::SpanningTreeStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("SpanningTree::stats".to_string())
        .base()
        .clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<SpanningTreeStats>, String> {
        // Use facade instead of direct algo calls
        let mut builder = gr
            .facade()
            .spanning_tree()
            .start_node(request.start_node)
            .compute_minimum(request.compute_minimum)
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .concurrency(request.common.concurrency.value());

        if !request.relationship_types.is_empty() {
            builder = builder.relationship_types(request.relationship_types.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources,
         stats: Option<SpanningTreeStats>,
         timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings| {
            let stats = stats.unwrap_or(SpanningTreeStats {
                effective_node_count: 0,
                total_weight: 0.0,
                computation_time_ms: timings.compute_millis as u64,
            });

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
        Err(e) => err(
            op,
            "EXECUTION_ERROR",
            &format!("SpanningTree stats failed: {e}"),
        ),
    }
}
