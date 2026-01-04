use crate::applications::algorithms::pathfinding::shared::{get_bool, get_str, get_u64, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SteinerTreeRequest {
    pub common: CommonRequest,
    pub source_node: u64,
    pub target_nodes: Vec<u64>,
    pub relationship_weight_property: Option<String>,
    pub delta: f64,
    pub apply_rerouting: bool,
}

impl SteinerTreeRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source_node = get_u64(request, "sourceNode")
            .or_else(|| get_u64(request, "source_node"))
            .or_else(|| get_u64(request, "source"))
            .ok_or_else(|| "Missing 'sourceNode' parameter".to_string())?;

        let target_nodes: Vec<u64> = request
            .get("targetNodes")
            .or_else(|| request.get("target_nodes"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
            .unwrap_or_default();

        if target_nodes.is_empty() {
            return Err("Missing 'targetNodes' parameter (must be non-empty)".to_string());
        }

        let relationship_weight_property = get_str(request, "relationshipWeightProperty")
            .or_else(|| get_str(request, "relationship_weight_property"))
            .map(|s| s.to_string());

        let delta = request.get("delta").and_then(|v| v.as_f64()).unwrap_or(1.0);

        let apply_rerouting = get_bool(request, "applyRerouting")
            .or_else(|| get_bool(request, "apply_rerouting"))
            .unwrap_or(true);

        Ok(Self {
            common,
            source_node,
            target_nodes,
            relationship_weight_property,
            delta,
            apply_rerouting,
        })
    }
}
