use crate::algo::similarity::NodeSimilarityMetric;
use crate::applications::algorithms::similarity::shared::{
    get_f64, get_str, get_u64, CommonRequest,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct NodeSimilarityRequest {
    pub common: CommonRequest,
    pub metric: NodeSimilarityMetric,
    pub similarity_cutoff: f64,
    pub top_k: usize,
    pub top_n: usize,
    pub weight_property: Option<String>,
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

impl NodeSimilarityRequest {
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

        Ok(Self {
            common,
            metric,
            similarity_cutoff,
            top_k,
            top_n,
            weight_property,
        })
    }
}
