//! Java: `ActivationFunctionWrapper`.

use super::activation_function::ActivationFunction;
use super::types::ActivationFunctionType;

pub trait ActivationFunctionWrapper: Send + Sync {
    fn activation_function(&self) -> Box<dyn ActivationFunction>;
    fn weight_init_bound(&self, rows: usize, cols: usize) -> f64;
    fn activation_function_type(&self) -> ActivationFunctionType;
}
