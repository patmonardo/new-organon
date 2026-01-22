//! FastRP storage runtime.
//!
//! This is the **Gross pole**: obtaining graph views and property-backed feature extractors.

use crate::ml::core::features::feature_extraction::{property_extractors, AnyFeatureExtractor};
use crate::types::graph::Graph;

/// FastRP storage runtime.
#[derive(Debug, Default, Clone)]
pub struct FastRPStorageRuntime;

impl FastRPStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Build feature extractors for the requested node properties.
    ///
    /// Returns an error string (caller maps to `AlgorithmError`).
    pub fn feature_extractors(
        &self,
        graph: &dyn Graph,
        feature_properties: &[String],
    ) -> Result<Vec<AnyFeatureExtractor>, String> {
        if feature_properties.is_empty() {
            return Ok(Vec::new());
        }

        let mut extractors = Vec::with_capacity(feature_properties.len());
        for key in feature_properties {
            // Avoid panics from the literal Java translation helper.
            if graph.node_properties(key).is_none() {
                return Err(format!(
                    "Missing node property `{key}`. Consider using a default value in the property projection."
                ));
            }
        }

        // Now safe to use the translated helper.
        extractors.extend(property_extractors(graph, feature_properties));

        Ok(extractors)
    }
}
