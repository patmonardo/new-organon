use crate::applications::algorithms::pathfinding::shared::{
    get_bool, get_output_graph_name, get_property_name, get_u64, CommonRequest,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DfsRequest {
    pub common: CommonRequest,
    pub source: u64,
    pub targets: Vec<u64>,
    pub max_depth: Option<u32>,
    pub track_paths: bool,
    pub property_name: Option<String>,
    pub output_graph_name: Option<String>,
}

impl DfsRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let source = get_u64(request, "source")
            .or_else(|| get_u64(request, "sourceNode"))
            .ok_or_else(|| "Missing 'source' parameter".to_string())?;

        let targets: Vec<u64> = if let Some(t) = get_u64(request, "target")
            .or_else(|| get_u64(request, "targetNode"))
        {
            vec![t]
        } else if let Some(arr) = request.get("targets").and_then(|v| v.as_array()) {
            arr.iter().filter_map(|v| v.as_u64()).collect()
        } else {
            Vec::new()
        };

        let max_depth = request
            .get("maxDepth")
            .or_else(|| request.get("max_depth"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);

        let track_paths = get_bool(request, "trackPaths")
            .or_else(|| get_bool(request, "track_paths"))
            .unwrap_or(false);

        let property_name = get_property_name(request).map(|s| s.to_string());
        let output_graph_name = get_output_graph_name(request).map(|s| s.to_string());

        Ok(Self {
            common,
            source,
            targets,
            max_depth,
            track_paths,
            property_name,
            output_graph_name,
        })
    }
}
