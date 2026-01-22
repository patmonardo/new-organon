//! Java: `GraphSageLoss`.

use crate::ml::core::abstract_variable::AbstractVariable;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::dimensions;
use crate::ml::core::functions::Sigmoid;
use crate::ml::core::relationship_weights::{RelationshipWeights, DEFAULT_VALUE};
use crate::ml::core::tensor::{Matrix, Scalar, Tensor};
use crate::ml::core::variable::{Variable, VariableRef};
use std::fmt;
use std::sync::Arc;

pub struct GraphSageLoss {
    base: AbstractVariable,
    relationship_weights: Arc<dyn RelationshipWeights>,
    combined_embeddings: VariableRef,
    batch: Vec<u64>,
    negative_sampling_factor: usize,
}

impl GraphSageLoss {
    // batch nodes (0), neighbor nodes(1), negative nodes(2)
    const SAMPLING_BUCKETS: usize = 3;
    const NEGATIVE_NODES_OFFSET: usize = 2;

    // Note: keep constant for now; make configurable to match Java later.
    const ALPHA: f64 = 1.0;

    pub fn new(
        relationship_weights: Arc<dyn RelationshipWeights>,
        combined_embeddings: VariableRef,
        batch: Vec<u64>,
        negative_sampling_factor: usize,
    ) -> Self {
        let base = AbstractVariable::with_gradient_requirement(
            vec![combined_embeddings.clone()],
            dimensions::scalar(),
            true,
        );
        Self {
            base,
            relationship_weights,
            combined_embeddings,
            batch,
            negative_sampling_factor,
        }
    }

    fn parent(&self) -> &dyn Variable {
        self.combined_embeddings.as_ref()
    }

    fn relationship_weight_factor(&self, node_id: u64, positive_node_id: u64) -> f64 {
        let w = self
            .relationship_weights
            .weight_with_default(node_id, positive_node_id, f64::NAN);
        let w = if w.is_nan() { DEFAULT_VALUE } else { w };
        w.powf(Self::ALPHA)
    }

    fn affinity(embedding_data: &Matrix, batch_idx: usize, other_batch_idx: usize) -> f64 {
        let cols = embedding_data.cols();
        let mut sum = 0.0;
        for col in 0..cols {
            sum += embedding_data.data_at(batch_idx, col)
                * embedding_data.data_at(other_batch_idx, col);
        }
        sum
    }

    fn logistic_function(affinity: f64) -> f64 {
        1.0 / (1.0 + affinity.exp())
    }
}

impl Variable for GraphSageLoss {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let embeddings_tensor = ctx.data(self.parent()).expect("embeddings not computed");
        let embeddings = embeddings_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("expected Matrix");

        let bucket_size = embeddings.rows() / Self::SAMPLING_BUCKETS;
        let negative_nodes_offset = Self::NEGATIVE_NODES_OFFSET * bucket_size;

        let mut loss = 0.0;
        for bucket_idx in 0..bucket_size {
            let positive_node_idx = bucket_idx + bucket_size;
            let negative_node_idx = bucket_idx + negative_nodes_offset;

            let pos_aff = Self::affinity(embeddings, bucket_idx, positive_node_idx);
            let neg_aff = Self::affinity(embeddings, bucket_idx, negative_node_idx);

            let weight = self
                .relationship_weight_factor(self.batch[bucket_idx], self.batch[positive_node_idx]);
            loss += -weight * (Sigmoid::sigmoid(pos_aff)).ln()
                - (self.negative_sampling_factor as f64) * (Sigmoid::sigmoid(-neg_aff)).ln();
        }

        Box::new(Scalar::new(loss / bucket_size.max(1) as f64))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        assert!(std::ptr::eq(parent, self.parent()), "unknown parent");

        let embeddings_tensor = ctx.data(self.parent()).expect("embeddings not computed");
        let embeddings = embeddings_tensor
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("expected Matrix");

        let mut grad = Matrix::with_dimensions(embeddings.rows(), embeddings.cols());

        let bucket_size = embeddings.rows() / Self::SAMPLING_BUCKETS;
        let negative_nodes_bucket_offset = Self::NEGATIVE_NODES_OFFSET * bucket_size;
        let cols = embeddings.cols();

        for bucket_idx in 0..bucket_size {
            let positive_node_idx = bucket_idx + bucket_size;
            let negative_node_idx = bucket_idx + negative_nodes_bucket_offset;

            let pos_aff = Self::affinity(embeddings, bucket_idx, positive_node_idx);
            let neg_aff = Self::affinity(embeddings, bucket_idx, negative_node_idx);

            let weight = self
                .relationship_weight_factor(self.batch[bucket_idx], self.batch[positive_node_idx]);
            let weighted_positive_logistic = weight * Self::logistic_function(pos_aff);
            let weighted_negative_logistic =
                (self.negative_sampling_factor as f64) * Self::logistic_function(-neg_aff);

            for emb_idx in 0..cols {
                let scaled_pos =
                    -embeddings.data_at(positive_node_idx, emb_idx) * weighted_positive_logistic;
                let scaled_neg =
                    weighted_negative_logistic * embeddings.data_at(negative_node_idx, emb_idx);
                grad.set_data_at(bucket_idx, emb_idx, scaled_pos + scaled_neg);

                let current = embeddings.data_at(bucket_idx, emb_idx);
                grad.set_data_at(
                    positive_node_idx,
                    emb_idx,
                    -current * weighted_positive_logistic,
                );
                grad.set_data_at(
                    negative_node_idx,
                    emb_idx,
                    weighted_negative_logistic * current,
                );
            }
        }

        let grad: Box<dyn Tensor> = Box::new(grad);
        grad.scalar_multiply(1.0 / bucket_size.max(1) as f64)
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    fn parents(&self) -> &[VariableRef] {
        self.base.parents()
    }

    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for GraphSageLoss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GraphSageLoss")
    }
}
