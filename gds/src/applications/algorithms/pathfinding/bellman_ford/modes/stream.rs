use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::bellman_ford::request::BellmanFordRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::traits::PathResult;
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

    let task = Tasks::leaf("BellmanFord::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<PathResult>>, String> {
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

        let iter = builder.stream().map_err(|e| e.to_string())?;
        let rows: Vec<PathResult> = iter.collect();
        Ok(Some(rows))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources, rows: Option<Vec<PathResult>>, timings| {
        json!({
            "ok": true,
            "op": op,
            "mode": "stream",
            "data": rows.unwrap_or_default(),
            "timings": timings_json(timings)
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, builder)
    {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Bellman-Ford stream failed: {e}")),
    }
}
