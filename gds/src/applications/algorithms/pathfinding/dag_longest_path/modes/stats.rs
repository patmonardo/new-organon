use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dag_longest_path::request::DagLongestPathRequest;
use crate::applications::algorithms::pathfinding::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::DagLongestPathStats;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DagLongestPathRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("DagLongestPath::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<DagLongestPathStats>, String> {
        let stats = gr
            .facade()
            .dag_longest_path()
            .concurrency(request.common.concurrency.value())
            .stats()
            .map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources,
                                       stats: Option<DagLongestPathStats>,
                                       timings| {
        json!({
            "ok": true,
            "op": op,
            "mode": "stats",
            "data": stats,
            "timings": timings_json(timings)
        })
    });

    match convenience.process_stats(graph_resources, request.common.concurrency, task, compute, builder)
    {
        Ok(v) => v,
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DagLongestPath stats failed: {e}")),
    }
}
