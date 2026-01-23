use crate::applications::graph_store_catalog::results::GraphStreamNodePropertiesResult;
use crate::projection::NodeLabel;
use crate::types::graph::id_map::IdMap;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::properties::node::node_property_container::NodePropertyContainer;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::collections::HashSet;

/// StreamNodePropertiesApplication
///
/// Java parity: streams node property values from a Graph view produced by the GraphStore.
pub struct StreamNodePropertiesApplication;

impl StreamNodePropertiesApplication {
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        node_properties: &[String],
        node_labels: &[NodeLabel],
        list_node_labels: bool,
    ) -> Result<Vec<GraphStreamNodePropertiesResult>, String> {
        // DefaultGraph parity: stream from the Graph view.
        let graph = graph_store.graph();
        let mut out: Vec<GraphStreamNodePropertiesResult> = Vec::new();

        let label_filter: HashSet<NodeLabel> = node_labels.iter().cloned().collect();
        // Java parity: ElementProjection.PROJECT_ALL is "*".
        // We also treat NodeLabel::all_nodes() ("__ALL__") as wildcard.
        let filter_all = label_filter.contains(&NodeLabel::all_nodes())
            || label_filter.contains(&NodeLabel::of("*"));

        let node_count = graph.node_count() as i64;
        for mapped_node_id in 0..node_count {
            if !label_filter.is_empty() && !filter_all {
                let labels = graph.node_labels(mapped_node_id);
                let any_match = labels.iter().any(|l| label_filter.contains(l));
                if !any_match {
                    continue;
                }
            }

            let original_id = graph
                .to_original_node_id(mapped_node_id)
                .unwrap_or(mapped_node_id);

            let node_labels: Vec<String> = if list_node_labels {
                graph
                    .node_labels(mapped_node_id)
                    .into_iter()
                    .map(|l: NodeLabel| l.name().to_string())
                    .collect()
            } else {
                Vec::new()
            };

            for prop_key in node_properties.iter() {
                let pv: Option<std::sync::Arc<dyn NodePropertyValues>> =
                    graph.node_properties(prop_key.as_str());
                let value = match pv {
                    None => serde_json::Value::Null,
                    Some(values) => match values.value_type() {
                        ValueType::Long => values
                            .long_value(mapped_node_id as u64)
                            .ok()
                            .map(|v| serde_json::Value::Number(serde_json::Number::from(v)))
                            .unwrap_or(serde_json::Value::Null),
                        ValueType::Double => values
                            .double_value(mapped_node_id as u64)
                            .ok()
                            .and_then(|v| {
                                serde_json::Number::from_f64(v).map(serde_json::Value::Number)
                            })
                            .unwrap_or(serde_json::Value::Null),
                        _ => {
                            // Note: array/object node property values are deferred until stable JSON encoding exists.
                            serde_json::Value::Null
                        }
                    },
                };

                out.push(GraphStreamNodePropertiesResult::new(
                    original_id,
                    prop_key.clone(),
                    value,
                    node_labels.clone(),
                ));
            }
        }

        Ok(out)
    }
}
