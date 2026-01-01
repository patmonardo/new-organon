//! K1Coloring Integration Tests

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    use crate::procedures::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
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

    fn assert_no_adjacent_equal(colors: &[u64], outgoing: &[Vec<i64>]) {
        for (u, targets) in outgoing.iter().enumerate() {
            for &v in targets {
                if v < 0 {
                    continue;
                }
                let v = v as usize;
                if u == v {
                    continue;
                }
                assert_ne!(colors[u], colors[v], "edge ({u},{v}) has same color");
            }
        }
    }

    #[test]
    fn k1_coloring_triangle_is_valid() {
        // Undirected triangle: 0-1-2-0
        let outgoing = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = Graph::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(10).run().unwrap();
        assert_eq!(result.colors.len(), 3);
        assert!(result.did_converge);
        assert_no_adjacent_equal(&result.colors, &outgoing);

        let unique: HashSet<u64> = result.colors.iter().copied().collect();
        assert!(unique.len() >= 3);
    }

    #[test]
    fn k1_coloring_square_uses_two_colors() {
        // 0-1-2-3-0 (bipartite)
        let outgoing = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = Graph::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(10).run().unwrap();
        assert_eq!(result.colors.len(), 4);
        assert!(result.did_converge);
        assert_no_adjacent_equal(&result.colors, &outgoing);

        let unique: HashSet<u64> = result.colors.iter().copied().collect();
        assert!(unique.len() <= 2);
    }

    #[test]
    fn k1_coloring_empty_graph() {
        let store = store_from_outgoing(vec![]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(10).run().unwrap();
        assert!(result.colors.is_empty());
        assert_eq!(result.ran_iterations, 0);
        assert!(result.did_converge);
    }
}
