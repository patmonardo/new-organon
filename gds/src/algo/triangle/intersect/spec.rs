use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RelationshipIntersectConfig {
    pub max_degree: u64,
}

impl Default for RelationshipIntersectConfig {
    fn default() -> Self {
        Self {
            max_degree: u64::MAX,
        }
    }
}
