//! Concrete Parameter trait
//!
//! Translated from ConcreteParameter.java
//! Base trait for concrete (non-range) parameter values.

use std::any::Any;

/// Trait for concrete parameter values
///
/// Java: `interface ConcreteParameter<T> { T value(); }`
pub trait ConcreteParameter: Any + Send + Sync {
    /// Get self as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get the value as a boxed Any for type erasure
    fn value_any(&self) -> Box<dyn Any>;
}

