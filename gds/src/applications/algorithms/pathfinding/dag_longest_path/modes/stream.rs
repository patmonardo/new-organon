use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::dag_longest_path::request::DagLongestPathRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::DagLongestPathRow;
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

    let task = Tasks::leaf("DagLongestPath::stream".to_string())
        .base()
        .clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<DagLongestPathRow>>, String> {
        let iter = gr
            .facade()
            .dag_longest_path()
            .concurrency(request.common.concurrency.value())
            .stream()
            .map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let result_builder = FnStreamResultBuilder::new(
        |_gr: &GraphResources, rows: Option<Vec<DagLongestPathRow>>| {
            rows.unwrap_or_default().into_iter()
        },
    );

    match convenience.process_stream(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        result_builder,
    ) {
        Ok(stream) => {
            let rows: Vec<DagLongestPathRow> = stream.collect();
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
            &format!("DagLongestPath stream failed: {e}"),
        ),
    }
}
