//! K-Means Integration Tests
//!
//! These tests validate K-Means behavior via the Graph facade and DefaultGraphStore.

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    use crate::collections::backends::vec::VecDoubleArray;
    use crate::procedures::facades::Graph;
    use crate::procedures::kmeans::KMeansSamplerType;
    use crate::projection::RelationshipType;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::GraphStore;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::node::DefaultDoubleArrayNodePropertyValues;
    use crate::types::properties::node::NodePropertyValues;
    use crate::types::schema::{Direction, MutableGraphSchema, NodeLabel};

    fn store_with_features(outgoing: Vec<Vec<i64>>, features: Vec<Vec<f64>>) -> DefaultGraphStore {
        let node_count = outgoing.len();
        assert_eq!(node_count, features.len());

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

        let mut store = DefaultGraphStore::new(
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
        );

        let data: Vec<Option<Vec<f64>>> = features.into_iter().map(Some).collect();
        let backend = VecDoubleArray::from(data);
        let pv = DefaultDoubleArrayNodePropertyValues::from_collection(backend, node_count);
        let pv: Arc<dyn NodePropertyValues> = Arc::new(pv);

        store
            .add_node_property(HashSet::<NodeLabel>::new(), "features".to_string(), pv)
            .unwrap();

        store
    }

    #[test]
    fn k_means_seeded_separates_two_clouds() {
        // Graph structure irrelevant; keep it simple.
        let outgoing = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

        // Two obvious clusters in 2D.
        let features = vec![
            vec![0.0, 0.0],
            vec![0.1, -0.1],
            vec![-0.1, 0.1],
            vec![10.0, 10.0],
            vec![9.9, 10.2],
            vec![10.1, 9.8],
        ];

        let store = store_with_features(outgoing, features);
        let graph = Graph::new(Arc::new(store));

        let result = graph
            .kmeans()
            .node_property("features")
            .k(2)
            .max_iterations(20)
            .number_of_restarts(1)
            .seed_centroids(vec![vec![0.0, 0.0], vec![10.0, 10.0]])
            .run()
            .unwrap();

        assert_eq!(result.communities.len(), 6);

        // First 3 should match; last 3 should match; and across groups should differ.
        assert_eq!(result.communities[0], result.communities[1]);
        assert_eq!(result.communities[1], result.communities[2]);
        assert_eq!(result.communities[3], result.communities[4]);
        assert_eq!(result.communities[4], result.communities[5]);
        assert_ne!(result.communities[0], result.communities[3]);

        // Sanity: centers have correct shape.
        assert_eq!(result.centers.len(), 2);
        assert_eq!(result.centers[0].len(), 2);
        assert_eq!(result.centers[1].len(), 2);
    }

    #[test]
    fn k_means_kmeanspp_is_deterministic_with_seed() {
        let outgoing = vec![vec![], vec![], vec![], vec![]];
        let features = vec![
            vec![0.0, 0.0],
            vec![0.0, 0.2],
            vec![10.0, 10.0],
            vec![10.0, 10.2],
        ];

        let store = store_with_features(outgoing, features);
        let graph = Graph::new(Arc::new(store));

        let a = graph
            .kmeans()
            .node_property("features")
            .k(2)
            .sampler_type(KMeansSamplerType::KmeansPlusPlus)
            .random_seed(123)
            .run()
            .unwrap();

        let b = graph
            .kmeans()
            .node_property("features")
            .k(2)
            .sampler_type(KMeansSamplerType::KmeansPlusPlus)
            .random_seed(123)
            .run()
            .unwrap();

        assert_eq!(a.communities, b.communities);
        assert_eq!(a.centers, b.centers);
    }

    #[test]
    fn k_means_silhouette_optional() {
        let outgoing = vec![vec![], vec![], vec![], vec![]];
        let features = vec![
            vec![0.0, 0.0],
            vec![0.0, 0.1],
            vec![10.0, 10.0],
            vec![10.0, 10.1],
        ];

        let store = store_with_features(outgoing, features);
        let graph = Graph::new(Arc::new(store));

        let result = graph
            .kmeans()
            .node_property("features")
            .k(2)
            .compute_silhouette(true)
            .seed_centroids(vec![vec![0.0, 0.0], vec![10.0, 10.0]])
            .run()
            .unwrap();

        let silhouette = result.silhouette.expect("silhouette enabled");
        assert_eq!(silhouette.len(), 4);
        assert!(result.average_silhouette >= -1.0);
        assert!(result.average_silhouette <= 1.0);
    }
}
