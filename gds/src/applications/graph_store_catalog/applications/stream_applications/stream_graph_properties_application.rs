use crate::applications::graph_store_catalog::results::GraphStreamGraphPropertiesResult;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::ValueType;

use serde_json::{json, Value};

/// Streams graph-level properties.
///
/// Java parity: `streamGraphProperty` returns a stream of (graphProperty, value).
/// Rust pass-1: we materialize into a `Vec`.
pub struct StreamGraphPropertiesApplication;

impl StreamGraphPropertiesApplication {
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        graph_property: &str,
    ) -> Result<Vec<GraphStreamGraphPropertiesResult>, String> {
        if !graph_store.has_graph_property(graph_property) {
            return Err(format!("Graph property '{graph_property}' does not exist"));
        }

        let value_type = graph_store
            .graph_property_type(graph_property)
            .map_err(|e| e.to_string())?;
        let values = graph_store
            .graph_property_values(graph_property)
            .map_err(|e| e.to_string())?;

        let property_value = graph_property_values_to_json(value_type, values.as_ref());
        Ok(vec![GraphStreamGraphPropertiesResult::new(
            graph_property.to_string(),
            property_value,
        )])
    }
}

fn graph_property_values_to_json(value_type: ValueType, values: &dyn GraphPropertyValues) -> Value {
    match value_type {
        ValueType::Long => {
            let collected: Vec<i64> = values.long_values().collect();
            match collected.as_slice() {
                [] => Value::Null,
                [one] => json!(*one),
                many => json!(many),
            }
        }
        ValueType::Double => {
            let collected: Vec<f64> = values.double_values().collect();
            match collected.as_slice() {
                [] => Value::Null,
                [one] => json!(*one),
                many => json!(many),
            }
        }
        ValueType::LongArray => {
            let collected: Vec<Vec<i64>> = values.long_array_values().collect();
            match collected.as_slice() {
                [] => Value::Null,
                [one] => json!(one),
                many => json!(many),
            }
        }
        ValueType::DoubleArray => {
            let collected: Vec<Vec<f64>> = values.double_array_values().collect();
            match collected.as_slice() {
                [] => Value::Null,
                [one] => json!(one),
                many => json!(many),
            }
        }
        ValueType::FloatArray => {
            let collected: Vec<Vec<f32>> = values.float_array_values().collect();
            match collected.as_slice() {
                [] => Value::Null,
                [one] => json!(one),
                many => json!(many),
            }
        }
        _ => Value::Null,
    }
}

impl Default for StreamGraphPropertiesApplication {
    fn default() -> Self {
        Self
    }
}
