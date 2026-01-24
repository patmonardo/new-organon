use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// Integer parameter value
///
/// Java: `interface IntegerParameter extends ConcreteParameter<Integer>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntegerParameter(pub i32);

impl IntegerParameter {
    /// Create a new IntegerParameter
    ///
    /// Java: `static IntegerParameter of(int value)`
    pub fn of(value: i32) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    /// Java: `Integer value()` (from ConcreteParameter)
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl ConcreteParameter for IntegerParameter {
    fn value(&self) -> ConcreteParameterValue {
        ConcreteParameterValue::Integer(self.0)
    }
}
