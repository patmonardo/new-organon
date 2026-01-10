use crate::applications::services::logging::Log;
use crate::projection::NodeLabel;
use crate::types::graph::IdMap as _;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore as _;

use crate::applications::graph_store_catalog::results::WriteResult;

/// WriteNodePropertiesApplication
///
/// Java parity: `WriteNodePropertiesApplication` orchestrates exporting node properties
/// back to the host database. In Rust pass-1 we don't have a DB integration yet, so this
/// computes *what would be written* from the current GraphStore.
#[derive(Clone, Debug)]
pub struct WriteNodePropertiesApplication {
    #[allow(dead_code)]
    log: Log,
}

impl WriteNodePropertiesApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        node_properties: &[String],
        node_labels: &[NodeLabel],
    ) -> Result<WriteResult, String> {
        if node_properties.is_empty() {
            return Err("node_properties must not be empty".to_string());
        }

        for k in node_properties.iter() {
            let t = k.trim();
            if t.is_empty() {
                return Err("node_properties must not contain empty property names".to_string());
            }
            if !graph_store.has_node_property(t) {
                return Err(format!("Unknown node property '{t}'"));
            }
        }

        let graph = graph_store.graph();

        let label_filter: std::collections::HashSet<NodeLabel> =
            node_labels.iter().cloned().collect();
        let filter_all = label_filter.is_empty()
            || label_filter.contains(&NodeLabel::all_nodes())
            || label_filter.contains(&NodeLabel::of("*"));

        let mut nodes_written: u64 = 0;
        let node_count = graph.node_count() as i64;
        for mapped_node_id in 0..node_count {
            if !filter_all {
                let labels = graph.node_labels(mapped_node_id);
                if !labels.iter().any(|l| label_filter.contains(l)) {
                    continue;
                }
            }
            nodes_written += 1;
        }

        let properties_written = nodes_written * node_properties.len() as u64;
        Ok(WriteResult::new(nodes_written, 0, properties_written))
    }
}

impl Default for WriteNodePropertiesApplication {
    fn default() -> Self {
        Self::new(Log::default())
    }
}
