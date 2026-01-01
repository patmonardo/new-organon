//! Java: `ActivationFunction`.

use crate::ml::core::variable::VariableRef;

/// Java: `ActivationFunction extends Function<Variable<Matrix>, Variable<Matrix>>`.
pub trait ActivationFunction: Send + Sync {
    fn apply(&self, input: VariableRef) -> VariableRef;
}
