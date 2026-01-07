use crate::applications::algorithms::pathfinding::shared::{get_str, get_u64, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DeltaSteppingRequest {
    pub common: CommonRequest,
    pub source: u64,
    pub delta: f64,
    pub weight_property: String,
    pub direction: String,
    pub relationship_types: Vec<String>,
    pub store_predecessors: bool,
}

impl DeltaSteppingRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source = get_u64(request, "source")
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'source' parameter".to_string())?;

        let delta = request.get("delta").and_then(|v| v.as_f64()).unwrap_or(1.0);

        let weight_property = get_str(request, "weightProperty")
            .or_else(|| get_str(request, "weight_property"))
            .or_else(|| get_str(request, "relationshipWeightProperty"))
            .or_else(|| get_str(request, "relationship_weight_property"))
            .unwrap_or("weight")
            .to_string();

        let direction = get_str(request, "direction")
            .or_else(|| get_str(request, "traversalDirection"))
            .unwrap_or("outgoing")
            .to_string();

        let relationship_types: Vec<String> = request
            .get("relationshipTypes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let store_predecessors = request
            .get("storePredecessors")
            .or_else(|| request.get("store_predecessors"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        Ok(Self {
            common,
            source,
            delta,
            weight_property,
            direction,
            relationship_types,
            store_predecessors,
        })
    }
}
