use crate::applications::graph_store_catalog::results::WriteResult;
use crate::applications::services::logging::Log;
use crate::projection::NodeLabel;
use crate::types::graph::IdMap as _;
use crate::types::graph_store::DefaultGraphStore;

/// WriteNodeLabelApplication
///
/// Java parity: writes a node label back to the database for nodes matching a filter.
/// Rust pass-1: compute *what would be written* from the current GraphStore.
#[derive(Clone, Debug)]
pub struct WriteNodeLabelApplication {
    #[allow(dead_code)]
    log: Log,
}

impl WriteNodeLabelApplication {
    pub fn new(log: Log) -> Self {
        Self { log }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        node_labels: &[String],
    ) -> Result<WriteResult, String> {
        if node_labels.is_empty() {
            return Err("node_labels must not be empty".to_string());
        }

        let graph = graph_store.graph();

        let label_filter: std::collections::HashSet<NodeLabel> = node_labels
            .iter()
            .filter_map(|s| {
                let t = s.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(NodeLabel::of(t))
                }
            })
            .collect();

        if label_filter.is_empty() {
            return Err("node_labels must not contain only empty labels".to_string());
        }

        let filter_all = label_filter.contains(&NodeLabel::all_nodes())
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

        Ok(WriteResult::new(nodes_written, 0, 0))
    }
}

impl Default for WriteNodeLabelApplication {
    fn default() -> Self {
        Self::new(Log::default())
    }
}
