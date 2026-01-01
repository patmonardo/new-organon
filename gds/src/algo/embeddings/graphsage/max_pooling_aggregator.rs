//! Java: `MaxPoolingAggregator`.

use crate::ml::core::functions::{
    ElementWiseMax, MatrixMultiplyWithTransposedSecondOperand, MatrixSum, MatrixVectorSum, Slice,
    Weights,
};
use crate::ml::core::subgraph::{BatchNeighbors, SubGraph};
use crate::ml::core::variable::VariableRef;
use std::sync::Arc;

use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::aggregator::Aggregator;
use super::types::{ActivationFunctionType, AggregatorType};

pub struct MaxPoolingAggregator {
    pool_weights: Arc<Weights>,
    self_weights: Arc<Weights>,
    neighbors_weights: Arc<Weights>,
    bias: Arc<Weights>,
    activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
}

impl MaxPoolingAggregator {
    pub fn new(
        pool_weights: Arc<Weights>,
        self_weights: Arc<Weights>,
        neighbors_weights: Arc<Weights>,
        bias: Arc<Weights>,
        activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
    ) -> Self {
        Self {
            pool_weights,
            self_weights,
            neighbors_weights,
            bias,
            activation_function_wrapper,
        }
    }
}

impl Aggregator for MaxPoolingAggregator {
    fn aggregate(
        &self,
        previous_layer_representations: VariableRef,
        sub_graph: &SubGraph,
    ) -> VariableRef {
        let weighted_prev = Arc::new(MatrixMultiplyWithTransposedSecondOperand::new_ref(
            previous_layer_representations.clone(),
            Arc::clone(&self.pool_weights) as VariableRef,
        )) as VariableRef;

        let biased = Arc::new(MatrixVectorSum::new_ref(
            weighted_prev,
            Arc::clone(&self.bias) as VariableRef,
        )) as VariableRef;

        let neighborhood_activations = self
            .activation_function_wrapper
            .activation_function()
            .apply(biased);

        let elementwise_max = Arc::new(ElementWiseMax::new_ref(
            neighborhood_activations,
            Box::new(sub_graph.clone()),
        )) as VariableRef;

        let self_prev = Arc::new(Slice::new_ref(
            previous_layer_representations,
            sub_graph.batch_ids().to_vec(),
        )) as VariableRef;

        let self_part = Arc::new(MatrixMultiplyWithTransposedSecondOperand::new_ref(
            self_prev,
            Arc::clone(&self.self_weights) as VariableRef,
        )) as VariableRef;

        let neighbors_part = Arc::new(MatrixMultiplyWithTransposedSecondOperand::new_ref(
            elementwise_max,
            Arc::clone(&self.neighbors_weights) as VariableRef,
        )) as VariableRef;

        let sum = Arc::new(MatrixSum::new_ref(vec![self_part, neighbors_part])) as VariableRef;

        self.activation_function_wrapper
            .activation_function()
            .apply(sum)
    }

    fn weights(&self) -> Vec<Arc<Weights>> {
        vec![
            Arc::clone(&self.pool_weights),
            Arc::clone(&self.self_weights),
            Arc::clone(&self.neighbors_weights),
            Arc::clone(&self.bias),
        ]
    }

    fn weights_without_bias(&self) -> Vec<Arc<Weights>> {
        vec![
            Arc::clone(&self.pool_weights),
            Arc::clone(&self.self_weights),
            Arc::clone(&self.neighbors_weights),
        ]
    }

    fn typ(&self) -> AggregatorType {
        AggregatorType::Pool
    }

    fn activation_function_type(&self) -> ActivationFunctionType {
        self.activation_function_wrapper.activation_function_type()
    }
}
