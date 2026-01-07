use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::random_walk::request::RandomWalkRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::RandomWalkStats;
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

    let task = Tasks::leaf("RandomWalk::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<RandomWalkStats>, String> {
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

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, stats: Option<RandomWalkStats>, timings| {
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
            &format!("RandomWalk stats failed: {e}"),
        ),
    }
}
