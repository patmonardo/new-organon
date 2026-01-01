//! Java: `ActivationFunctionFactory`.

use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::relu_wrapper::ReluWrapper;
use super::sigmoid_wrapper::SigmoidWrapper;
use super::types::ActivationFunctionType;

pub struct ActivationFunctionFactory;

impl ActivationFunctionFactory {
    pub fn activation_function_wrapper(
        activation_function_type: ActivationFunctionType,
    ) -> Box<dyn ActivationFunctionWrapper> {
        match activation_function_type {
            ActivationFunctionType::Sigmoid => Box::new(SigmoidWrapper),
            ActivationFunctionType::Relu => Box::new(ReluWrapper),
        }
    }
}
