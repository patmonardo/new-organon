use crate::algo::similarity::knn::metrics::SimilarityMetric;
use crate::applications::algorithms::similarity::shared::{
    get_array, get_f64, get_str, get_u64, CommonRequest,
};
use crate::projection::NodeLabel;
use serde_json::Value;

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
