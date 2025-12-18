//! Articulation Points Integration Tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::procedures::facades::traits::StreamResults;
    use crate::procedures::facades::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_undirected_edges(
        node_count: usize,
        edges: &[(usize, usize)],
    ) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            outgoing[b].push(a as i64);
            incoming[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Undirected);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            crate::config::GraphStoreConfig::default(),
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
    fn test_simple_path() {
        // Simple path: 0-1-2-3-4
        // Node 1, 2, 3 should be articulation points
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }

    #[test]
    fn test_cycle() {
        // Simple cycle: 0-1-2-3-0
        // No articulation points in a simple cycle
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_bridge_connected_components() {
        // Two cycles connected by a bridge: (0-1-2-0) - 3 - (4-5-6-4)
        // Node 3 should be an articulation point
        let store = store_from_undirected_edges(
            7,
            &[
                (0, 1),
                (1, 2),
                (2, 0),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 4),
            ],
        );
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        // Both 2 and 3 are articulation points in this shape.
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }

    #[test]
    fn test_single_node() {
        // Single node
        let store = store_from_undirected_edges(1, &[]);
        let graph = Graph::new(Arc::new(store));
        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_two_nodes_with_edge() {
        // Two nodes connected: 0-1
        let store = store_from_undirected_edges(2, &[(0, 1)]);
        let graph = Graph::new(Arc::new(store));
        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_star_graph() {
        // Star graph: center node 0 connected to 1, 2, 3, 4
        // Node 0 should be an articulation point
        let store = store_from_undirected_edges(5, &[(0, 1), (0, 2), (0, 3), (0, 4)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&0));
        assert!(!ids.contains(&1));
        assert!(!ids.contains(&2));
    }

    #[test]
    fn test_disconnected_components() {
        // Two separate components: 0-1 and 2-3
        let store = store_from_undirected_edges(4, &[(0, 1), (2, 3)]);
        let graph = Graph::new(Arc::new(store));
        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_complex_graph() {
        // More complex graph with multiple articulation points
        //   0
        //   |
        //   1 -- 2
        //   |    |
        //   3 -- 4
        //        |
        //        5
        let store =
            store_from_undirected_edges(6, &[(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5)]);
        let graph = Graph::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&1), "Node 1 should be an articulation point");
        assert!(ids.contains(&4), "Node 4 should be an articulation point");
    }
}
