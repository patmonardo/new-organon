//! String Parameter
//!
//! Translated from StringParameter.java

use super::concrete_parameter::ConcreteParameter;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// String parameter value
///
/// Java: `interface StringParameter extends ConcreteParameter<String>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, From, Serialize, Deserialize)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value_any(&self) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

