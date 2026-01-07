use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::node_similarity::request::NodeSimilarityRequest;
use crate::applications::algorithms::similarity::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::algo::similarity::NodeSimilarityResult;
use crate::procedures::similarity::NodeSimilarityBuilder;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(op: &str, request: &NodeSimilarityRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("NodeSimilarity::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<NodeSimilarityResult>>, String> {
        let mut builder = NodeSimilarityBuilder::new(Arc::clone(gr.store()))
            .metric(request.metric)
            .similarity_cutoff(request.similarity_cutoff)
            .top_k(request.top_k)
            .top_n(request.top_n)
            .concurrency(request.common.concurrency.value());

        if let Some(ref prop) = request.weight_property {
            builder = builder.weight_property(prop.clone());
        }

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources,
                                       rows: Option<Vec<NodeSimilarityResult>>,
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("NodeSimilarity stream failed: {e}")),
    }
}
