//! Label Propagation Integration Tests
//!
//! These tests validate label propagation behavior through the facade layer.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::procedures::facades::Graph;
    use crate::projection::RelationshipType;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_undirected_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for &(u, v) in edges {
            outgoing[u].push(v as i64);
            if u != v {
                outgoing[v].push(u as i64);
            }
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Undirected);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(rel_type, RelationshipTopology::new(outgoing, None));

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
    fn label_prop_empty_graph_is_ok() {
        let store = store_from_undirected_edges(0, &[]);
        let graph = Graph::new(Arc::new(store));

        let stats = graph.label_propagation().stats().unwrap();
        assert!(stats.did_converge);
        assert_eq!(stats.ran_iterations, 0);
        assert_eq!(stats.community_count, 0);
    }

    #[test]
    fn label_prop_single_node_converges_immediately() {
        let store = store_from_undirected_edges(1, &[]);
        let graph = Graph::new(Arc::new(store));

        let result = graph.label_propagation().run().unwrap();
        assert_eq!(result.labels, vec![0]);
        assert!(result.did_converge);
        assert_eq!(result.ran_iterations, 1);
    }

    #[test]
    fn label_prop_seed_property_sets_initial_labels() {
        let mut store = store_from_undirected_edges(4, &[(0, 1), (2, 3)]);
        store
            .add_node_property_i64("seed".to_string(), vec![100, 100, 200, 200])
            .unwrap();

        let graph = Graph::new(Arc::new(store));
        let result = graph
            .label_propagation()
            .seed_property("seed")
            .run()
            .unwrap();

        assert_eq!(result.labels[0], result.labels[1]);
        assert_eq!(result.labels[2], result.labels[3]);
        assert_ne!(result.labels[0], result.labels[2]);
    }

    #[test]
    fn label_prop_tie_breaks_to_smallest_label() {
        // Node 0 connected to 1 and 2, both neighbors vote with equal weight.
        // Labels: node1=20, node2=10 => node0 should pick 10.
        let mut store = store_from_undirected_edges(3, &[(0, 1), (0, 2)]);
        store
            .add_node_property_i64("seed".to_string(), vec![0, 20, 10])
            .unwrap();
        let graph = Graph::new(Arc::new(store));

        let result = graph
            .label_propagation()
            .seed_property("seed")
            .max_iterations(1)
            .run()
            .unwrap();

        assert_eq!(result.labels[0], 10);
    }

    #[test]
    fn label_prop_node_weight_property_biases_votes() {
        // Node 0 connected to 1 and 2.
        // Labels: 1->100, 2->200. Weights: node1=10.0, node2=1.0 => node0 should pick 100.
        let mut store = store_from_undirected_edges(3, &[(0, 1), (0, 2)]);
        store
            .add_node_property_i64("seed".to_string(), vec![0, 100, 200])
            .unwrap();
        store
            .add_node_property_f64("w".to_string(), vec![1.0, 10.0, 1.0])
            .unwrap();

        let graph = Graph::new(Arc::new(store));
        let result = graph
            .label_propagation()
            .seed_property("seed")
            .node_weight_property("w")
            .max_iterations(1)
            .run()
            .unwrap();

        assert_eq!(result.labels[0], 100);
    }
}
