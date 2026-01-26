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
use crate::algo::similarity::filtered_node_similarity::FilteredNodeSimilarityStats;
use crate::procedures::similarity::filtered_node_similarity::FilteredNodeSimilarityFacade;
use crate::projection::NodeLabel;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilteredNodeSimilarityRequest {
    pub common: CommonRequest,
    pub metric: NodeSimilarityMetric,
    pub similarity_cutoff: f64,
    pub top_k: usize,
    pub top_n: usize,
    pub weight_property: Option<String>,
    pub source_node_label: Option<NodeLabel>,
    pub target_node_label: Option<NodeLabel>,
}

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

impl FilteredNodeSimilarityRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let metric = get_str(request, "similarityMetric")
            .map(parse_metric)
            .transpose()?
            .unwrap_or(NodeSimilarityMetric::Jaccard);
        let similarity_cutoff = get_f64(request, "similarityCutoff").unwrap_or(0.1);
        let top_k = get_u64(request, "topK").unwrap_or(10) as usize;
        let top_n = get_u64(request, "topN").unwrap_or(0) as usize;

        let weight_property = get_str(request, "weightProperty")
            .or_else(|| get_str(request, "weight_property"))
            .map(|s| s.to_string());

        let source_node_label = get_str(request, "sourceNodeLabel")
            .or_else(|| get_str(request, "source_node_label"))
            .map(|s| NodeLabel::of(s));

        let target_node_label = get_str(request, "targetNodeLabel")
            .or_else(|| get_str(request, "target_node_label"))
            .map(|s| NodeLabel::of(s));

        Ok(Self {
            common,
            metric,
            similarity_cutoff,
            top_k,
            top_n,
            weight_property,
            source_node_label,
            target_node_label,
        })
    }
}

pub fn handle_filtered_node_similarity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "filtered_node_similarity";

    let parsed = match FilteredNodeSimilarityRequest::parse(request) {
        Ok(r) => r,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let graph_resources =
        match CatalogLoader::load_or_err(catalog.as_ref(), &parsed.common.graph_name) {
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

    match parsed.common.mode {
        Mode::Stream => {
            let task = Tasks::leaf("filtered_node_similarity::stream".to_string())
                .base()
                .clone();

            let request = parsed.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<NodeSimilarityResult>>, String> {
                let builder = configure_builder(gr, &request);
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
                parsed.common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("FilteredNodeSimilarity stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("filtered_node_similarity::stats".to_string())
                .base()
                .clone();

            let request = parsed.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<FilteredNodeSimilarityStats>, String> {
                let builder = configure_builder(gr, &request);
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
                &graph_resources,
                parsed.common.concurrency,
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
        Mode::Estimate => match parsed.common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let builder = configure_builder(&graph_resources, &parsed);
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

            let builder = configure_builder(&graph_resources, &parsed);
            match builder.mutate(&property_name) {
                Ok(result) => {
                    catalog.set(&parsed.common.graph_name, result.updated_store);
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
                    &format!("FilteredNodeSimilarity mutate failed: {e:?}"),
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

            let builder = configure_builder(&graph_resources, &parsed);
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
                    &format!("FilteredNodeSimilarity write failed: {e:?}"),
                ),
            }
        }
    }
}

fn configure_builder(
    graph_resources: &GraphResources,
    request: &FilteredNodeSimilarityRequest,
) -> FilteredNodeSimilarityFacade {
    let mut builder = FilteredNodeSimilarityFacade::new(Arc::clone(graph_resources.store()))
        .metric(request.metric)
        .similarity_cutoff(request.similarity_cutoff)
        .top_k(request.top_k)
        .top_n(request.top_n)
        .concurrency(request.common.concurrency.value());

    if let Some(ref property) = request.weight_property {
        builder = builder.weight_property(property.clone());
    }

    if let Some(ref label) = request.source_node_label {
        builder = builder.source_node_label(label.clone());
    }

    if let Some(ref label) = request.target_node_label {
        builder = builder.target_node_label(label.clone());
    }

    builder
}
