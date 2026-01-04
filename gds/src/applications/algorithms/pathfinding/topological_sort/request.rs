use crate::applications::algorithms::pathfinding::shared::{get_bool, CommonRequest};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TopologicalSortRequest {
    pub common: CommonRequest,
    pub compute_max_distance: bool,
}

impl TopologicalSortRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;

        let compute_max_distance = get_bool(request, "computeMaxDistance")
            .or_else(|| get_bool(request, "compute_max_distance"))
            .unwrap_or(false);

        Ok(Self {
            common,
            compute_max_distance,
        })
    }
}
