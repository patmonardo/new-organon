use crate::algo::similarity::knn::KnnResultRow;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::knn::request::KnnRequest;
use crate::applications::algorithms::similarity::shared::{err, timings_json};
use crate::concurrency::TerminationFlag;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::knn::KnnBuilder;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(op: &str, request: &KnnRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("KNN::stream".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<Vec<KnnResultRow>>, String> {
        let primary = &request.node_properties[0];
        let mut builder = KnnBuilder::new(Arc::clone(gr.store()), primary.name.clone())
            .k(request.top_k)
            .similarity_cutoff(request.similarity_cutoff)
            .metric(primary.metric)
            .concurrency(request.common.concurrency.value());

        if request.node_properties.len() > 1 {
            for prop in request.node_properties.iter().skip(1) {
                builder = builder.add_property(prop.name.clone(), prop.metric);
            }
        }

        let iter = builder.stream().map_err(|e| e.to_string())?;
        Ok(Some(iter.collect()))
    };

    let builder = FnStatsResultBuilder(
        |_gr: &GraphResources, rows: Option<Vec<KnnResultRow>>, timings| {
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN stream failed: {e}")),
    }
}
