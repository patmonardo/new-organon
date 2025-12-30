//! Double Parameter
//!
//! Translated from DoubleParameter.java

use super::concrete_parameter::ConcreteParameter;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Double parameter value
///
/// Java: `interface DoubleParameter extends ConcreteParameter<Double>`
#[derive(Debug, Clone, PartialEq, From, Serialize, Deserialize)]
pub struct DoubleParameter(pub f64);

impl DoubleParameter {
    /// Create a new DoubleParameter
    ///
    /// Java: `static DoubleParameter of(double value)`
    pub fn of(value: f64) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    /// Java: `Double value()` (from ConcreteParameter)
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl ConcreteParameter for DoubleParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value_any(&self) -> Box<dyn Any> {
        Box::new(self.0)
    }
}

