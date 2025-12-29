//! Java: `MaxPoolAggregatingLayer`.

use crate::ml::core::functions::Weights;
use std::sync::Arc;

use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::layer::Layer;
use super::max_pooling_aggregator::MaxPoolingAggregator;

pub struct MaxPoolAggregatingLayer {
    sample_size: usize,
    pool_weights: Arc<Weights>,
    self_weights: Arc<Weights>,
    neighbors_weights: Arc<Weights>,
    bias: Arc<Weights>,
    activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
}

impl MaxPoolAggregatingLayer {
    pub fn new(
        sample_size: usize,
        pool_weights: Arc<Weights>,
        self_weights: Arc<Weights>,
        neighbors_weights: Arc<Weights>,
        bias: Arc<Weights>,
        activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
    ) -> Self {
        Self {
            sample_size,
            pool_weights,
            self_weights,
            neighbors_weights,
            bias,
            activation_function_wrapper,
        }
    }
}

impl Layer for MaxPoolAggregatingLayer {
    fn sample_size(&self) -> usize {
        self.sample_size
    }

    fn aggregator(&self) -> Box<dyn super::aggregator::Aggregator> {
        Box::new(MaxPoolingAggregator::new(
            Arc::clone(&self.pool_weights),
            Arc::clone(&self.self_weights),
            Arc::clone(&self.neighbors_weights),
            Arc::clone(&self.bias),
            Arc::clone(&self.activation_function_wrapper),
        ))
    }
}
