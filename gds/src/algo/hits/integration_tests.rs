use crate::algo::hits::HITSAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algo::hits::HitsComputationRuntime;
    use crate::algo::hits::HitsStorageRuntime;
    use crate::config::GraphStoreConfig;
    use crate::core::utils::progress::tasks::NoopProgressTracker;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
        GraphStore,
    };
    use crate::types::schema::{
        Direction, GraphSchema, MutableGraphSchema, NodeLabel, RelationshipType,
    };

    #[test]
    fn test_hits_algorithm_spec_contract_basics() {
        let algorithm = HITSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "hits");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }

    #[test]
    fn hits_tiny_graph_identifies_top_hub_and_authority() {
        // Build a tiny directed graph with a unique top authority and hub:
        // 0 -> 1, 0 -> 2, 0 -> 3
        // 1 -> 3
        // 2 -> 3
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("hits_tiny");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(NodeLabel::all_nodes());
        let rel_type = RelationshipType::of("R");
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
        let outgoing = vec![vec![1, 2, 3], vec![3], vec![3], vec![]];
        let topo = RelationshipTopology::new(outgoing, None);
        let mut topologies = std::collections::HashMap::new();
        topologies.insert(rel_type, topo);

        let store = DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        );
        assert_eq!(GraphStore::node_count(&store), 4);

        let storage =
            HitsStorageRuntime::with_default_projection(&store).expect("hits storage projection");
        let computation = HitsComputationRuntime::new(1e-8);

        let mut tracker = NoopProgressTracker;
        let result = storage.run(&computation, 25, 1, &mut tracker);

        // Expect clear extremes:
        // - Node 3 is the best authority (many incoming links)
        // - Node 0 is the best hub (points to many nodes incl. the best authority)
        // - Node 0 has no incoming links → lowest authority
        // - Node 3 has no outgoing links → lowest hub
        let eps = 1e-12;

        let hub0 = result.hub_scores[0];
        let hub1 = result.hub_scores[1];
        let hub2 = result.hub_scores[2];
        let hub3 = result.hub_scores[3];

        let auth0 = result.authority_scores[0];
        let auth1 = result.authority_scores[1];
        let auth2 = result.authority_scores[2];
        let auth3 = result.authority_scores[3];

        assert!(hub0 > hub1 + eps);
        assert!(hub0 > hub2 + eps);
        assert!(hub1 >= hub3 - eps);
        assert!(hub2 >= hub3 - eps);

        assert!(auth3 > auth1 + eps);
        assert!(auth3 > auth2 + eps);
        assert!(auth1 >= auth0 - eps);
        assert!(auth2 >= auth0 - eps);
    }
}
