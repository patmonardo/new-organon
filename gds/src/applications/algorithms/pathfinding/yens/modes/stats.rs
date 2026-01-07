use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::applications::algorithms::pathfinding::yens::request::YensRequest;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::YensStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &YensRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("Yens::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<YensStats>, String> {
        let mut builder = gr
            .facade()
            .yens()
            .source(request.source)
            .target(request.target)
            .k(request.k)
            .weight_property(&request.weight_property)
            .direction(&request.direction)
            .concurrency(request.common.concurrency.value());

        if !request.relationship_types.is_empty() {
            builder = builder.relationship_types(request.relationship_types.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder =
        FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<YensStats>, timings| {
            json!({
                "ok": true,
                "op": op,
                "mode": "stats",
                "data": stats,
                "timings": timings_json(timings)
            })
        });

    match convenience.process_stats(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        builder,
    ) {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("Yen's stats failed: {e}")),
    }
}
