//! IndexInverse computation runtime.
//!
//! This algorithm is GraphStore-driven: the heavy lifting is done by the
//! storage runtime via `with_inverse_indices`. The computation runtime is a
//! placeholder to keep the controller pattern consistent.

#[derive(Debug, Default, Clone)]
pub struct IndexInverseComputationRuntime;

impl IndexInverseComputationRuntime {
    pub fn new() -> Self {
        Self
    }
}
