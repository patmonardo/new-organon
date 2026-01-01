//! K-Core Integration Tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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

    #[test]
    fn kcore_simple_chain() {
        // Chain: 0-1-2-3 (all nodes have degree 1 or 2, so core values should be 1)
        let store = store_from_outgoing(vec![vec![1], vec![0, 2], vec![1, 3], vec![2]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();
        assert_eq!(result.core_values.len(), 4);
        assert_eq!(result.degeneracy, 1);
        // All nodes have degree <= 2, so max k-core is 1
        for &core_val in &result.core_values {
            assert!(core_val <= 2);
        }
    }

    #[test]
    fn kcore_triangle() {
        // Triangle: 0-1-2-0 (all nodes have degree 2, so 2-core)
        let store = store_from_outgoing(vec![vec![1, 2], vec![0, 2], vec![0, 1]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();
        assert_eq!(result.core_values.len(), 3);
        assert_eq!(result.degeneracy, 2);
        // All nodes are in 2-core
        for &core_val in &result.core_values {
            assert_eq!(core_val, 2);
        }
    }

    #[test]
    fn kcore_star_graph() {
        // Star: center node 0 connected to 1,2,3,4 (center has degree 4, leaves have degree 1)
        let store = store_from_outgoing(vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();
        assert_eq!(result.core_values.len(), 5);
        assert_eq!(result.degeneracy, 1);
        // All nodes have core value 1 (leaves removed first, then center)
        for &core_val in &result.core_values {
            assert_eq!(core_val, 1);
        }
    }

    #[test]
    fn kcore_empty_graph() {
        let store = store_from_outgoing(vec![]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();
        assert!(result.core_values.is_empty());
        assert_eq!(result.degeneracy, 0);
    }

    #[test]
    fn kcore_isolated_nodes() {
        // Three isolated nodes (no edges)
        let store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();
        assert_eq!(result.core_values.len(), 3);
        assert_eq!(result.degeneracy, 0);
        // All nodes have degree 0, so core value is 0
        for &core_val in &result.core_values {
            assert_eq!(core_val, 0);
        }
    }
}
