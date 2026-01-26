//! IndexInverse storage runtime.
//!
//! Translation source: `org.neo4j.gds.indexInverse.InverseRelationships`.
//! Builds a graph store with inverse relationship indices populated.

use super::IndexInverseComputationRuntime;
use super::spec::{IndexInverseConfig, IndexInverseResult};
use crate::projection::Orientation;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::prelude::DefaultGraphStore;
use crate::types::schema::RelationshipType;
use std::collections::HashSet;

pub struct IndexInverseStorageRuntime {
    concurrency: usize,
}

impl IndexInverseStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Build inverse indices for the configured relationship types.
    ///
    /// Note: current implementation applies to all relationship types in the
    /// store. Relationship-type filtering can be layered later if needed.
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        config: &IndexInverseConfig,
        _computation: &mut IndexInverseComputationRuntime,
    ) -> Result<IndexInverseResult, String> {
        let graph_name = GraphName::new(&config.mutate_graph_name);

        let selected_types: Option<HashSet<RelationshipType>> =
            if config.relationship_types.is_empty()
                || config.relationship_types.iter().any(|t| t == "*")
            {
                None
            } else {
                Some(
                    config
                        .relationship_types
                        .iter()
                        .map(|t| RelationshipType::of(t))
                        .collect(),
                )
            };

        let all_rel_types = graph_store.relationship_types();
        let target_types: Vec<RelationshipType> = match selected_types.as_ref() {
            Some(set) => all_rel_types
                .into_iter()
                .filter(|t| set.contains(t))
                .collect(),
            None => all_rel_types.into_iter().collect(),
        };

        let mut topologies = std::collections::HashMap::new();
        for rel_type in target_types {
            let mut rel_set = HashSet::new();
            rel_set.insert(rel_type.clone());

            let graph = graph_store
                .get_graph_with_types_and_orientation(&rel_set, Orientation::Natural)
                .map_err(|e| e.to_string())?;

            let (outgoing, incoming) = _computation.build_topology(graph.as_ref());
            let topology = RelationshipTopology::new(outgoing, Some(incoming));
            topologies.insert(rel_type, topology);
        }

        let new_store = graph_store
            .with_rebuilt_relationship_topologies(graph_name, topologies)
            .map_err(|e| e.to_string())?;

        Ok(IndexInverseResult {
            graph_name: config.mutate_graph_name.clone(),
            node_count: new_store.node_count() as u64,
            relationship_count: new_store.relationship_count() as u64,
            graph_store: new_store,
        })
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};
    use crate::types::schema::{
        Direction, GraphSchema, MutableGraphSchema, NodeLabel, RelationshipType,
    };
    use std::collections::HashMap;

    fn build_store() -> DefaultGraphStore {
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("g");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(NodeLabel::all_nodes());
        let rel1 = RelationshipType::of("R1");
        let rel2 = RelationshipType::of("R2");
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel1.clone(), Direction::Directed);
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel2.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

        let outgoing_r1 = vec![vec![1], vec![], vec![]];
        let outgoing_r2 = vec![vec![2], vec![], vec![]];
        let mut topologies = HashMap::new();
        topologies.insert(rel1, RelationshipTopology::new(outgoing_r1, None));
        topologies.insert(rel2, RelationshipTopology::new(outgoing_r2, None));

        DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        )
    }

    #[test]
    fn filters_selected_relationship_types() {
        let store = build_store();
        let mut computation = IndexInverseComputationRuntime::new();
        let runtime = IndexInverseStorageRuntime::new(2);

        let config = IndexInverseConfig {
            relationship_types: vec!["R1".to_string()],
            concurrency: 2,
            mutate_graph_name: "g_inv".to_string(),
        };

        let result = runtime.compute(&store, &config, &mut computation).unwrap();
        let graph = result.graph_store.graph();

        assert_eq!(result.graph_name, "g_inv");
        // Only R1 is inverse indexed; node 1 has one inverse edge, node 2 has none.
        assert_eq!(graph.degree_inverse(1).unwrap(), 1);
        assert_eq!(graph.degree_inverse(2).unwrap(), 0);
    }
}
