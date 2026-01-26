use crate::algo::similarity::node_similarity::NodeSimilarityStats;
use crate::algo::similarity::node_similarity::{NodeSimilarityMetric, NodeSimilarityResult};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::{
    err, get_f64, get_str, get_u64, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::NodeSimilarityFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

fn parse_metric(raw: &str) -> Result<NodeSimilarityMetric, String> {
    match raw.to_ascii_lowercase().as_str() {
        "jaccard" => Ok(NodeSimilarityMetric::Jaccard),
        "cosine" => Ok(NodeSimilarityMetric::Cosine),
        "overlap" => Ok(NodeSimilarityMetric::Overlap),
        other => Err(format!(
            "Invalid similarityMetric '{other}'. Use jaccard|cosine|overlap"
        )),
    }
}

pub fn handle_node_similarity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "node_similarity";

    let common = match CommonRequest::parse(request) {
        Ok(r) => r,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let metric = match get_str(request, "similarityMetric") {
        Some(raw) => match parse_metric(raw) {
            Ok(m) => m,
            Err(message) => return err(op, "INVALID_REQUEST", &message),
        },
        None => NodeSimilarityMetric::Jaccard,
    };
    let similarity_cutoff = get_f64(request, "similarityCutoff").unwrap_or(0.1);
    let top_k = get_u64(request, "topK").unwrap_or(10) as usize;
    let top_n = get_u64(request, "topN").unwrap_or(0) as usize;

    let weight_property = get_str(request, "weightProperty")
        .or_else(|| get_str(request, "weight_property"))
        .map(|s| s.to_string());

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), &common.graph_name) {
        Ok(r) => r,
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

    match common.mode {
        Mode::Stream => {
            let task = Tasks::leaf("node_similarity::stream".to_string())
                .base()
                .clone();

            let weight_property = weight_property.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<NodeSimilarityResult>>, String> {
                let mut builder = NodeSimilarityFacade::new(Arc::clone(gr.store()))
                    .metric(metric)
                    .similarity_cutoff(similarity_cutoff)
                    .top_k(top_k)
                    .top_n(top_n)
                    .concurrency(common.concurrency.value());

                if let Some(property) = &weight_property {
                    builder = builder.weight_property(property.clone());
                }

                let iter = builder.stream().map_err(|e| e.to_string())?;
                Ok(Some(iter.collect()))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, rows: Option<Vec<NodeSimilarityResult>>, timings| {
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
                &graph_resources,
                common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("NodeSimilarity stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("node_similarity::stats".to_string())
                .base()
                .clone();

            let weight_property = weight_property.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<NodeSimilarityStats>, String> {
                let mut builder = NodeSimilarityFacade::new(Arc::clone(gr.store()))
                    .metric(metric)
                    .similarity_cutoff(similarity_cutoff)
                    .top_k(top_k)
                    .top_n(top_n)
                    .concurrency(common.concurrency.value());

                if let Some(property) = &weight_property {
                    builder = builder.weight_property(property.clone());
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<NodeSimilarityStats>, timings| {
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
                &graph_resources,
                common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("NodeSimilarity stats failed: {e}"),
                ),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let mut builder = NodeSimilarityFacade::new(Arc::clone(graph_resources.store()))
                    .metric(metric)
                    .similarity_cutoff(similarity_cutoff)
                    .top_k(top_k)
                    .top_n(top_n)
                    .concurrency(common.concurrency.value());

                if let Some(property) = &weight_property {
                    builder = builder.weight_property(property.clone());
                }

                let memory = builder.estimate_memory();

                json!({
                    "ok": true,
                    "op": op,
                    "mode": "estimate",
                    "submode": "memory",
                    "data": {
                        "minBytes": memory.min(),
                        "maxBytes": memory.max()
                    }
                })
            }
            Some(other) => err(
                op,
                "INVALID_REQUEST",
                &format!("Invalid estimate submode '{other}'. Use 'memory'"),
            ),
        },
        Mode::Mutate => {
            let property_name = match get_str(request, "mutateProperty") {
                Some(name) => name.to_string(),
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'mutateProperty' parameter for mutate mode",
                    )
                }
            };

            let mut builder = NodeSimilarityFacade::new(Arc::clone(graph_resources.store()))
                .metric(metric)
                .similarity_cutoff(similarity_cutoff)
                .top_k(top_k)
                .top_n(top_n)
                .concurrency(common.concurrency.value());

            if let Some(property) = &weight_property {
                builder = builder.weight_property(property.clone());
            }

            match builder.mutate(&property_name) {
                Ok(result) => {
                    catalog.set(&common.graph_name, result.updated_store);
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "mutate",
                        "data": {
                            "nodesCompared": result.stats.nodes_compared,
                            "similarityPairs": result.stats.similarity_pairs,
                            "similarityDistribution": result.stats.similarity_distribution,
                            "computeMillis": result.stats.compute_millis,
                            "success": result.stats.success,
                            "relationshipsWritten": result.summary.nodes_updated,
                            "mutateProperty": result.summary.property_name,
                            "executionTimeMs": result.summary.execution_time_ms
                        }
                    })
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("NodeSimilarity mutate failed: {e:?}"),
                ),
            }
        }
        Mode::Write => {
            let property_name = match get_str(request, "writeProperty") {
                Some(name) => name.to_string(),
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'writeProperty' parameter for write mode",
                    )
                }
            };

            let mut builder = NodeSimilarityFacade::new(Arc::clone(graph_resources.store()))
                .metric(metric)
                .similarity_cutoff(similarity_cutoff)
                .top_k(top_k)
                .top_n(top_n)
                .concurrency(common.concurrency.value());

            if let Some(property) = &weight_property {
                builder = builder.weight_property(property.clone());
            }

            match builder.write(&property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "write",
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("NodeSimilarity write failed: {e:?}"),
                ),
            }
        }
    }
}
