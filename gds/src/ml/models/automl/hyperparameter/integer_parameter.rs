//! Integer Parameter
//!
//! Translated from IntegerParameter.java

use super::concrete_parameter::ConcreteParameter;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Integer parameter value
///
/// Java: `interface IntegerParameter extends ConcreteParameter<Integer>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, From, Serialize, Deserialize)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value_any(&self) -> Box<dyn Any> {
        Box::new(self.0)
    }
}

