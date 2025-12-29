//! Java: `LayerFactory`.

use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::{Matrix, Vector};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

use super::activation_function_factory::ActivationFunctionFactory;
use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::layer::Layer;
use super::max_pool_aggregating_layer::MaxPoolAggregatingLayer;
use super::mean_aggregating_layer::MeanAggregatingLayer;
use super::types::{AggregatorType, LayerConfig};

pub struct LayerFactory;

impl LayerFactory {
    pub fn create_layer(layer_config: &LayerConfig) -> Arc<dyn Layer> {
        let rows = layer_config.rows;
        let cols = layer_config.cols;

        let activation_wrapper = ActivationFunctionFactory::activation_function_wrapper(
            layer_config.activation_function,
        );
        let activation_wrapper: Arc<dyn ActivationFunctionWrapper> = Arc::from(activation_wrapper);

        let random_seed = layer_config.random_seed;
        let weights = Arc::new(generate_weights(
            rows,
            cols,
            activation_wrapper.weight_init_bound(rows, cols),
            random_seed,
        ));

        match layer_config.aggregator_type {
            AggregatorType::Mean => Arc::new(MeanAggregatingLayer::new(
                weights,
                layer_config.sample_size,
                activation_wrapper,
            )),
            AggregatorType::Pool => {
                let pool_weights = weights;
                let self_weights = Arc::new(generate_weights(
                    rows,
                    cols,
                    activation_wrapper.weight_init_bound(rows, cols),
                    random_seed + 1,
                ));
                let neighbors_weights = Arc::new(generate_weights(
                    rows,
                    rows,
                    activation_wrapper.weight_init_bound(rows, rows),
                    random_seed + 2,
                ));

                let bias = Arc::new(Weights::new(Box::new(Vector::create(0.0, rows))));

                Arc::new(MaxPoolAggregatingLayer::new(
                    layer_config.sample_size,
                    pool_weights,
                    self_weights,
                    neighbors_weights,
                    bias,
                    activation_wrapper,
                ))
            }
        }
    }
}

pub fn generate_weights(rows: usize, cols: usize, weight_bound: f64, random_seed: u64) -> Weights {
    let mut rng = ChaCha8Rng::seed_from_u64(random_seed);
    let mut data = Vec::with_capacity(rows * cols);
    for _ in 0..(rows * cols) {
        data.push(rng.gen_range(-weight_bound..weight_bound));
    }
    Weights::new(Box::new(Matrix::new(data, rows, cols)))
}
