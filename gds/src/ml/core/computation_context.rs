//! Computation context for ML operations in GDS.
//!
//! This is the **heart of the ML platform** - it manages forward and backward propagation
//! over computation graphs consisting of Variables.
//!
//! ## Key Features
//! - **Forward Pass**: Computes variable values with caching
//! - **Backward Pass**: Computes gradients via backpropagation using topological sorting
//! - **Identity**: Uses pointer-based keys to avoid trait object identity pitfalls
//! - **Interior Mutability**: Allows caching during forward/backward passes
//!
//! ## Thread Safety
//! ⚠️ **This implementation is NOT thread-safe!** Use with caution in concurrent contexts.
//!
//! ## Usage
//! ```rust
//! let ctx = ComputationContext::new();
//! let result = ctx.forward(&my_variable);
//! ctx.backward(&loss_variable);
//! let gradient = ctx.gradient(&weight_variable);
//! ```

use crate::ml::core::{dimensions, variable_id, Tensor, Variable, VariableRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

/// The computation context manages forward and backward propagation over computation graphs.
///
/// ## Architecture
/// - **Data Cache**: Stores computed variable values (`HashMap<String, Box<dyn Tensor>>`)
/// - **Gradient Cache**: Stores computed gradients (`HashMap<String, Box<dyn Tensor>>`)
/// - **Pointer-Based Keys**: Uses trait object address + shape for identity
/// - **Interior Mutability**: Uses `RefCell` to allow caching with immutable references
///
/// ## Thread Safety
/// ⚠️ **NOT thread-safe** - uses `RefCell` for interior mutability
pub struct ComputationContext {
    data: RefCell<HashMap<String, Box<dyn Tensor>>>,
    gradients: RefCell<HashMap<String, Box<dyn Tensor>>>,
}

impl ComputationContext {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            gradients: RefCell::new(HashMap::new()),
        }
    }

    /// Create a unique key for a variable.
    ///
    /// This uses the trait object address plus shape information.
    ///
    /// Note: this is identity-by-instance (like Java object identity), not structural identity.
    fn variable_key(&self, variable: &dyn Variable) -> String {
        let dims = variable.dimensions();
        let dim_str = format!("{:?}", dims);
        // Use process-local identity (allocation address) for unique identification
        let addr = variable_id(variable).0;
        format!("{}:{}", addr, dim_str)
    }

    /// Forward pass - compute variable value with caching.
    ///
    /// ## Process
    /// 1. Check cache for existing result
    /// 2. Recursively compute all parent variables
    /// 3. Compute this variable using `variable.apply()`
    /// 4. Cache and return the result
    ///
    /// ## Thread Safety
    /// Uses interior mutability (`RefCell`) to allow caching with immutable references.
    pub fn forward(&self, variable: &dyn Variable) -> Box<dyn Tensor> {
        let var_key = self.variable_key(variable);

        // Check cache first
        if let Some(cached) = self.data.borrow().get(&var_key) {
            return cached.clone_box();
        }

        // Compute parents first (recursive)
        for parent in variable.parents() {
            self.forward(parent.as_ref());
        }

        // Compute this variable
        let result = variable.apply(self);
        self.data.borrow_mut().insert(var_key, result.clone_box());
        result
    }

    /// Get cached data for a variable.
    pub fn data(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        let var_key = self.variable_key(variable);
        self.data.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Get cached gradient for a variable.
    pub fn gradient(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        let var_key = self.variable_key(variable);
        self.gradients.borrow().get(&var_key).map(|t| t.clone_box())
    }

    /// Get the number of computed variables (for debugging).
    pub fn computed_variables_count(&self) -> usize {
        self.data.borrow().len()
    }

    /// Backward pass - compute gradients via backpropagation.
    ///
    /// ## Algorithm
    /// 1. **Validate**: Ensure root is scalar and requires gradients
    /// 2. **Initialize**: Set root gradient to 1.0
    /// 3. **Topological Sort**: Collect variables in dependency order
    /// 4. **Reverse Topological Pass**: For each variable with a gradient, propagate to its parents
    /// 5. **Gradient Accumulation**: Accumulate gradients from multiple children
    ///
    /// ## Key Detail
    /// Uses pointer-based keys for identity, mirroring Java object identity.
    ///
    /// ## Thread Safety
    /// ⚠️ **NOT thread-safe** - modifies internal state
    pub fn backward(&self, function: &dyn Variable) {
        assert!(
            dimensions::is_scalar(function.dimensions()),
            "Root variable must be scalar."
        );
        assert!(
            function.require_gradient(),
            "Root variable must have requireGradient==true"
        );

        self.gradients.borrow_mut().clear();

        // Initialize root variable gradient to 1
        let root_data = self.data(function).expect("Root variable not computed");
        let root_gradient = root_data.map(|_| 1.0);
        self.update_gradient(function, root_gradient);

        // Collect all variables in topological order (children before parents)
        let mut execution_order = Vec::new();
        let mut visited = HashSet::new();
        self.collect_variables_topological(
            VariableNode::Borrowed(function),
            &mut visited,
            &mut execution_order,
        );

        // Single reverse-topological pass:
        // - Each edge (variable -> parent) is visited once.
        // - If a parent has multiple children, `update_gradient` will accumulate.
        for node in execution_order.iter().rev() {
            let variable = node.as_dyn();

            // Skip if gradient not available yet (should not happen with correct ordering).
            if self.gradient(variable).is_none() {
                continue;
            }

            for parent in variable.parents() {
                if !parent.require_gradient() {
                    continue;
                }

                let parent_gradient = variable.gradient(parent.as_ref(), self);
                self.update_gradient(parent.as_ref(), parent_gradient);
            }
        }
    }

    /// Collect all variables in topological order (children before parents).
    /// This ensures that when we process in reverse order, parents are processed before children.
    #[allow(clippy::only_used_in_recursion)]
    fn collect_variables_topological<'a>(
        &self,
        variable: VariableNode<'a>,
        visited: &mut HashSet<usize>,
        result: &mut Vec<VariableNode<'a>>,
    ) {
        let key = variable.ptr_usize();

        // Skip if already collected
        if !visited.insert(key) {
            return;
        }

        // First, collect all parents (recursively)
        for parent in variable.as_dyn().parents() {
            if parent.require_gradient() {
                self.collect_variables_topological(
                    VariableNode::Owned(parent.clone()),
                    visited,
                    result,
                );
            }
        }

        // Then add this variable
        result.push(variable);
    }

    /// Update gradient for a variable (accumulate if already exists)
    fn update_gradient(&self, variable: &dyn Variable, gradient: Box<dyn Tensor>) {
        let var_key = self.variable_key(variable);

        let mut gradients = self.gradients.borrow_mut();
        if let Some(existing_gradient) = gradients.get_mut(&var_key) {
            // Accumulate gradients
            existing_gradient.add_inplace(gradient.as_ref());
        } else {
            // Store new gradient
            gradients.insert(var_key, gradient);
        }
    }
}

enum VariableNode<'a> {
    Borrowed(&'a dyn Variable),
    Owned(VariableRef),
}

impl VariableNode<'_> {
    fn as_dyn(&self) -> &dyn Variable {
        match self {
            VariableNode::Borrowed(v) => *v,
            VariableNode::Owned(v) => v.as_ref(),
        }
    }

    fn ptr_usize(&self) -> usize {
        variable_id(self.as_dyn()).0
    }
}

impl Default for ComputationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::{Scalar, Tensor};
    use std::fmt;

    #[test]
    fn test_context_creation() {
        let ctx = ComputationContext::new();
        assert!(ctx.data.borrow().is_empty());
        assert!(ctx.gradients.borrow().is_empty());
    }

    struct TestLeafVariable {
        dims: Vec<usize>,
        parents: Vec<VariableRef>,
    }

    impl TestLeafVariable {
        fn new() -> Self {
            Self {
                dims: dimensions::scalar(),
                parents: vec![],
            }
        }
    }

    impl fmt::Display for TestLeafVariable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestLeafVariable")
        }
    }

    impl Variable for TestLeafVariable {
        fn apply(&self, _ctx: &ComputationContext) -> Box<dyn Tensor> {
            Box::new(Scalar::new(0.0))
        }

        fn gradient(&self, _parent: &dyn Variable, _ctx: &ComputationContext) -> Box<dyn Tensor> {
            panic!("TestLeafVariable has no parents")
        }

        fn require_gradient(&self) -> bool {
            true
        }

        fn parents(&self) -> &[VariableRef] {
            &self.parents
        }

        fn dimensions(&self) -> &[usize] {
            &self.dims
        }
    }

    #[test]
    fn test_update_gradient_accumulates() {
        let ctx = ComputationContext::new();
        let leaf = TestLeafVariable::new();

        ctx.update_gradient(&leaf, Box::new(Scalar::new(1.25)));
        ctx.update_gradient(&leaf, Box::new(Scalar::new(2.75)));

        let grad = ctx
            .gradient(&leaf)
            .expect("Expected gradient to be present")
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar gradient")
            .value();

        assert!((grad - 4.0).abs() < 1e-12);
    }
}
