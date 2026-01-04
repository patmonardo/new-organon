use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dijkstra::request::DijkstraRequest;
use crate::applications::algorithms::pathfinding::dijkstra::steps::write_step::{
    DijkstraWriteOutcome, DijkstraWriteStep,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::traits::PathResult;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(
    op: &str,
    request: &DijkstraRequest,
    catalog: Arc<dyn GraphCatalog>,
    graph_resources: &GraphResources,
) -> Value {
    let property_name = request
        .property_name
        .clone()
        .unwrap_or_else(|| "dijkstraDistance".to_string());

    let target_graph_name = request
        .output_graph_name
        .clone()
        .unwrap_or_else(|| request.common.graph_name.clone());

    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("Dijkstra::write".to_string()).base().clone();

    let compute = move |gr: &GraphResources,
                        _tracker: &mut dyn ProgressTracker,
                        _termination: &TerminationFlag|
     -> Result<Option<DijkstraWriteOutcome>, String> {
        let mut builder = gr.facade().dijkstra().source(request.source);
        if !request.targets.is_empty() {
            builder = builder.targets(request.targets.clone());
        }
        builder = builder
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .track_relationships(request.track_relationships)
            .concurrency(request.common.concurrency.value());

        let iter = builder.stream().map_err(|e| e.to_string())?;
        let rows: Vec<PathResult> = iter.collect();

        let step = DijkstraWriteStep {
            catalog,
            target_graph_name,
            property_name,
            source: request.source,
        };
        let outcome = step.execute(gr, &rows)?;
        Ok(Some(outcome))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, outcome: Option<DijkstraWriteOutcome>, timings| {
            json!({
                "ok": true,
                "op": op,
                "mode": "write",
                "data": outcome,
                "timings": timings_json(timings)
            })
        },
    );

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, builder)
    {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Dijkstra write failed: {e}")),
    }
}
