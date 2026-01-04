use crate::applications::algorithms::pathfinding::shared::{
    get_bool, get_str, get_u64, CommonRequest,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SpanningTreeRequest {
    pub common: CommonRequest,
    pub start_node: u64,
    pub compute_minimum: bool,
    pub weight_property: String,
    pub direction: String,
    pub relationship_types: Vec<String>,
}

impl SpanningTreeRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let start_node = get_u64(request, "startNode")
            .or_else(|| get_u64(request, "start_node"))
            .or_else(|| get_u64(request, "source"))
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'startNode' parameter".to_string())?;

        let compute_minimum = get_bool(request, "computeMinimum")
            .or_else(|| get_bool(request, "compute_minimum"))
            .or_else(|| get_bool(request, "minimum"))
            .unwrap_or(true);

        let weight_property = get_str(request, "weightProperty")
            .or_else(|| get_str(request, "weight_property"))
            .or_else(|| get_str(request, "relationshipWeightProperty"))
            .or_else(|| get_str(request, "relationship_weight_property"))
            .unwrap_or("weight")
            .to_string();

        let direction = get_str(request, "direction")
            .or_else(|| get_str(request, "traversalDirection"))
            .unwrap_or("undirected")
            .to_string();

        let relationship_types: Vec<String> = request
            .get("relationshipTypes")
            .or_else(|| request.get("relationship_types"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            common,
            start_node,
            compute_minimum,
            weight_property,
            direction,
            relationship_types,
        })
    }
}
