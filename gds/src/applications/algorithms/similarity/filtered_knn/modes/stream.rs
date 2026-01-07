use crate::algo::similarity::filtered_knn::FilteredKnnResultRow;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::filtered_knn::request::FilteredKnnRequest;
use crate::applications::algorithms::similarity::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::filtered_knn::FilteredKnnBuilder;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(op: &str, request: &FilteredKnnRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("FilteredKNN::stream".to_string())
        .base()
        .clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<FilteredKnnResultRow>>, String> {
        let primary = &request.node_properties[0];
        let mut builder = FilteredKnnBuilder::new(Arc::clone(gr.store()), primary.name.clone())
            .k(request.top_k)
            .similarity_cutoff(request.similarity_cutoff)
            .metric(primary.metric)
            .concurrency(request.common.concurrency.value())
            .source_labels(request.source_node_labels.clone())
            .target_labels(request.target_node_labels.clone());

        if request.node_properties.len() > 1 {
            for prop in request.node_properties.iter().skip(1) {
                builder = builder.add_property(prop.name.clone(), prop.metric);
            }
        }

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, rows: Option<Vec<FilteredKnnResultRow>>, timings| {
            json!({
                "ok": true,
                "op": op,
                "mode": "stream",
                "data": rows.unwrap_or_default(),
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
            &format!("FilteredKNN stream failed: {e}"),
        ),
    }
}
