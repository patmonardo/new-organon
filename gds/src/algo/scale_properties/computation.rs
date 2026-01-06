//! ScaleProperties computation runtime.
//!
//! The scaling work is orchestrated by the storage runtime; this runtime
//! remains a lightweight placeholder to maintain the controller pattern.

#[derive(Debug, Default, Clone)]
pub struct ScalePropertiesComputationRuntime;

impl ScalePropertiesComputationRuntime {
    pub fn new() -> Self {
        Self
    }
}
