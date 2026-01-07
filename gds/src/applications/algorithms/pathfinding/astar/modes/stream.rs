use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::astar::request::AStarRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::traits::PathResult;
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

    let task = Tasks::leaf("AStar::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<PathResult>>, String> {
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

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let result_builder =
        FnStreamResultBuilder::new(|_gr: &GraphResources, rows: Option<Vec<PathResult>>| {
            rows.unwrap_or_default().into_iter()
        });

    match convenience.process_stream(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        result_builder,
    ) {
        Ok(stream) => {
            let rows: Vec<PathResult> = stream.collect::<Vec<_>>();
            json!({
                "ok": true,
                "op": op,
                "mode": "stream",
                "data": rows
            })
        }
        Err(e) => err(op, "EXECUTION_ERROR", &format!("A* stream failed: {e}")),
    }
}
