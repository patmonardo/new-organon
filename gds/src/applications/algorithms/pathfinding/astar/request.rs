use crate::applications::algorithms::pathfinding::shared::{get_str, get_u64, CommonRequest};
use crate::procedures::pathfinding::Heuristic;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AStarRequest {
    pub common: CommonRequest,
    pub source: u64,
    pub target: u64,
    pub weight_property: String,
    pub direction: String,
    pub relationship_types: Vec<String>,
    pub heuristic: Heuristic,
}

impl AStarRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source = get_u64(request, "source")
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'source' parameter".to_string())?;

        let target = get_u64(request, "target")
            .or_else(|| get_u64(request, "targetNode"))
            .ok_or_else(|| "Missing 'target' parameter".to_string())?;

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

        let heuristic = match get_str(request, "heuristic").unwrap_or("manhattan") {
            "euclidean" => Heuristic::Euclidean,
            "haversine" => Heuristic::Haversine,
            _ => Heuristic::Manhattan,
        };

        Ok(Self {
            common,
            source,
            target,
            weight_property,
            direction,
            relationship_types,
            heuristic,
        })
    }
}
