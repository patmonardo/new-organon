//! CollapsePath storage runtime.
//!
//! Translation source: `org.neo4j.gds.walking.CollapsePathAlgorithmFactory`.
//!
//! Responsible for constructing per-relationship-type graph views and invoking
//! the computation runtime.

use super::computation::CollapsePathComputationRuntime;
use super::spec::{CollapsePathConfig, CollapsePathResult};
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::graph_store::GraphStore;
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
        graph_store: &impl GraphStore,
        config: &CollapsePathConfig,
    ) -> Result<CollapsePathResult, String> {
        if config.path_templates.is_empty() {
            return Err("pathTemplates must be non-empty".to_string());
        }

        let mut path_template_graphs: Vec<Vec<Arc<dyn Graph>>> = Vec::new();

        for path in &config.path_templates {
            if path.is_empty() {
                return Err("Each path template must contain at least one relationship type".to_string());
            }

            let mut graphs_for_path: Vec<Arc<dyn Graph>> = Vec::new();
            let mut expected_node_count: Option<usize> = None;

            for rel_name in path {
                let rel_type = RelationshipType::of(rel_name);
                let mut rel_set = HashSet::new();
                rel_set.insert(rel_type);

                let graph = graph_store
                    .get_graph_with_types_and_orientation(&rel_set, Orientation::Natural)
                    .map_err(|e| format!("failed to build graph for relationship type '{rel_name}': {e}"))?;

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

        let computation = CollapsePathComputationRuntime::new(config.allow_self_loops);
        Ok(computation.compute(
            &path_template_graphs,
            &config.mutate_relationship_type,
        ))
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}
