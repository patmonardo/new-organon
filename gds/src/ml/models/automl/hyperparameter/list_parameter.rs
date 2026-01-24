use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// List parameter value (for integer or double lists)
///
/// Java: `interface ListParameter extends ConcreteParameter<List<Integer>>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListParameterValue {
    Int(Vec<i32>),
    Double(Vec<f64>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParameter(pub ListParameterValue);

impl ListParameter {
    /// Create a new ListParameter
    ///
    /// Java: `static ListParameter of(List value)`
    pub fn of(value: ListParameterValue) -> Self {
        Self(value)
    }

    pub fn of_ints(values: Vec<i32>) -> Self {
        Self(ListParameterValue::Int(values))
    }

    pub fn of_doubles(values: Vec<f64>) -> Self {
        Self(ListParameterValue::Double(values))
    }

    /// Get the value
    ///
    /// Java: `List<Integer> value()` (from ConcreteParameter)
    pub fn value(&self) -> &ListParameterValue {
        &self.0
    }
}

impl ConcreteParameter for ListParameter {
    fn value(&self) -> ConcreteParameterValue {
        match &self.0 {
            ListParameterValue::Int(values) => ConcreteParameterValue::IntList(values.clone()),
            ListParameterValue::Double(values) => {
                ConcreteParameterValue::DoubleList(values.clone())
            }
        }
    }
}
