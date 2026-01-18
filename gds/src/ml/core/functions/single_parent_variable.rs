//! Single parent variable base implementation for ML functions in GDS.
//!
//! Translated from Java GDS ml-core functions/SingleParentVariable.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Uses type erasure pattern (Box<dyn Variable>) to match our architecture.

use crate::ml::core::AbstractVariable;
use crate::ml::core::ComputationContext;
use crate::ml::core::Tensor;
use crate::ml::core::{Variable, VariableRef};

/// Abstract base for variables with a single parent.
///
/// Provides common functionality for functions that have exactly one input variable.
/// Uses composition with AbstractVariable to match Java's inheritance pattern.
///
/// This corresponds to Java's `SingleParentVariable<P extends Tensor<P>, T extends Tensor<T>> extends AbstractVariable<T>`
pub struct SingleParentVariable {
    base: AbstractVariable,
    parent: VariableRef,
}

impl SingleParentVariable {
    /// Create a new single parent variable.
    /// Java: `public SingleParentVariable(Variable<P> parent, int[] dimensions)`
    pub fn new(parent: Box<dyn Variable>, dimensions: Vec<usize>) -> Self {
        let parent: VariableRef = parent.into();
        let require_gradient = parent.require_gradient();
        let base = AbstractVariable::with_gradient_requirement(
            vec![parent.clone()],
            dimensions,
            require_gradient,
        );
        Self { base, parent }
    }

    /// Get the parent variable.
    /// Java: `protected final Variable<P> parent`
    pub fn parent(&self) -> &dyn Variable {
        self.parent.as_ref()
    }

    /// Validate that the given variable is our parent.
    /// Java: `private void validateParent(Variable<?> variable)`
    pub fn validate_parent(&self, variable: &dyn Variable) {
        if !std::ptr::eq::<dyn Variable>(self.parent.as_ref(), variable) {
            panic!("Calling gradient with a `parent` that was not expected");
        }
    }

    /// Template method for gradient computation.
    /// Java: `protected abstract P gradientForParent(ComputationContext ctx);`
    ///
    /// Concrete implementations must override this method.
    pub fn gradient_for_parent(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!(
            "SingleParentVariable is abstract; implement gradient_for_parent() in a concrete type"
        )
    }
}

impl Variable for SingleParentVariable {
    /// Apply: Must be implemented by concrete subclasses.
    /// Java: `public abstract T apply(ComputationContext ctx);`
    fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
        panic!("SingleParentVariable is abstract; implement apply() in a concrete type")
    }

    /// Gradient: Template method that validates parent and calls gradient_for_parent.
    /// Java: `public final Tensor<?> gradient(Variable<?> variable, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor> {
        self.validate_parent(parent);
        self.gradient_for_parent(ctx)
    }

    // DELEGATION: Forward to AbstractVariable
    fn dimensions(&self) -> &[usize] {
        self.base.dimensions()
    }

    // DELEGATION: Forward to AbstractVariable
    fn require_gradient(&self) -> bool {
        self.base.require_gradient()
    }

    // DELEGATION: Forward to AbstractVariable
    fn parents(&self) -> &[VariableRef] {
        self.base.parents()
    }
}

impl std::fmt::Display for SingleParentVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SingleParentVariable")
    }
}
