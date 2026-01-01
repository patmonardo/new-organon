//! Java: `TrainConfigTransformer`.

use super::graphsage_helper;
use super::types::{
    GraphSageTrainConfig, GraphSageTrainMemoryEstimateParameters, GraphSageTrainParameters,
};

pub struct TrainConfigTransformer;

impl TrainConfigTransformer {
    pub fn to_memory_estimate_parameters(
        config: &GraphSageTrainConfig,
    ) -> GraphSageTrainMemoryEstimateParameters {
        let estimation_feature_dimension = config
            .projected_feature_dimension
            .unwrap_or(config.feature_properties.len());

        let layer_configs = graphsage_helper::layer_configs(
            estimation_feature_dimension,
            &config.sample_sizes,
            config.random_seed,
            config.aggregator,
            config.activation_function,
            config.embedding_dimension,
        );

        GraphSageTrainMemoryEstimateParameters {
            layer_configs,
            is_multi_label: config.is_multi_label,
            feature_property_count: config.feature_properties.len(),
            estimation_feature_dimension,
            batch_size: config.batch_size,
            embedding_dimension: config.embedding_dimension,
        }
    }

    pub fn to_parameters(config: &GraphSageTrainConfig) -> GraphSageTrainParameters {
        GraphSageTrainParameters {
            concurrency: config.concurrency,
            batch_size: config.batch_size,
            max_iterations: config.max_iterations,
            search_depth: config.search_depth,
            epochs: config.epochs,
            learning_rate: config.learning_rate,
            tolerance: config.tolerance,
            negative_sample_weight: config.negative_sample_weight,
            penalty_l2: config.penalty_l2,
            embedding_dimension: config.embedding_dimension,
            sample_sizes: config.sample_sizes.clone(),
            feature_properties: config.feature_properties.clone(),
            maybe_batch_sampling_ratio: config.maybe_batch_sampling_ratio,
            random_seed: config.random_seed,
            aggregator: config.aggregator,
            activation_function: config.activation_function,
            is_multi_label: config.is_multi_label,
            projected_feature_dimension: config.projected_feature_dimension,
        }
    }
}
