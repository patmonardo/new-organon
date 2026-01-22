//! Label Propagation integration tests

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

    fn store_from_outgoing(outgoing: Vec<Vec<(i64, f64)>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut out_ids: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (s, targets) in outgoing.iter().enumerate() {
            for &(t, _w) in targets {
                out_ids[s].push(t);
            }
        }

        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in out_ids.iter().enumerate() {
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
        relationship_topologies
            .insert(rel_type, RelationshipTopology::new(out_ids, Some(incoming)));

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
    fn label_propagation_converges_on_two_components() {
        // Two disconnected edges: 0-1 and 2-3
        let outgoing = vec![
            vec![(1, 1.0)],
            vec![(0, 1.0)],
            vec![(3, 1.0)],
            vec![(2, 1.0)],
        ];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.label_propagation().max_iterations(20).run().unwrap();

        assert!(result.did_converge);
        assert_eq!(result.labels.len(), 4);

        // Each edge should end up with a shared label.
        assert_eq!(result.labels[0], result.labels[1]);
        assert_eq!(result.labels[2], result.labels[3]);
        assert_ne!(result.labels[0], result.labels[2]);
    }

    #[test]
    fn label_propagation_tie_breaks_to_smallest_label() {
        // Star: 0 connected to 1 and 2 (symmetric).
        // With identity init labels, node 0 sees labels {1,2} equal weight -> picks 1.
        let outgoing = vec![vec![(1, 1.0), (2, 1.0)], vec![(0, 1.0)], vec![(0, 1.0)]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.label_propagation().max_iterations(1).run().unwrap();
        assert_eq!(result.labels[0], 1);
    }
}
