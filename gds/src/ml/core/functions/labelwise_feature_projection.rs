//! Labelwise feature projection for GraphSAGE multi-label training.
//!
//! Java: `LabelwiseFeatureProjection`.
//!
//! This variable projects per-node feature vectors into a fixed dimensional space using
//! a label-specific weight matrix.
//!
//! For each node \(i\) with label \(\ell_i\):
//! \[
//! y_i = x_i \cdot W_{\ell_i}^T
//! \]
//! where \(W_{\ell}\) has shape (projected_dim, feature_dim_for_label).

use crate::collections::HugeObjectArray;
use crate::ml::core::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::{Matrix, Tensor};
use crate::ml::core::{Variable, VariableRef};
use crate::types::schema::NodeLabel;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::weights::Weights;

pub struct LabelwiseFeatureProjection {
    node_ids: Vec<u64>,
    features: Arc<HugeObjectArray<Vec<f64>>>,
    labels: Vec<NodeLabel>,
    projected_feature_dimension: usize,

    // Stable parent ordering.
    label_to_parent_idx: HashMap<NodeLabel, usize>,
    weight_parents: Vec<Arc<Weights>>,
    parents: Vec<VariableRef>,
    require_gradient: bool,
    dims: Vec<usize>,
}

impl LabelwiseFeatureProjection {
    pub fn new(
        node_ids: Vec<u64>,
        features: Arc<HugeObjectArray<Vec<f64>>>,
        weights_by_label: HashMap<NodeLabel, Arc<Weights>>,
        projected_feature_dimension: usize,
        labels: Vec<NodeLabel>,
    ) -> Self {
        assert_eq!(
            node_ids.len(),
            labels.len(),
            "labels must match node_ids length"
        );

        // Deterministic order: sort labels by string.
        let mut entries: Vec<(NodeLabel, Arc<Weights>)> = weights_by_label.into_iter().collect();
        entries.sort_by(|(a, _), (b, _)| a.name().cmp(b.name()));

        let mut label_to_parent_idx = HashMap::new();
        let mut weight_parents: Vec<Arc<Weights>> = Vec::with_capacity(entries.len());
        let mut parents: Vec<VariableRef> = Vec::with_capacity(entries.len());
        let mut require_gradient = false;
        for (idx, (label, w)) in entries.into_iter().enumerate() {
            require_gradient |= w.require_gradient();
            label_to_parent_idx.insert(label, idx);
            parents.push(Arc::clone(&w) as VariableRef);
            weight_parents.push(w);
        }

        let dims = dimensions::matrix(node_ids.len(), projected_feature_dimension);

        Self {
            node_ids,
            features,
            labels,
            projected_feature_dimension,
            label_to_parent_idx,
            weight_parents,
            parents,
            require_gradient,
            dims,
        }
    }

    fn weight_for_label(&self, label: &NodeLabel) -> &Weights {
        let idx = *self
            .label_to_parent_idx
            .get(label)
            .unwrap_or_else(|| panic!("Missing weights for label `{}`", label.name()));
        self.weight_parents[idx].as_ref()
    }
}

impl Variable for LabelwiseFeatureProjection {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let mut out = Matrix::create(0.0, self.node_ids.len(), self.projected_feature_dimension);

        for (row, (&node_id, label)) in self.node_ids.iter().zip(self.labels.iter()).enumerate() {
            let x = self.features.get(node_id as usize);

            let w_tensor = ctx
                .data(self.weight_for_label(label))
                .expect("weights not computed");
            let w = w_tensor
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("weights must be Matrix");

            assert_eq!(
                w.rows(),
                self.projected_feature_dimension,
                "weights rows must equal projected_feature_dimension"
            );
            assert_eq!(
                w.cols(),
                x.len(),
                "weights cols must match feature length for label `{}`",
                label.name()
            );

            for out_col in 0..self.projected_feature_dimension {
                let mut sum = 0.0;
                for (k, &x_val) in x.iter().enumerate() {
                    sum += x_val * w.data_at(out_col, k);
                }
                out.set_data_at(row, out_col, sum);
            }
        }

        Box::new(out)
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_data = ctx.data(parent).expect("parent data not computed");
        let parent_tensor = parent_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("parent weights must be Matrix");

        let self_grad_data = ctx.gradient(self).expect("self gradient not computed");
        let self_grad = self_grad_data
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("self gradient must be Matrix");

        let mut grad_w = Matrix::with_dimensions(parent_tensor.rows(), parent_tensor.cols());

        // Determine which label this parent corresponds to (by identity match).
        let mut parent_idx = None;
        for (i, p) in self.parents.iter().enumerate() {
            if std::ptr::eq(p.as_ref(), parent) {
                parent_idx = Some(i);
                break;
            }
        }
        let parent_idx = parent_idx.expect("unknown parent");

        // For each batch row that uses this label, accumulate outer product.
        for (row, (&node_id, label)) in self.node_ids.iter().zip(self.labels.iter()).enumerate() {
            let idx = *self
                .label_to_parent_idx
                .get(label)
                .expect("label missing from mapping");
            if idx != parent_idx {
                continue;
            }

            let x = self.features.get(node_id as usize);
            for out_row in 0..grad_w.rows() {
                let dy = self_grad.data_at(row, out_row);
                for (k, &x_val) in x.iter().enumerate().take(grad_w.cols()) {
                    grad_w.add_data_at(out_row, k, dy * x_val);
                }
            }
        }

        Box::new(grad_w)
    }

    fn require_gradient(&self) -> bool {
        self.require_gradient
    }

    fn parents(&self) -> &[VariableRef] {
        &self.parents
    }

    fn dimensions(&self) -> &[usize] {
        &self.dims
    }
}

impl fmt::Display for LabelwiseFeatureProjection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LabelwiseFeatureProjection")
    }
}
