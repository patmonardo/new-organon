use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate, FnStatsResultBuilder,
    ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::applications::algorithms::pathfinding::steiner_tree::request::SteinerTreeRequest;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::SteinerTreeRow;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SteinerTreeRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("SteinerTree::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<SteinerTreeRow>>, String> {
        let mut builder = gr
            .facade()
            .steiner_tree()
            .source_node(request.source_node)
            .target_nodes(request.target_nodes.clone())
            .delta(request.delta)
            .apply_rerouting(request.apply_rerouting)
            .concurrency(request.common.concurrency.value());

        if let Some(ref prop) = request.relationship_weight_property {
            builder = builder.relationship_weight_property(prop);
        }

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources,
                                       rows: Option<Vec<SteinerTreeRow>>,
                                       timings| {
        json!({
            "ok": true,
            "op": op,
            "mode": "stream",
            "data": rows.unwrap_or_default(),
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("SteinerTree stream failed: {e}")),
    }
}
