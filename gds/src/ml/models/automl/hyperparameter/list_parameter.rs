//! List Parameter
//!
//! Translated from ListParameter.java

use super::concrete_parameter::ConcreteParameter;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// List parameter value (for integer lists like hiddenLayerSizes)
///
/// Java: `interface ListParameter extends ConcreteParameter<List<Integer>>`
#[derive(Debug, Clone, PartialEq, Eq, Hash, From, Serialize, Deserialize)]
pub struct ListParameter(pub Vec<i32>);

impl ListParameter {
    /// Create a new ListParameter
    ///
    /// Java: `static ListParameter of(List value)`
    pub fn of(value: Vec<i32>) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    /// Java: `List<Integer> value()` (from ConcreteParameter)
    pub fn value(&self) -> &[i32] {
        &self.0
    }
}

impl ConcreteParameter for ListParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn value_any(&self) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

