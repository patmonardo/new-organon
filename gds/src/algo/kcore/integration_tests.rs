//! K-Core Decomposition integration tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::algo::kcore::KCoreComputationRuntime;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
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
    fn kcore_triggers_rebuild_on_sparse_graph() {
        // Force remainingNodes < ceil(REBUILD_CONSTANT * nodeCount) at the start:
        // - 197 isolated nodes (degree 0 => core 0, not counted as remaining)
        // - 3-node clique (degree 2 => remaining = 3)
        // For nodeCount=200, rebuildLimit=ceil(0.02*200)=4, so rebuild runs immediately.

        let node_count = 200usize;
        let a = 197usize;
        let b = 198usize;
        let c = 199usize;

        let neighbors = move |node: usize| -> Vec<usize> {
            if node == a {
                vec![b, c]
            } else if node == b {
                vec![a, c]
            } else if node == c {
                vec![a, b]
            } else {
                Vec::new()
            }
        };

        let mut runtime = KCoreComputationRuntime::new().concurrency(4);
        let result = runtime.compute(node_count, neighbors);

        assert_eq!(runtime.rebuild_count(), 1);
        assert_eq!(result.degeneracy, 2);

        // Isolated nodes are 0-core.
        for v in 0..197 {
            assert_eq!(result.core_values[v], 0);
        }
        // Clique nodes are 2-core.
        assert_eq!(result.core_values[a], 2);
        assert_eq!(result.core_values[b], 2);
        assert_eq!(result.core_values[c], 2);
    }

    #[test]
    fn kcore_triangle_is_2_core() {
        let outgoing = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();

        assert_eq!(result.degeneracy, 2);
        assert_eq!(result.core_values, vec![2, 2, 2]);
    }

    #[test]
    fn kcore_path_is_1_core() {
        let outgoing = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();

        assert_eq!(result.degeneracy, 1);
        assert_eq!(result.core_values, vec![1, 1, 1, 1]);
    }

    #[test]
    fn kcore_isolated_node_is_0_core() {
        let outgoing = vec![vec![]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.kcore().run().unwrap();

        assert_eq!(result.degeneracy, 0);
        assert_eq!(result.core_values, vec![0]);
    }
}
