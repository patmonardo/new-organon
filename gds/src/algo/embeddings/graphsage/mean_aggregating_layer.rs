//! Java: `MeanAggregatingLayer`.

use crate::ml::core::functions::Weights;
use std::sync::Arc;

use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::layer::Layer;
use super::mean_aggregator::MeanAggregator;

pub struct MeanAggregatingLayer {
    sample_size: usize,
    weights: Arc<Weights>,
    activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
}

impl MeanAggregatingLayer {
    pub fn new(
        weights: Arc<Weights>,
        sample_size: usize,
        activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
    ) -> Self {
        Self {
            sample_size,
            weights,
            activation_function_wrapper,
        }
    }
}

impl Layer for MeanAggregatingLayer {
    fn sample_size(&self) -> usize {
        self.sample_size
    }

    fn aggregator(&self) -> Box<dyn super::aggregator::Aggregator> {
        Box::new(MeanAggregator::new(
            Arc::clone(&self.weights),
            Arc::clone(&self.activation_function_wrapper),
        ))
    }
}
