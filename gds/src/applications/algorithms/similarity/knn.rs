use crate::algo::similarity::knn::{metrics::SimilarityMetric, KnnResultRow};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::similarity::shared::{
    err, get_array, get_f64, get_str, get_u64, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::similarity::knn::{KnnBuilder, KnnStats};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct KnnPropertySpec {
    pub name: String,
    pub metric: SimilarityMetric,
}

#[derive(Debug, Clone)]
pub struct KnnRequest {
    pub common: CommonRequest,
    pub node_properties: Vec<KnnPropertySpec>,
    pub top_k: usize,
    pub similarity_cutoff: f64,
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

impl KnnRequest {
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

        let mut node_properties: Vec<KnnPropertySpec> = Vec::new();
        for item in raw_props {
            if let Some(name) = item.as_str() {
                node_properties.push(KnnPropertySpec {
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

                node_properties.push(KnnPropertySpec {
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

        Ok(Self {
            common,
            node_properties,
            top_k,
            similarity_cutoff,
        })
    }
}

pub fn handle_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "knn";

    let parsed = match KnnRequest::parse(request) {
        Ok(r) => r,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let graph_resources =
        match CatalogLoader::load_or_err(catalog.as_ref(), &parsed.common.graph_name) {
            Ok(r) => r,
            Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
        };

    match parsed.common.mode {
        Mode::Stream => run_stream(op, &parsed, &graph_resources),
        Mode::Stats => run_stats(op, &parsed, &graph_resources),
        Mode::Estimate => run_estimate(op, &parsed, &graph_resources),
        Mode::Mutate => err(op, "NOT_IMPLEMENTED", "KNN mutate is not implemented yet"),
        Mode::Write => err(op, "NOT_IMPLEMENTED", "KNN write is not implemented yet"),
    }
}

fn run_stream(op: &str, request: &KnnRequest, graph_resources: &GraphResources) -> Value {
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
        let builder = configure_builder(gr, request);
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

fn run_stats(op: &str, request: &KnnRequest, graph_resources: &GraphResources) -> Value {
    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    let task = Tasks::leaf("KNN::stats".to_string()).base().clone();

    let compute = |gr: &GraphResources,
                   _tracker: &mut dyn ProgressTracker,
                   _termination: &TerminationFlag|
     -> Result<Option<KnnStats>, String> {
        let builder = configure_builder(gr, request);
        let stats = builder.stats().map_err(|e| e.to_string())?;
        Ok(Some(stats))
    };

    let builder = FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<KnnStats>, timings| {
        json!({
            "ok": true,
            "op": op,
            "mode": "stats",
            "data": stats,
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN stats failed: {e}")),
    }
}

fn run_estimate(op: &str, request: &KnnRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let builder = configure_builder(graph_resources, request);
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
    }
}

fn configure_builder(graph_resources: &GraphResources, request: &KnnRequest) -> KnnBuilder {
    let primary = &request.node_properties[0];
    let mut builder = KnnBuilder::new(Arc::clone(graph_resources.store()), primary.name.clone())
        .k(request.top_k)
        .similarity_cutoff(request.similarity_cutoff)
        .metric(primary.metric)
        .concurrency(request.common.concurrency.value());

    if request.node_properties.len() > 1 {
        for prop in request.node_properties.iter().skip(1) {
            builder = builder.add_property(prop.name.clone(), prop.metric);
        }
    }

    builder
}
