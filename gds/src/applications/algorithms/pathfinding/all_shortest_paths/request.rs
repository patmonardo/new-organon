use crate::applications::algorithms::pathfinding::shared::{get_str, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AllShortestPathsRequest {
    pub common: CommonRequest,
    pub weighted: bool,
    pub direction: String,
    pub weight_property: String,
    pub relationship_types: Vec<String>,
    pub max_results: Option<usize>,
}

impl AllShortestPathsRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let weighted = request
            .get("weighted")
            .or_else(|| request.get("useWeights"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let direction = get_str(request, "direction")
            .unwrap_or("outgoing")
            .to_string();

        let weight_property = get_str(request, "weightProperty")
            .or_else(|| get_str(request, "weight_property"))
            .or_else(|| get_str(request, "relationshipWeightProperty"))
            .or_else(|| get_str(request, "relationship_weight_property"))
            .unwrap_or("weight")
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

        let max_results = request
            .get("maxResults")
            .or_else(|| request.get("max_results"))
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        Ok(Self {
            common,
            weighted,
            direction,
            weight_property,
            relationship_types,
            max_results,
        })
    }
}
