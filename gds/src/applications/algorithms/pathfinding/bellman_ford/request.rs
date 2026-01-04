use crate::applications::algorithms::pathfinding::shared::{
    get_bool, get_str, get_u64, CommonRequest,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BellmanFordRequest {
    pub common: CommonRequest,
    pub source: u64,
    pub weight_property: String,
    pub direction: String,
    pub relationship_types: Vec<String>,
    pub track_negative_cycles: bool,
    pub track_paths: bool,
}

impl BellmanFordRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source = get_u64(request, "source")
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'source' parameter".to_string())?;

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

        let track_negative_cycles = get_bool(request, "trackNegativeCycles")
            .or_else(|| get_bool(request, "track_negative_cycles"))
            .unwrap_or(true);

        let track_paths = get_bool(request, "trackPaths")
            .or_else(|| get_bool(request, "track_paths"))
            .unwrap_or(true);

        Ok(Self {
            common,
            source,
            weight_property,
            direction,
            relationship_types,
            track_negative_cycles,
            track_paths,
        })
    }
}
