//! Variable trait for ML computations in GDS.
//!
//! Translated from Java GDS ml-core Variable.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! ## Type Erasure Design
//!
//! This implementation uses full type erasure (Option C) to match Java's runtime behavior.
//! The Variable trait has NO generic parameter, and all Tensor returns are boxed.
//! This makes the trait fully object-safe, enabling heterogeneous variable trees.
//!
//! Java's `Variable<T extends Tensor<T>>` uses wildcards (`Variable<?>`) which erase
//! to just `Variable` at runtime. Our Rust equivalent: `Box<dyn Variable>` works
//! identically, with `Box<dyn Tensor>` for tensor values.

use crate::ml::core::ComputationContext;
use crate::ml::core::Tensor;
use std::any::Any;
use std::fmt;
use std::sync::Arc;

/// Trait for variables in ML computations.
///
/// This corresponds to the Variable interface in Java GDS.
/// All tensor values are type-erased via `Box<dyn Tensor>`.
/// Extends Any for downcasting and pointer-based keying.
pub trait Variable: fmt::Display + Any + Send + Sync {
    /// Apply this variable to get its tensor value.
    /// Java: `T apply(ComputationContext ctx)`
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Compute gradient with respect to a parent variable.
    /// Java: `Tensor<?> gradient(Variable<?> parent, ComputationContext ctx)`
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor>;

    /// Whether this variable requires gradient computation.
    /// Java: `boolean requireGradient()`
    fn require_gradient(&self) -> bool;

    /// Get parent variables.
    /// Java: `Iterable<? extends Variable<?>> parents()`
    fn parents(&self) -> &[VariableRef];

    /// Get dimensions of this variable's output tensor.
    /// Java: `int[] dimensions()`
    fn dimensions(&self) -> &[usize];

    /// Get a specific dimension by index.
    /// Java: `int dimension(int i)`
    fn dimension(&self, i: usize) -> usize {
        self.dimensions()[i]
    }
}

/// Shared reference to a variable node.
///
/// This enables **DAG-shaped computation graphs** (shared parents) like Java object graphs.
pub type VariableRef = Arc<dyn Variable>;

/// Identity of a `Variable` instance, based on its allocation address.
///
/// This mirrors Java-style reference identity (object identity), not structural identity.
///
/// Note: this is only meaningful within a single process execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableId(pub usize);

/// Return the process-local identity of a variable instance.
///
/// We intentionally key by the *data pointer* behind the trait object (thin pointer),
/// so identity remains stable even if metadata (vtable) details differ.
pub fn variable_id(variable: &dyn Variable) -> VariableId {
    VariableId(variable as *const dyn Variable as *const () as usize)
}

/// Helper function to traverse variable tree and collect all variables.
pub fn collect_variables(variable: &VariableRef) -> Vec<VariableRef> {
    let mut result = Vec::new();
    collect_variables_recursive(variable, &mut result);
    result
}

fn collect_variables_recursive(variable: &VariableRef, result: &mut Vec<VariableRef>) {
    for parent in variable.parents() {
        collect_variables_recursive(parent, result);
    }
    result.push(variable.clone());
}

/// Helper function to check if any parent requires gradient.
pub fn any_parent_requires_gradient(parents: &[VariableRef]) -> bool {
    parents.iter().any(|parent| parent.require_gradient())
}
