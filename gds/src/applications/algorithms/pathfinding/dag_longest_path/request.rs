use crate::applications::algorithms::pathfinding::shared::CommonRequest;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DagLongestPathRequest {
    pub common: CommonRequest,
}

impl DagLongestPathRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let common = CommonRequest::parse(request)?;
        Ok(Self { common })
    }
}
