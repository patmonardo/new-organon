use serde::{Deserialize, Serialize};

/// Result for sub-graph projection (filtering an existing graph into a new one).
/// Mirrors Java `GraphFilterResult`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFilterResult {
    pub graph_name: String,
    pub from_graph_name: String,
    pub node_filter: String,
    pub relationship_filter: String,
    pub node_count: u64,
    pub relationship_count: u64,
    pub filter_millis: u64,
}

impl GraphFilterResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph_name: String,
        from_graph_name: String,
        node_filter: String,
        relationship_filter: String,
        node_count: u64,
        relationship_count: u64,
        filter_millis: u64,
    ) -> Self {
        Self {
            graph_name,
            from_graph_name,
            node_filter,
            relationship_filter,
            node_count,
            relationship_count,
            filter_millis,
        }
    }
}
