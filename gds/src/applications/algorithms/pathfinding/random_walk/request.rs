use crate::applications::algorithms::pathfinding::shared::{get_u64, get_usize, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RandomWalkRequest {
    pub common: CommonRequest,
    pub walks_per_node: usize,
    pub walk_length: usize,
    pub return_factor: f64,
    pub in_out_factor: f64,
    pub source_nodes: Vec<u64>,
    pub random_seed: Option<u64>,
}

fn parse_f64(request: &Value, key: &str) -> Option<f64> {
    request.get(key).and_then(|v| v.as_f64())
}

impl RandomWalkRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let walks_per_node = get_usize(request, "walksPerNode")
            .or_else(|| get_usize(request, "walks_per_node"))
            .unwrap_or(10);

        let walk_length = get_usize(request, "walkLength")
            .or_else(|| get_usize(request, "walk_length"))
            .unwrap_or(80);

        let return_factor = parse_f64(request, "returnFactor")
            .or_else(|| parse_f64(request, "return_factor"))
            .unwrap_or(1.0);

        let in_out_factor = parse_f64(request, "inOutFactor")
            .or_else(|| parse_f64(request, "in_out_factor"))
            .unwrap_or(1.0);

        let source_nodes: Vec<u64> = request
            .get("sourceNodes")
            .or_else(|| request.get("source_nodes"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
            .unwrap_or_default();

        let random_seed = get_u64(request, "randomSeed").or_else(|| get_u64(request, "random_seed"));

        Ok(Self {
            common,
            walks_per_node,
            walk_length,
            return_factor,
            in_out_factor,
            source_nodes,
            random_seed,
        })
    }
}
