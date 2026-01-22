//! L2 norm squared function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions L2NormSquared.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::dimensions;
use crate::ml::core::AbstractVariable;
use crate::ml::core::ComputationContext;
use crate::ml::core::{size_in_bytes, Scalar, Tensor};
use crate::ml::core::{Variable, VariableRef};
use std::fmt;

/// L2 norm squared of a tensor.
///
/// Computes the sum of squared elements. Corresponds to L2NormSquared in Java GDS.
/// Uses composition with AbstractVariable to match Java's inheritance pattern.
pub struct L2NormSquared {
    base: AbstractVariable,
    parent: VariableRef,
}

impl L2NormSquared {
    pub fn new(parent: Box<dyn Variable>) -> Self {
        Self::new_ref(parent.into())
    }

    pub fn new_ref(parent: VariableRef) -> Self {
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(
            vec![parent.clone()],
            dimensions::scalar(),
            require_gradient,
        );
        Self { base, parent }
    }

    pub fn size_in_bytes_of_apply() -> usize {
        size_in_bytes(&[1])
    }

    fn gradient_for_parent(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let self_gradient = ctx
            .gradient(self)
            .expect("Self gradient not computed")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Self gradient must be Scalar")
            .value();

        let parent_data = ctx
            .data(self.parent.as_ref())
            .expect("Parent data not computed");

        parent_data.scalar_multiply(2.0 * self_gradient)
    }
}

impl Variable for L2NormSquared {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        let parent_matrix = ctx
            .data(self.parent.as_ref())
            .expect("Parent data not computed");

        let length = parent_matrix.total_size();
        let mut l2_norm = 0.0;

        for idx in 0..length {
            let value = parent_matrix.data()[idx];
            l2_norm += value * value;
        }

        Box::new(Scalar::new(l2_norm))
    }

    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        assert!(
            std::ptr::eq(parent, self.parent.as_ref()),
            "Gradient requested for unknown parent"
        );
        self.gradient_for_parent(ctx)
    }

    // DELEGATION: Forward to AbstractVariable
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    // DELEGATION: Forward to AbstractVariable
    fn parents(&self) -> &[VariableRef] {
        self.base.parents()
    }

    // DELEGATION: Forward to AbstractVariable
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }
}

impl fmt::Display for L2NormSquared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L2NormSquared")
    }
}
