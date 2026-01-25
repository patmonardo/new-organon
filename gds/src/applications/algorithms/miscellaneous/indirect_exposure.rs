//! IndirectExposure dispatch handler.

use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, AlgorithmProcessingTimings,
    DefaultAlgorithmProcessingTemplate, ProgressTrackerCreator, RequestScopedDependencies,
    StatsResultBuilder,
};
use crate::applications::algorithms::miscellaneous::{err, timings_json};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::miscellaneous::IndirectExposureFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_indirect_exposure(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "indirect_exposure";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stats");

    let sanctioned_property = request
        .get("sanctionedProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("sanctioned")
        .to_string();

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(20);

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(4);

    let concurrency = match Concurrency::new(concurrency_value) {
        Some(value) => value,
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "concurrency must be greater than zero",
            )
        }
    };

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), graph_name) {
        Ok(resources) => resources,
        Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
    };

    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    match mode {
        "stats" => {
            let task = Tasks::leaf("indirect_exposure::stats".to_string())
                .base()
                .clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let result = IndirectExposureFacade::new(Arc::clone(gr.store()))
                    .sanctioned_property(sanctioned_property.clone())
                    .relationship_weight_property(relationship_weight_property.clone())
                    .max_iterations(max_iterations)
                    .concurrency(concurrency_value)
                    .stats()
                    .map_err(|e| e.to_string())?;

                Ok(Some(json!({
                    "exposures": result.exposures,
                    "roots": result.roots,
                    "parents": result.parents,
                    "hops": result.hops,
                    "iterationsRan": result.iterations_ran,
                    "didConverge": result.did_converge,
                })))
            };

            let builder = IndirectExposureStatsBuilder;

            match convenience.process_stats(&graph_resources, concurrency, task, compute, builder) {
                Ok(response) => response,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("indirectExposure failed: {e}"),
                ),
            }
        }
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}

struct IndirectExposureStatsBuilder;

impl StatsResultBuilder<Value, Value> for IndirectExposureStatsBuilder {
    fn build(
        &self,
        _graph_resources: &GraphResources,
        data: Option<Value>,
        timings: AlgorithmProcessingTimings,
    ) -> Value {
        json!({
            "ok": true,
            "op": "indirect_exposure",
            "mode": "stats",
            "data": data,
            "timings": timings_json(timings),
        })
    }
}
