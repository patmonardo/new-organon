//! Java: `Aggregator`.

use crate::ml::core::functions::Weights;
use crate::ml::core::subgraph::SubGraph;
use crate::ml::core::variable::VariableRef;
use std::sync::Arc;

use super::types::{ActivationFunctionType, AggregatorType};

pub trait Aggregator: Send + Sync {
    fn aggregate(
        &self,
        previous_layer_representations: VariableRef,
        sub_graph: &SubGraph,
    ) -> VariableRef;

    fn weights(&self) -> Vec<Arc<Weights>>;
    fn weights_without_bias(&self) -> Vec<Arc<Weights>>;
    fn typ(&self) -> AggregatorType;
    fn activation_function_type(&self) -> ActivationFunctionType;
}
