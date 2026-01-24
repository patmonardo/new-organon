use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// String parameter value
///
/// Java: `interface StringParameter extends ConcreteParameter<String>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringParameter(pub String);

impl StringParameter {
    /// Create a new StringParameter
    ///
    /// Java: `static StringParameter of(String value)`
    pub fn of(value: String) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    /// Java: `String value()` (from ConcreteParameter)
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ConcreteParameter for StringParameter {
    fn value(&self) -> ConcreteParameterValue {
        ConcreteParameterValue::String(self.0.clone())
    }
}
