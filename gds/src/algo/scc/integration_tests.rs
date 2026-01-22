//! SCC Integration Tests
//!
//! Validates SCC behavior via the Graph facade and DefaultGraphStore fixture helpers.

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
    fn scc_single_cycle_is_one_component() {
        // 0 -> 1 -> 2 -> 0
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![0]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 1);
        assert_eq!(result.components.len(), 3);
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[1], result.components[2]);
    }

    #[test]
    fn scc_two_disjoint_cycles_are_two_components() {
        // (0 <-> 1) and (2 <-> 3)
        let store = store_from_outgoing(vec![vec![1], vec![0], vec![3], vec![2]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 2);
        assert_eq!(result.components.len(), 4);
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[2], result.components[3]);
        assert_ne!(result.components[0], result.components[2]);
    }

    #[test]
    fn scc_directed_chain_has_each_node_its_own_component() {
        // 0 -> 1 -> 2 -> 3
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![3], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 4);
        assert_eq!(result.components.len(), 4);
        assert_ne!(result.components[0], result.components[1]);
        assert_ne!(result.components[1], result.components[2]);
        assert_ne!(result.components[2], result.components[3]);
    }

    #[test]
    fn scc_isolated_nodes_each_form_component() {
        let store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 3);
        assert_eq!(result.components.len(), 3);
        assert_ne!(result.components[0], result.components[1]);
        assert_ne!(result.components[1], result.components[2]);
    }

    #[test]
    fn scc_empty_graph_has_no_components() {
        let store = store_from_outgoing(vec![]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 0);
        assert!(result.components.is_empty());
    }
}
