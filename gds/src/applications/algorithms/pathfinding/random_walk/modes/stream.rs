use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::random_walk::request::RandomWalkRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::RandomWalkRow;
use serde_json::{json, Value};

pub fn run(op: &str, request: &RandomWalkRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("RandomWalk::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<RandomWalkRow>>, String> {
        let builder = gr
            .facade()
            .random_walk()
            .walks_per_node(request.walks_per_node)
            .walk_length(request.walk_length)
            .return_factor(request.return_factor)
            .in_out_factor(request.in_out_factor)
            .source_nodes(request.source_nodes.clone())
            .concurrency(request.common.concurrency.value());

        let builder = match request.random_seed {
            Some(seed) => builder.random_seed(seed),
            None => builder,
        };

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder =
        FnStreamResultBuilder::new(|_gr: &GraphResources, rows: Option<Vec<RandomWalkRow>>| {
            rows.unwrap_or_default().into_iter()
        });

    match convenience.process_stream(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        builder,
    ) {
        Ok(stream) => {
            let rows: Vec<RandomWalkRow> = stream.collect();
            json!({
                "ok": true,
                "op": op,
                "mode": "stream",
                "data": rows,
                "timings": json!({
                    "pre_processing_millis": 0,
                    "compute_millis": 0,
                    "side_effect_millis": 0
                })
            })
        }
        Err(e) => err(
            op,
            "EXECUTION_ERROR",
            &format!("RandomWalk stream failed: {e}"),
        ),
    }
}
