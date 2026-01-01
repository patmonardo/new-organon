//! Java: `SigmoidWrapper`.

use crate::ml::core::functions::Sigmoid;
use crate::ml::core::variable::VariableRef;
use std::sync::Arc;

use super::activation_function::ActivationFunction;
use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::types::ActivationFunctionType;

struct SigmoidFn;

impl ActivationFunction for SigmoidFn {
    fn apply(&self, input: VariableRef) -> VariableRef {
        Arc::new(Sigmoid::new_ref(input))
    }
}

pub struct SigmoidWrapper;

impl ActivationFunctionWrapper for SigmoidWrapper {
    fn activation_function(&self) -> Box<dyn ActivationFunction> {
        Box::new(SigmoidFn)
    }

    fn weight_init_bound(&self, rows: usize, cols: usize) -> f64 {
        (2.0f64 / (rows as f64 + cols as f64)).sqrt()
    }

    fn activation_function_type(&self) -> ActivationFunctionType {
        ActivationFunctionType::Sigmoid
    }
}
