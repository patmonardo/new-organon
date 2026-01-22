//! Java: `GraphSageModelTrainer`.

use crate::collections::HugeObjectArray;
use crate::concurrency::virtual_threads::RunWithConcurrency;
use crate::concurrency::TerminationFlag;
use crate::core::model::ModelCatalogCustomInfo;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::functions::{ConstantScale, ElementSum, L2NormSquared, Weights};
use crate::ml::core::optimizer::AdamOptimizer;
use crate::ml::core::optimizer::Updater;
use crate::ml::core::relationship_weights::{
    ClosureRelationshipWeights, RelationshipWeights, UNWEIGHTED,
};
use crate::ml::core::tensor::{Scalar, Tensor};
use crate::ml::core::variable::{Variable, VariableRef};
use crate::types::graph::Graph;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

use super::batch_sampler::BatchSampler;
use super::feature_function::FeatureFunction;
use super::graphsage_helper;
use super::graphsage_loss::GraphSageLoss;
use super::layer_factory::LayerFactory;
use super::model_data::ModelData;
use super::types::GraphSageTrainParameters;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphSageTrainMetrics {
    pub iteration_loss_per_epoch: Vec<Vec<f64>>,
    pub did_converge: bool,
}

impl ModelCatalogCustomInfo for GraphSageTrainMetrics {
    fn to_map(&self) -> serde_json::Value {
        serde_json::json!({
            "didConverge": self.did_converge,
            "iterationLossPerEpoch": self.iteration_loss_per_epoch,
        })
    }
}

#[derive(Clone)]
pub struct ModelTrainResult {
    pub metrics: GraphSageTrainMetrics,
    pub model_data: ModelData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::embeddings::graphsage::layer_factory::generate_weights;
    use crate::algo::embeddings::graphsage::multi_label_feature_function::MultiLabelFeatureFunction;
    use crate::algo::embeddings::graphsage::train_config_transformer::TrainConfigTransformer;
    use crate::algo::embeddings::graphsage::types::{
        ActivationFunctionType, AggregatorType, GraphSageTrainConfig,
    };
    use crate::concurrency::Concurrency;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use crate::types::schema::NodeLabel;
    use std::collections::HashMap;

    #[test]
    fn graphsage_train_smoke_multi_label() {
        let config = RandomGraphConfig {
            graph_name: "graphsage-train-mlabel".into(),
            database_name: "in-memory".into(),
            node_count: 60,
            node_labels: vec!["A".into(), "B".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.2)],
            directed: true,
            inverse_indexed: false,
            seed: Some(7),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let feature_properties = vec!["random_score".to_string()];
        let features = Arc::new(graphsage_helper::initialize_multi_label_features(
            graph.as_ref(),
            &feature_properties,
        ));

        let projected_dim = 4;
        let feature_dim_with_bias = 2; // random_score + bias

        let mut weights_by_label = HashMap::new();
        let w_a = Arc::new(generate_weights(
            projected_dim,
            feature_dim_with_bias,
            0.5,
            3,
        ));
        let w_b = Arc::new(generate_weights(
            projected_dim,
            feature_dim_with_bias,
            0.5,
            5,
        ));
        weights_by_label.insert(NodeLabel::of("A"), Arc::clone(&w_a));
        weights_by_label.insert(NodeLabel::of("B"), Arc::clone(&w_b));

        let feature_fn = Arc::new(MultiLabelFeatureFunction::new(
            weights_by_label,
            projected_dim,
        ));

        // Minimal training config.
        let train_cfg = GraphSageTrainConfig {
            model_user: "user".to_string(),
            model_name: "model".to_string(),
            concurrency: Concurrency::of(1),
            batch_size: 10,
            max_iterations: 2,
            search_depth: 2,
            epochs: 1,
            learning_rate: 0.01,
            tolerance: 0.0,
            negative_sample_weight: 1,
            penalty_l2: 0.001,
            embedding_dimension: 8,
            sample_sizes: vec![5],
            feature_properties,
            maybe_batch_sampling_ratio: None,
            random_seed: Some(42),
            aggregator: AggregatorType::Mean,
            activation_function: ActivationFunctionType::Relu,
            is_multi_label: true,
            projected_feature_dimension: Some(projected_dim),
        };
        let params = TrainConfigTransformer::to_parameters(&train_cfg);

        // label projection weights must be updated by optimizer -> pass them explicitly.
        let label_projection_weights: Vec<Arc<Weights>> = vec![w_a, w_b];

        let trainer = GraphSageModelTrainer::new(
            params,
            feature_fn,
            label_projection_weights,
            TerminationFlag::default(),
        );

        let res = trainer.train(graph, features, projected_dim);
        assert!(!res.metrics.iteration_loss_per_epoch.is_empty());
        assert!(!res.model_data.layers().is_empty());
    }
}

pub struct GraphSageModelTrainer {
    parameters: GraphSageTrainParameters,
    feature_function: Arc<dyn FeatureFunction>,
    label_projection_weights: Vec<Arc<Weights>>,
    termination_flag: TerminationFlag,
}

impl GraphSageModelTrainer {
    pub fn new(
        parameters: GraphSageTrainParameters,
        feature_function: Arc<dyn FeatureFunction>,
        label_projection_weights: Vec<Arc<Weights>>,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            parameters,
            feature_function,
            label_projection_weights,
            termination_flag,
        }
    }

    /// Java: `train(Graph graph, HugeObjectArray<double[]> features)`
    pub fn train(
        &self,
        graph: Arc<dyn Graph>,
        features: Arc<HugeObjectArray<Vec<f64>>>,
        feature_dimension: usize,
    ) -> ModelTrainResult {
        // Build layers from config-derived LayerConfig
        let layer_configs = self.parameters.layer_configs(feature_dimension);
        let layers: Vec<Arc<dyn super::layer::Layer>> = layer_configs
            .iter()
            .map(|cfg| LayerFactory::create_layer(cfg))
            .collect();

        // Collect all weights in the model
        let mut weights: Vec<Arc<Weights>> = Vec::new();
        weights.extend(self.label_projection_weights.iter().cloned());
        for layer in &layers {
            weights.extend(layer.aggregator().weights());
        }

        let batch_sampler = BatchSampler::new(Arc::clone(&graph), self.termination_flag.clone());
        let extended_batches = batch_sampler.extended_batches(
            self.parameters.batch_size,
            self.parameters.search_depth,
            self.parameters.random_seed.unwrap_or(42),
        );

        let mut rng = ChaCha8Rng::seed_from_u64(self.parameters.random_seed.unwrap_or(42));

        let mut iteration_losses_per_epoch: Vec<Vec<f64>> = Vec::new();
        let mut prev_epoch_loss = f64::NAN;
        let mut converged = false;

        let relationship_weights: Arc<dyn RelationshipWeights> =
            if graph.has_relationship_property() {
                let g = Arc::clone(&graph);
                Arc::new(ClosureRelationshipWeights::new(move |s, t, default| {
                    g.relationship_property(s as i64, t as i64, default)
                }))
            } else {
                Arc::new(UNWEIGHTED)
            };

        for _epoch in 1..=self.parameters.epochs {
            self.termination_flag.assert_running();

            let batches_per_iter = self.parameters.batches_per_iteration(graph.node_count());

            let mut updater = AdamOptimizer::new(weights.clone(), self.parameters.learning_rate);

            let mut prev_loss = prev_epoch_loss;
            let mut epoch_losses = Vec::new();

            for _iteration in 0..self.parameters.max_iterations {
                self.termination_flag.assert_running();

                // Sample batch tasks
                let mut sampled_batches: Vec<Vec<u64>> = Vec::with_capacity(batches_per_iter);
                for _ in 0..batches_per_iter {
                    let idx = rng.gen_range(0..extended_batches.len().max(1));
                    if let Some(batch) = extended_batches.get(idx) {
                        sampled_batches.push(batch.clone());
                    }
                }

                let results = Arc::new(parking_lot::Mutex::new(Vec::<BatchResult>::new()));

                let tasks: Vec<Box<dyn FnOnce() + Send>> = sampled_batches
                    .into_iter()
                    .map(|batch| {
                        let graph = <dyn Graph as Graph>::concurrent_copy(&*graph);
                        let features = Arc::clone(&features);
                        let layers = layers.clone();
                        let feature_function = Arc::clone(&self.feature_function);
                        let weights = weights.clone();
                        let results = Arc::clone(&results);
                        let relationship_weights = Arc::clone(&relationship_weights);
                        let penalty_l2 = self.parameters.penalty_l2;
                        let negative_sample_weight = self.parameters.negative_sample_weight;
                        let node_count = graph.node_count().max(1);
                        let label_projection_weight_count = self.label_projection_weights.len();
                        Box::new(move || {
                            let res = run_batch(
                                graph,
                                features,
                                layers,
                                feature_function,
                                weights,
                                relationship_weights,
                                batch,
                                negative_sample_weight,
                                penalty_l2,
                                node_count,
                                label_projection_weight_count,
                            );
                            results.lock().push(res);
                        }) as Box<dyn FnOnce() + Send>
                    })
                    .collect();

                RunWithConcurrency::builder()
                    .concurrency(self.parameters.concurrency)
                    .termination_flag(self.termination_flag.clone())
                    .tasks(tasks)
                    .run()
                    .expect("run batch tasks");

                let results = match Arc::try_unwrap(results) {
                    Ok(mutex) => mutex.into_inner(),
                    Err(_) => panic!("results still referenced"),
                };
                let avg_loss =
                    results.iter().map(|r| r.loss).sum::<f64>() / results.len().max(1) as f64;
                epoch_losses.push(avg_loss);

                if prev_loss.is_finite() && (prev_loss - avg_loss).abs() < self.parameters.tolerance
                {
                    converged = true;
                    break;
                }
                prev_loss = avg_loss;

                // Average gradients across batch tasks and update weights
                let mean_gradients = average_gradients(&results);
                updater.update(&mean_gradients);
            }

            prev_epoch_loss = *epoch_losses.last().unwrap_or(&prev_epoch_loss);
            iteration_losses_per_epoch.push(epoch_losses);

            if converged {
                break;
            }
        }

        ModelTrainResult {
            metrics: GraphSageTrainMetrics {
                iteration_loss_per_epoch: iteration_losses_per_epoch,
                did_converge: converged,
            },
            model_data: ModelData::of(layers, Arc::clone(&self.feature_function)),
        }
    }
}

struct BatchResult {
    loss: f64,
    gradients: Vec<Box<dyn Tensor>>,
}

#[allow(clippy::too_many_arguments)]
fn run_batch(
    graph: Arc<dyn Graph>,
    features: Arc<HugeObjectArray<Vec<f64>>>,
    layers: Vec<Arc<dyn super::layer::Layer>>,
    feature_function: Arc<dyn FeatureFunction>,
    weights: Vec<Arc<Weights>>,
    relationship_weights: Arc<dyn RelationshipWeights>,
    batch: Vec<u64>,
    negative_sampling_factor: usize,
    penalty_l2: f64,
    graph_node_count: usize,
    label_projection_weight_count: usize,
) -> BatchResult {
    let sub_graphs = graphsage_helper::sub_graphs_per_layer(
        Arc::clone(&graph),
        &batch,
        &layers,
        0, // caller already seeds sampling; GraphSageHelper further mixes
    );

    let deepest = sub_graphs
        .last()
        .map(|s| s.original_node_ids())
        .unwrap_or(&batch);

    let batched_features =
        feature_function.apply(Arc::clone(&graph), deepest, Arc::clone(&features));
    let embedding_var =
        graphsage_helper::embeddings_computation_graph(&sub_graphs, &layers, batched_features);

    let loss_without_penalty: VariableRef = Arc::new(GraphSageLoss::new(
        relationship_weights,
        embedding_var,
        batch.clone(),
        negative_sampling_factor,
    ));

    let loss: VariableRef = if penalty_l2 > 0.0 {
        // Java regularizes weights without bias, plus label-projection weights (if any).
        let mut l2_terms: Vec<VariableRef> = Vec::new();

        for w in weights.iter().take(label_projection_weight_count) {
            l2_terms.push(Arc::new(L2NormSquared::new_ref(
                Arc::clone(w) as VariableRef
            )));
        }
        for layer in &layers {
            for w in layer.aggregator().weights_without_bias() {
                l2_terms.push(Arc::new(L2NormSquared::new_ref(w as VariableRef)));
            }
        }

        let penalty_sum: VariableRef = Arc::new(ElementSum::new_ref(l2_terms));

        // Java scales by originalBatchSize / nodeCount.
        let original_batch_size = batch.len() / 3;
        let scale = penalty_l2 * (original_batch_size as f64) / (graph_node_count as f64);
        let scaled_penalty: VariableRef = Arc::new(ConstantScale::new_ref(penalty_sum, scale));

        Arc::new(ElementSum::new_ref(vec![
            loss_without_penalty,
            scaled_penalty,
        ]))
    } else {
        loss_without_penalty
    };

    let ctx = ComputationContext::new();
    let loss_tensor = ctx.forward(loss.as_ref());
    let loss_value = loss_tensor
        .as_any()
        .downcast_ref::<Scalar>()
        .expect("loss is scalar")
        .value();

    ctx.backward(loss.as_ref());
    let gradients: Vec<Box<dyn Tensor>> = weights
        .iter()
        .map(|w| ctx.gradient((w.as_ref()) as &dyn Variable).unwrap())
        .collect();

    BatchResult {
        loss: loss_value,
        gradients,
    }
}

fn average_gradients(results: &[BatchResult]) -> Vec<Box<dyn Tensor>> {
    if results.is_empty() {
        return Vec::new();
    }
    let n = results.len() as f64;
    let mut acc: Vec<Box<dyn Tensor>> =
        results[0].gradients.iter().map(|g| g.clone_box()).collect();
    for r in &results[1..] {
        for (i, g) in r.gradients.iter().enumerate() {
            acc[i].add_inplace(g.as_ref());
        }
    }
    acc.into_iter()
        .map(|t| t.scalar_multiply(1.0 / n))
        .collect()
}
