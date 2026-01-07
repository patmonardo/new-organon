use crate::applications::algorithms::pathfinding::shared::{
    get_bool, get_output_graph_name, get_property_name, get_str, get_u64, CommonRequest,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DijkstraRequest {
    pub common: CommonRequest,
    pub source: u64,
    pub targets: Vec<u64>,
    pub weight_property: String,
    pub direction: String,
    pub track_relationships: bool,
    pub property_name: Option<String>,
    pub output_graph_name: Option<String>,
}

impl DijkstraRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source = get_u64(request, "source")
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'source' parameter".to_string())?;

        let targets: Vec<u64> = if let Some(t) =
            get_u64(request, "target").or_else(|| get_u64(request, "targetNode"))
        {
            vec![t]
        } else if let Some(arr) = request.get("targets").and_then(|v| v.as_array()) {
            arr.iter().filter_map(|v| v.as_u64()).collect()
        } else {
            Vec::new()
        };

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

        let track_relationships = get_bool(request, "trackRelationships")
            .or_else(|| get_bool(request, "track_relationships"))
            .or_else(|| get_bool(request, "trackPaths"))
            .unwrap_or(false);

        let property_name = get_property_name(request).map(|s| s.to_string());
        let output_graph_name = get_output_graph_name(request).map(|s| s.to_string());

        Ok(Self {
            common,
            source,
            targets,
            weight_property,
            direction,
            track_relationships,
            property_name,
            output_graph_name,
        })
    }
}
