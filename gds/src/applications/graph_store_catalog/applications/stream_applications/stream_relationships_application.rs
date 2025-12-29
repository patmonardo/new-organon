use std::collections::HashSet;

use crate::applications::graph_store_catalog::results::TopologyResult;
use crate::projection::RelationshipType;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;

/// StreamRelationshipsApplication
///
/// Java parity: `StreamRelationshipsApplication.compute(GraphStore, GraphStreamRelationshipsConfig)`
/// runs `graphStore.getGraph(relationshipType)` and then streams relationships from the `Graph`.
pub struct StreamRelationshipsApplication;

impl StreamRelationshipsApplication {
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        relationship_types: &[RelationshipType],
    ) -> Result<Vec<TopologyResult>, String> {
        let mut out: Vec<TopologyResult> = Vec::new();

        // Java parity: ElementProjection.PROJECT_ALL is "*".
        let has_wildcard = relationship_types.iter().any(|t| t.name() == "*");
        let types: Vec<RelationshipType> = if relationship_types.is_empty() || has_wildcard {
            graph_store.relationship_types().into_iter().collect()
        } else {
            relationship_types.to_vec()
        };

        for rel_type in types.into_iter() {
            let mut filter: HashSet<RelationshipType> = HashSet::new();
            filter.insert(rel_type.clone());

            let graph = graph_store
                .get_graph_with_types(&filter)
                .map_err(|e| e.to_string())?;

            // Stream outgoing relationships for each mapped node id.
            let node_count = graph.node_count() as i64;
            for mapped_source in 0..node_count {
                let source_original = graph
                    .to_original_node_id(mapped_source)
                    .unwrap_or(mapped_source);
                for cursor in graph.stream_relationships(mapped_source, f64::NAN) {
                    let target_mapped = cursor.target_id();
                    let target_original = graph
                        .to_original_node_id(target_mapped)
                        .unwrap_or(target_mapped);
                    out.push(TopologyResult::new(
                        source_original,
                        target_original,
                        rel_type.name().to_string(),
                    ));
                }
            }
        }

        Ok(out)
    }
}
