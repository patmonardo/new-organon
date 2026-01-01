//! HashGNN storage runtime.
//!
//! This is the **Gross pole**: obtaining graph views and validating the presence
//! of node properties used for feature extraction.

use crate::types::graph::Graph;

/// HashGNN storage runtime.
#[derive(Debug, Default, Clone)]
pub struct HashGNNStorageRuntime;

impl HashGNNStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Validate that all requested node properties exist.
    pub fn validate_feature_properties(
        &self,
        graph: &dyn Graph,
        feature_properties: &[String],
    ) -> Result<(), String> {
        for key in feature_properties {
            if graph.node_properties(key).is_none() {
                return Err(format!(
                    "Missing node property `{key}`. Consider using a default value in the property projection."
                ));
            }
        }
        Ok(())
    }
}
