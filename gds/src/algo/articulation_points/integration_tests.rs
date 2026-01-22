//! Articulation Points integration tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
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
    fn finds_articulation_points_in_path() {
        // Path 0-1-2-3-4 => internal nodes are articulation points.
        let store = store_from_undirected_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();

        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }

    #[test]
    fn cycle_has_no_articulation_points() {
        let store = store_from_undirected_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        assert!(rows.is_empty());
    }

    #[test]
    fn star_center_is_articulation_point() {
        let store = store_from_undirected_edges(5, &[(0, 1), (0, 2), (0, 3), (0, 4)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph.articulation_points().stream().unwrap().collect();
        let ids: Vec<u64> = rows.into_iter().map(|r| r.node_id).collect();
        assert!(ids.contains(&0));
    }
}
