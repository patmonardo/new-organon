use crate::applications::algorithms::pathfinding::shared::{get_str, get_u64, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct KSpanningTreeRequest {
    pub common: CommonRequest,
    pub source_node: u64,
    pub k: u64,
    pub objective: String,
    pub weight_property: Option<String>,
}

impl KSpanningTreeRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source_node = get_u64(request, "sourceNode")
            .or_else(|| get_u64(request, "source_node"))
            .or_else(|| get_u64(request, "source"))
            .ok_or_else(|| "Missing 'sourceNode' parameter".to_string())?;

        let k = get_u64(request, "k").unwrap_or(1);

        let objective = get_str(request, "objective")
            .or_else(|| get_str(request, "obj"))
            .unwrap_or("min")
            .to_string();

        let weight_property = get_str(request, "weightProperty")
            .or_else(|| get_str(request, "weight_property"))
            .map(|s| s.to_string());

        Ok(Self {
            common,
            source_node,
            k,
            objective,
            weight_property,
        })
    }
}
