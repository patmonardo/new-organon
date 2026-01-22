//! Louvain Integration Tests
//!
//! These tests are smoke checks to ensure the facade wiring works end-to-end.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_outgoing(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                if target >= 0 {
                    let t = target as usize;
                    if t < node_count {
                        incoming[t].push(source as i64);
                    }
                }
            }
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn louvain_splits_isolated_nodes() {
        // 0--1 connected, 2 isolated => expect two communities.
        let store = store_from_outgoing(vec![vec![1], vec![0], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.louvain().run().unwrap();
        assert_eq!(result.data.len(), 3);
        assert_eq!(result.data[0], result.data[1]);
        assert_ne!(result.data[2], result.data[0]);

        let stats = graph.louvain().stats().unwrap();
        assert_eq!(stats.community_count, 2);
    }

    #[test]
    fn louvain_empty_graph_is_empty() {
        let store = store_from_outgoing(vec![]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.louvain().run().unwrap();
        assert!(result.data.is_empty());

        let stats = graph.louvain().stats().unwrap();
        assert_eq!(stats.community_count, 0);
    }
}
