use super::super::super::services::GraphListingService;
use crate::core::User;
use crate::types::graph_store::DatabaseId;
use serde_json::{json, Value};

/// Application for listing graphs in the catalog.
///
/// Mirrors Java ListGraphApplication class.
/// Contains graph listing logic with optional degree distribution computation.
pub struct ListGraphApplication {
    graph_listing_service: GraphListingService,
}

impl ListGraphApplication {
    /// Creates a new ListGraphApplication.
    pub fn new(graph_listing_service: GraphListingService) -> Self {
        Self {
            graph_listing_service,
        }
    }

    /// Computes the list of graphs with optional degree distribution.
    ///
    /// Returns a JSON response containing the list of graphs.
    pub fn compute(
        &self,
        graph_name: Option<&str>,
        include_degree_distribution: bool,
        user: &dyn User,
        database_id: &DatabaseId,
    ) -> Value {
        let entries = self.graph_listing_service.list_graphs(
            user,
            database_id,
            graph_name,
            include_degree_distribution,
        );

        let arr: Vec<Value> = entries
            .iter()
            .map(|e| {
                let mut obj = serde_json::Map::new();
                obj.insert("name".to_string(), json!(e.name));
                obj.insert("nodeCount".to_string(), json!(e.node_count));
                obj.insert("relationshipCount".to_string(), json!(e.relationship_count));
                if let Some(dd) = &e.degree_distribution {
                    let map: serde_json::Map<String, Value> =
                        dd.iter().map(|(k, v)| (k.to_string(), json!(v))).collect();
                    obj.insert("degreeDistribution".to_string(), Value::Object(map));
                }
                Value::Object(obj)
            })
            .collect();

        json!({ "ok": true, "op": "listGraphs", "data": { "entries": arr } })
    }
}
