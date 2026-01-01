//! Java: `MultiLabelFeatureFunction`.
//!
//! This depends on ml-core `LabelwiseFeatureProjection`.

use crate::collections::HugeObjectArray;
use crate::ml::core::functions::LabelwiseFeatureProjection;
use crate::ml::core::functions::Weights;
use crate::ml::core::variable::VariableRef;
use crate::types::graph::Graph;
use crate::types::schema::NodeLabel;
use std::collections::HashMap;
use std::sync::Arc;

use super::feature_function::FeatureFunction;

pub struct MultiLabelFeatureFunction {
    #[allow(dead_code)]
    weights_by_label: HashMap<NodeLabel, Arc<Weights>>,
    #[allow(dead_code)]
    projected_feature_dimension: usize,
}

impl MultiLabelFeatureFunction {
    pub fn new(
        weights_by_label: HashMap<NodeLabel, Arc<Weights>>,
        projected_feature_dimension: usize,
    ) -> Self {
        Self {
            weights_by_label,
            projected_feature_dimension,
        }
    }
}

impl FeatureFunction for MultiLabelFeatureFunction {
    fn apply(
        &self,
        graph: Arc<dyn Graph>,
        node_ids: &[u64],
        features: Arc<HugeObjectArray<Vec<f64>>>,
    ) -> VariableRef {
        // Java expects that each node has exactly one label (validated during feature init).
        let mut labels: Vec<NodeLabel> = Vec::with_capacity(node_ids.len());
        for &node_id in node_ids {
            let label = single_label_of(graph.as_ref(), node_id);
            labels.push(label);
        }

        Arc::new(LabelwiseFeatureProjection::new(
            node_ids.to_vec(),
            features,
            self.weights_by_label.clone(),
            self.projected_feature_dimension,
            labels,
        ))
    }
}

fn single_label_of(graph: &dyn Graph, node_id: u64) -> NodeLabel {
    let mut label_ref: Option<NodeLabel> = None;
    let mut count = 0usize;
    graph.for_each_node_label(node_id as i64, &mut |node_label: &NodeLabel| {
        count += 1;
        if count == 1 {
            label_ref = Some(node_label.clone());
        }
        // stop as soon as we detect a second label
        count < 2
    });

    if count != 1 {
        panic!(
            "Each node must have exactly one label: nodeId={}, labels={:?}",
            node_id,
            graph.node_labels(node_id as i64)
        );
    }

    label_ref.expect("label missing")
}
