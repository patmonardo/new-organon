use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::kspanningtree::request::KSpanningTreeRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::pathfinding::KSpanningTreeRow;
use serde_json::{json, Value};

pub fn run(op: &str, request: &KSpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("KSpanningTree::stream".to_string())
        .base()
        .clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<KSpanningTreeRow>>, String> {
        let mut builder = gr
            .facade()
            .kspanning_tree()
            .source_node(request.source_node)
            .k(request.k)
            .objective(&request.objective);

        if let Some(ref prop) = request.weight_property {
            builder = builder.weight_property(prop);
        }

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder = FnStreamResultBuilder::new(
        |_gr: &GraphResources, rows: Option<Vec<KSpanningTreeRow>>| {
            rows.unwrap_or_default().into_iter()
        },
    );

    match convenience.process_stream(
        graph_resources,
        request.common.concurrency,
        task,
        compute,
        builder,
    ) {
        Ok(stream) => {
            let rows: Vec<KSpanningTreeRow> = stream.collect();
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
            &format!("KSpanningTree stream failed: {e}"),
        ),
    }
}
