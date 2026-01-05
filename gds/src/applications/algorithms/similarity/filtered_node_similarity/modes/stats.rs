use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::filtered_node_similarity::request::FilteredNodeSimilarityRequest;
use crate::applications::algorithms::similarity::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::filtered_node_similarity::{
    FilteredNodeSimilarityBuilder, FilteredNodeSimilarityStats,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(
    op: &str,
    request: &FilteredNodeSimilarityRequest,
    graph_resources: &GraphResources,
) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("FilteredNodeSimilarity::stats".to_string())
        .base()
        .clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<FilteredNodeSimilarityStats>, String> {
        let mut builder = FilteredNodeSimilarityBuilder::new(Arc::clone(gr.store()))
            .metric(request.metric)
            .similarity_cutoff(request.similarity_cutoff)
            .top_k(request.top_k)
            .top_n(request.top_n)
            .concurrency(request.common.concurrency.value());

        if let Some(ref prop) = request.weight_property {
            builder = builder.weight_property(prop.clone());
        }
        if let Some(ref label) = request.source_node_label {
            builder = builder.source_node_label(label.clone());
        }
        if let Some(ref label) = request.target_node_label {
            builder = builder.target_node_label(label.clone());
        }

        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, stats: Option<FilteredNodeSimilarityStats>, timings| {
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
            &format!("FilteredNodeSimilarity stats failed: {e}"),
        ),
    }
}
