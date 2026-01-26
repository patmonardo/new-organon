//! CollapsePath storage runtime.
//!
//! Translation source: `org.neo4j.gds.walking.CollapsePathAlgorithmFactory`.
//!
//! Responsible for constructing per-relationship-type graph views and invoking
//! the computation runtime.

use super::CollapsePathComputationRuntime;
use super::spec::{CollapsePathConfig, CollapsePathResult};
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::MappedNodeId;
use crate::types::graph::Graph;
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::prelude::DefaultGraphStore;
use crate::types::schema::Direction;
use std::collections::HashSet;
use std::sync::Arc;

pub struct CollapsePathStorageRuntime {
    concurrency: usize,
}

impl CollapsePathStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        config: &CollapsePathConfig,
        computation: &mut CollapsePathComputationRuntime,
    ) -> Result<CollapsePathResult, String> {
        if config.path_templates.is_empty() {
            return Err("pathTemplates must be non-empty".to_string());
        }

        let mut path_template_graphs: Vec<Vec<Arc<dyn Graph>>> = Vec::new();

        for path in &config.path_templates {
            if path.is_empty() {
                return Err(
                    "Each path template must contain at least one relationship type".to_string(),
                );
            }

            let mut graphs_for_path: Vec<Arc<dyn Graph>> = Vec::new();
            let mut expected_node_count: Option<usize> = None;

            for rel_name in path {
                let rel_type = RelationshipType::of(rel_name);
                let mut rel_set = HashSet::new();
                rel_set.insert(rel_type);

                let graph = graph_store
                    .get_graph_with_types_and_orientation(&rel_set, Orientation::Natural)
                    .map_err(|e| {
                        format!("failed to build graph for relationship type '{rel_name}': {e}")
                    })?;

                if let Some(expected) = expected_node_count {
                    if graph.node_count() != expected {
                        return Err(format!(
                            "path template mixes relationship types with differing node counts (expected {expected}, got {})",
                            graph.node_count()
                        ));
                    }
                } else {
                    expected_node_count = Some(graph.node_count());
                }

                graphs_for_path.push(graph);
            }

            path_template_graphs.push(graphs_for_path);
        }

        let edges = computation.compute(&path_template_graphs);
        let outgoing = build_outgoing(graph_store.node_count(), edges)?;
        let edges_for_result: Vec<(u64, u64)> = outgoing
            .iter()
            .enumerate()
            .flat_map(|(src, targets)| targets.iter().map(move |t| (src as u64, *t as u64)))
            .collect();

        let store = graph_store
            .with_added_relationship_type(
                GraphName::new(&config.mutate_graph_name),
                RelationshipType::of(&config.mutate_relationship_type),
                outgoing,
                Direction::Directed,
            )
            .map_err(|e| e.to_string())?;

        Ok(CollapsePathResult {
            graph_name: config.mutate_graph_name.clone(),
            mutate_relationship_type: config.mutate_relationship_type.clone(),
            node_count: store.node_count() as u64,
            relationship_count: store.relationship_count() as u64,
            edges: edges_for_result,
            graph_store: store,
        })
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

fn build_outgoing(
    node_count: usize,
    mut edges: Vec<(u64, u64)>,
) -> Result<Vec<Vec<MappedNodeId>>, String> {
    let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
    for (src, tgt) in edges.drain(..) {
        let src_usize = src as usize;
        let tgt_usize = tgt as usize;
        if src_usize >= node_count || tgt_usize >= node_count {
            return Err(format!(
                "edge ({src},{tgt}) is out of bounds for node_count={node_count}"
            ));
        }
        outgoing[src_usize].push(tgt as MappedNodeId);
    }

    for adj in outgoing.iter_mut() {
        adj.sort_unstable();
        adj.dedup();
    }

    Ok(outgoing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};
    use crate::types::schema::{GraphSchema, MutableGraphSchema, NodeLabel};
    use std::collections::{HashMap, HashSet};

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
        let rel = RelationshipType::of("R1");
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

        let outgoing = vec![vec![1], vec![2], vec![]];
        let mut topologies = HashMap::new();
        topologies.insert(rel, RelationshipTopology::new(outgoing, None));

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
    fn adds_collapsed_relationship_type() {
        let store = build_store();
        let mut computation = CollapsePathComputationRuntime::new(false);
        let runtime = CollapsePathStorageRuntime::new(2);

        let config = CollapsePathConfig {
            path_templates: vec![vec!["R1".to_string(), "R1".to_string()]],
            mutate_relationship_type: "C".to_string(),
            mutate_graph_name: "g_collapsed".to_string(),
            allow_self_loops: false,
            concurrency: 2,
        };

        let result = runtime.compute(&store, &config, &mut computation).unwrap();
        assert_eq!(result.graph_name, "g_collapsed");
        assert_eq!(result.mutate_relationship_type, "C");
        assert_eq!(
            result.relationship_count,
            store.relationship_count() as u64 + 1
        );

        let rel_graph = result
            .graph_store
            .get_graph_with_types_and_orientation(
                &HashSet::from([RelationshipType::of("C")]),
                Orientation::Natural,
            )
            .unwrap();

        let mut edges: Vec<(u64, u64)> = Vec::new();
        let fallback = rel_graph.default_property_value();
        for src in 0..rel_graph.node_count() {
            for cursor in rel_graph.stream_relationships(src as i64, fallback) {
                edges.push((src as u64, cursor.target_id() as u64));
            }
        }

        assert_eq!(edges, vec![(0, 2)]);
    }
}
