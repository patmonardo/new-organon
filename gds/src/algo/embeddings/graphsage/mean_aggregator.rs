//! Java: `MeanAggregator`.

use crate::ml::core::functions::{MatrixMultiplyWithTransposedSecondOperand, MultiMean, Weights};
use crate::ml::core::subgraph::SubGraph;
use crate::ml::core::variable::VariableRef;
use std::sync::Arc;

use super::activation_function_wrapper::ActivationFunctionWrapper;
use super::aggregator::Aggregator;
use super::types::{ActivationFunctionType, AggregatorType};

pub struct MeanAggregator {
    weights: Arc<Weights>,
    activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
}

impl MeanAggregator {
    pub fn new(
        weights: Arc<Weights>,
        activation_function_wrapper: Arc<dyn ActivationFunctionWrapper>,
    ) -> Self {
        Self {
            weights,
            activation_function_wrapper,
        }
    }
}

impl Aggregator for MeanAggregator {
    fn aggregate(
        &self,
        previous_layer_representations: VariableRef,
        sub_graph: &SubGraph,
    ) -> VariableRef {
        let means = Arc::new(MultiMean::new_ref(
            previous_layer_representations,
            Box::new(sub_graph.clone()),
        )) as VariableRef;

        let product = Arc::new(MatrixMultiplyWithTransposedSecondOperand::new_ref(
            means,
            Arc::clone(&self.weights) as VariableRef,
        )) as VariableRef;

        self.activation_function_wrapper
            .activation_function()
            .apply(product)
    }

    fn weights(&self) -> Vec<Arc<Weights>> {
        vec![Arc::clone(&self.weights)]
    }

    fn weights_without_bias(&self) -> Vec<Arc<Weights>> {
        vec![Arc::clone(&self.weights)]
    }

    fn typ(&self) -> AggregatorType {
        AggregatorType::Mean
    }

    fn activation_function_type(&self) -> ActivationFunctionType {
        self.activation_function_wrapper.activation_function_type()
    }
}
