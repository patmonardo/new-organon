use crate::algo::similarity::knn::KnnStats;
use crate::algo::similarity::knn::{metrics::SimilarityMetric, KnnResultRow};
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
use crate::procedures::similarity::knn::KnnFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

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

pub fn handle_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "knn";

    let common = match CommonRequest::parse(request) {
        Ok(r) => r,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let top_k = get_u64(request, "topK").unwrap_or(10) as usize;
    let similarity_cutoff = get_f64(request, "similarityCutoff").unwrap_or(0.0);

    let default_metric = match get_str(request, "similarityMetric") {
        Some(raw) => match parse_metric(raw) {
            Ok(m) => m,
            Err(message) => return err(op, "INVALID_REQUEST", &message),
        },
        None => SimilarityMetric::Default,
    };

    let raw_props = match get_array(request, "nodeProperties") {
        Some(v) => v,
        None => return err(op, "INVALID_REQUEST", "Missing 'nodeProperties' parameter"),
    };

    let mut node_properties: Vec<(String, SimilarityMetric)> = Vec::new();
    for item in raw_props {
        if let Some(name) = item.as_str() {
            node_properties.push((name.to_string(), default_metric));
            continue;
        }

        if let Some(obj) = item.as_object() {
            let name = match obj.get("name").and_then(|v| v.as_str()) {
                Some(v) => v,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "nodeProperties items must be strings or objects with 'name'",
                    )
                }
            };

            let metric = match obj.get("metric").and_then(|v| v.as_str()) {
                Some(raw) => match parse_metric(raw) {
                    Ok(m) => m,
                    Err(message) => return err(op, "INVALID_REQUEST", &message),
                },
                None => default_metric,
            };

            node_properties.push((name.to_string(), metric));
            continue;
        }

        return err(
            op,
            "INVALID_REQUEST",
            "nodeProperties items must be strings or objects with {name, metric}",
        );
    }

    if node_properties.is_empty() {
        return err(
            op,
            "INVALID_REQUEST",
            "nodeProperties array cannot be empty",
        );
    }

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
            let task = Tasks::leaf("knn::stream".to_string()).base().clone();

            let node_properties = node_properties.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<KnnResultRow>>, String> {
                let (primary_name, primary_metric) = node_properties
                    .first()
                    .expect("node_properties empty; validated earlier")
                    .clone();

                let mut builder = KnnFacade::new(Arc::clone(gr.store()), primary_name)
                    .k(top_k)
                    .similarity_cutoff(similarity_cutoff)
                    .metric(primary_metric)
                    .concurrency(common.concurrency.value());

                if node_properties.len() > 1 {
                    for (name, metric) in node_properties.iter().skip(1) {
                        builder = builder.add_property(name.clone(), *metric);
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
                &graph_resources,
                common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN stream failed: {e}")),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("knn::stats".to_string()).base().clone();

            let node_properties = node_properties.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<KnnStats>, String> {
                let (primary_name, primary_metric) = node_properties
                    .first()
                    .expect("node_properties empty; validated earlier")
                    .clone();

                let mut builder = KnnFacade::new(Arc::clone(gr.store()), primary_name)
                    .k(top_k)
                    .similarity_cutoff(similarity_cutoff)
                    .metric(primary_metric)
                    .concurrency(common.concurrency.value());

                if node_properties.len() > 1 {
                    for (name, metric) in node_properties.iter().skip(1) {
                        builder = builder.add_property(name.clone(), *metric);
                    }
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder =
                FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<KnnStats>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings)
                    })
                });

            match convenience.process_stats(
                &graph_resources,
                common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN stats failed: {e}")),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let (primary_name, primary_metric) = node_properties
                    .first()
                    .expect("node_properties empty; validated earlier")
                    .clone();

                let mut builder = KnnFacade::new(Arc::clone(graph_resources.store()), primary_name)
                    .k(top_k)
                    .similarity_cutoff(similarity_cutoff)
                    .metric(primary_metric)
                    .concurrency(common.concurrency.value());

                if node_properties.len() > 1 {
                    for (name, metric) in node_properties.iter().skip(1) {
                        builder = builder.add_property(name.clone(), *metric);
                    }
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

            let (primary_name, primary_metric) = node_properties
                .first()
                .expect("node_properties empty; validated earlier")
                .clone();

            let mut builder = KnnFacade::new(Arc::clone(graph_resources.store()), primary_name)
                .k(top_k)
                .similarity_cutoff(similarity_cutoff)
                .metric(primary_metric)
                .concurrency(common.concurrency.value());

            if node_properties.len() > 1 {
                for (name, metric) in node_properties.iter().skip(1) {
                    builder = builder.add_property(name.clone(), *metric);
                }
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN mutate failed: {e:?}")),
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

            let (primary_name, primary_metric) = node_properties
                .first()
                .expect("node_properties empty; validated earlier")
                .clone();

            let mut builder = KnnFacade::new(Arc::clone(graph_resources.store()), primary_name)
                .k(top_k)
                .similarity_cutoff(similarity_cutoff)
                .metric(primary_metric)
                .concurrency(common.concurrency.value());

            if node_properties.len() > 1 {
                for (name, metric) in node_properties.iter().skip(1) {
                    builder = builder.add_property(name.clone(), *metric);
                }
            }

            match builder.write(&property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "write",
                    "data": result
                }),
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN write failed: {e:?}")),
            }
        }
    }
}
