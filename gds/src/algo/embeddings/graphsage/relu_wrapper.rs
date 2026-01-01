//! Java: `ReluWrapper`.

use crate::ml::core::functions::Relu;
use crate::ml::core::variable::VariableRef;
use std::sync::Arc;

use super::activation_function::ActivationFunction;
use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::types::ActivationFunctionType;

struct ReluFn;

impl ActivationFunction for ReluFn {
    fn apply(&self, input: VariableRef) -> VariableRef {
        Arc::new(Relu::with_default_alpha_ref(input))
    }
}

pub struct ReluWrapper;

impl ActivationFunctionWrapper for ReluWrapper {
    fn activation_function(&self) -> Box<dyn ActivationFunction> {
        Box::new(ReluFn)
    }

    fn weight_init_bound(&self, _rows: usize, cols: usize) -> f64 {
        (2.0f64 / cols as f64).sqrt()
    }

    fn activation_function_type(&self) -> ActivationFunctionType {
        ActivationFunctionType::Relu
    }
}
