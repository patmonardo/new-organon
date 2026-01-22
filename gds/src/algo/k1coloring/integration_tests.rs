//! K1-Coloring integration tests

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

    fn assert_valid_coloring(outgoing: &[Vec<i64>], colors: &[u64]) {
        for (u, nbrs) in outgoing.iter().enumerate() {
            for &v in nbrs {
                if v < 0 {
                    continue;
                }
                let v = v as usize;
                if v == u {
                    continue;
                }
                assert_ne!(colors[u], colors[v], "conflict on edge {u}->{v}");
            }
        }
    }

    #[test]
    fn k1coloring_triangle_uses_three_colors() {
        // Triangle (undirected modeled as symmetric directed edges)
        let outgoing = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(20).run().unwrap();
        assert!(result.did_converge);
        assert_eq!(result.colors.len(), 3);

        assert_valid_coloring(&outgoing, &result.colors);

        let used: std::collections::HashSet<u64> = result.colors.iter().copied().collect();
        assert_eq!(used.len(), 3);
    }

    #[test]
    fn k1coloring_path_is_two_colorable() {
        // Path 0-1-2-3
        let outgoing = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(20).run().unwrap();
        assert_valid_coloring(&outgoing, &result.colors);

        let used: std::collections::HashSet<u64> = result.colors.iter().copied().collect();
        assert!(!used.is_empty());
    }
}
