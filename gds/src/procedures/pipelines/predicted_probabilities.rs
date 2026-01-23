use std::sync::Arc;

use crate::applications::algorithms::machinery::NodeProperty;
use crate::collections::backends::vec::VecDoubleArray;
use crate::collections::backends::vec::VecLong;
use crate::procedures::pipelines::NodeClassificationPipelineResult;
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultLongNodePropertyValues,
};
use crate::types::properties::node::NodePropertyValues;

pub fn as_properties(
    result: Option<&NodeClassificationPipelineResult>,
    property_name: &str,
    predicted_probability_property: Option<&str>,
) -> Vec<NodeProperty> {
    let Some(result) = result else {
        return vec![];
    };

    let predicted_classes = result.predicted_classes();
    let node_count = predicted_classes.size();

    let mut class_values_vec = Vec::with_capacity(node_count);
    for idx in 0..node_count {
        class_values_vec.push(predicted_classes.get(idx));
    }

    let class_values: Arc<dyn NodePropertyValues> =
        Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
            VecLong::from(class_values_vec),
            node_count,
        ));

    let mut node_properties = vec![NodeProperty::new(property_name, class_values)];

    if let Some(probabilities) = result.predicted_probabilities() {
        let property_key = predicted_probability_property
            .expect("predictedProbabilityProperty must be set when probabilities are present");

        let mut rows: Vec<Option<Vec<f64>>> = Vec::with_capacity(node_count);
        for idx in 0..node_count {
            rows.push(Some(probabilities.get(idx).clone()));
        }

        let probability_values: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
                VecDoubleArray::from(rows),
                node_count,
            ),
        );

        node_properties.push(NodeProperty::new(property_key, probability_values));
    }

    node_properties
}
