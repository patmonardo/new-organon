//! Lazy constant function for ML in GDS.
//!
//! Translated from Java GDS ml-core functions LazyConstant.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::NotAFunctionException;
use crate::ml::core::ComputationContext;
use crate::ml::core::Tensor;
use crate::ml::core::{Variable, VariableRef};
use std::fmt;

/// A constant that lazily produces its value via a supplier function.
///
/// Corresponds to LazyConstant in Java GDS.
pub struct LazyConstant {
    data_producer: Box<dyn Fn() -> Box<dyn Tensor> + Send + Sync>,
    dimensions: Vec<usize>,
}

impl LazyConstant {
    pub fn new<F>(data_producer: F, expected_dimensions: Vec<usize>) -> Self
    where
        F: Fn() -> Box<dyn Tensor> + Send + Sync + 'static,
    {
        Self {
            data_producer: Box::new(data_producer),
            dimensions: expected_dimensions,
        }
    }
}

impl Variable for LazyConstant {
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        (self.data_producer)()
    }

    fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("{}", NotAFunctionException);
    }

    fn require_gradient(&self) -> bool {
        false
    }

    fn parents(&self) -> &[VariableRef] {
        &[]
    }

    fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
}

impl fmt::Display for LazyConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LazyConstant")
    }
}
