use crate::algo::similarity::filtered_knn::FilteredKnnResultRow;
use crate::algo::similarity::filtered_knn::FilteredKnnStats;
use crate::algo::similarity::knn::metrics::SimilarityMetric;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::{
    err, get_array, get_f64, get_str, get_u64, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::filtered_knn::FilteredKnnFacade;
use crate::projection::NodeLabel;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilteredKnnPropertySpec {
    pub name: String,
    pub metric: SimilarityMetric,
}

#[derive(Debug, Clone)]
pub struct FilteredKnnRequest {
    pub common: CommonRequest,
    pub node_properties: Vec<FilteredKnnPropertySpec>,
    pub top_k: usize,
    pub similarity_cutoff: f64,
    pub source_node_labels: Vec<NodeLabel>,
    pub target_node_labels: Vec<NodeLabel>,
}

fn parse_metric(raw: &str) -> Result<SimilarityMetric, String> {
    match raw.to_ascii_uppercase().as_str() {
        "DEFAULT" => Ok(SimilarityMetric::Default),
        "COSINE" => Ok(SimilarityMetric::Cosine),
        "EUCLIDEAN" => Ok(SimilarityMetric::Euclidean),
        "PEARSON" => Ok(SimilarityMetric::Pearson),
        "JACCARD" => Ok(SimilarityMetric::Jaccard),
        "OVERLAP" => Ok(SimilarityMetric::Overlap),
        other => Err(format!(
            "Invalid similarityMetric '{other}'. Use DEFAULT|COSINE|EUCLIDEAN|PEARSON|JACCARD|OVERLAP"
        )),
    }
}

fn parse_label_list(request: &Value, key: &str) -> Result<Vec<NodeLabel>, String> {
    let Some(raw) = get_array(request, key) else {
        return Ok(Vec::new());
    };

    let mut labels = Vec::new();
    for item in raw {
        let Some(name) = item.as_str() else {
            return Err(format!("{key} must be an array of strings"));
        };
        labels.push(NodeLabel::of(name));
    }
    Ok(labels)
}

impl FilteredKnnRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let top_k = get_u64(request, "topK").unwrap_or(10) as usize;
        let similarity_cutoff = get_f64(request, "similarityCutoff").unwrap_or(0.0);

        let default_metric = get_str(request, "similarityMetric")
            .map(parse_metric)
            .transpose()?
            .unwrap_or(SimilarityMetric::Default);

        let raw_props = get_array(request, "nodeProperties")
            .ok_or_else(|| "Missing 'nodeProperties' parameter".to_string())?;

        let mut node_properties: Vec<FilteredKnnPropertySpec> = Vec::new();
        for item in raw_props {
            if let Some(name) = item.as_str() {
                node_properties.push(FilteredKnnPropertySpec {
                    name: name.to_string(),
                    metric: default_metric,
                });
                continue;
            }

            if let Some(obj) = item.as_object() {
                let name = obj.get("name").and_then(|v| v.as_str()).ok_or_else(|| {
                    "nodeProperties items must be strings or objects with 'name'".to_string()
                })?;

                let metric = obj
                    .get("metric")
                    .and_then(|v| v.as_str())
                    .map(parse_metric)
                    .transpose()?
                    .unwrap_or(default_metric);

                node_properties.push(FilteredKnnPropertySpec {
                    name: name.to_string(),
                    metric,
                });
                continue;
            }

            return Err(
                "nodeProperties items must be strings or objects with {name, metric}".to_string(),
            );
        }

        if node_properties.is_empty() {
            return Err("nodeProperties array cannot be empty".to_string());
        }

        let source_node_labels = parse_label_list(request, "sourceNodeLabels")?;
        let target_node_labels = parse_label_list(request, "targetNodeLabels")?;

        Ok(Self {
            common,
            node_properties,
            top_k,
            similarity_cutoff,
            source_node_labels,
            target_node_labels,
        })
    }
}

pub fn handle_filtered_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "filtered_knn";

    let parsed = match FilteredKnnRequest::parse(request) {
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
            let task = Tasks::leaf("filtered_knn::stream".to_string())
                .base()
                .clone();

            let request = parsed.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<FilteredKnnResultRow>>, String> {
                let builder = configure_builder(gr, &request);
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
                    &format!("FilteredKNN stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("filtered_knn::stats".to_string())
                .base()
                .clone();

            let request = parsed.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<FilteredKnnStats>, String> {
                let builder = configure_builder(gr, &request);
                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<FilteredKnnStats>, timings| {
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
                    &format!("FilteredKNN stats failed: {e}"),
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
                            "ranIterations": result.stats.ran_iterations,
                            "didConverge": result.stats.did_converge,
                            "nodePairsConsidered": result.stats.node_pairs_considered,
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
                    &format!("FilteredKNN mutate failed: {e:?}"),
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
                    &format!("FilteredKNN write failed: {e:?}"),
                ),
            }
        }
    }
}

fn configure_builder(
    graph_resources: &GraphResources,
    request: &FilteredKnnRequest,
) -> FilteredKnnFacade {
    let primary = &request.node_properties[0];
    let mut builder =
        FilteredKnnFacade::new(Arc::clone(graph_resources.store()), primary.name.clone())
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

    builder
}
