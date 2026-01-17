// Feature and label extraction for link prediction.

use super::FeaturesAndLabels;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::projection::eval::pipeline::link_pipeline::{LinkFeatureExtractor, LinkFeatureStep};
use crate::types::graph::Graph;

/// Label value for POSITIVE relationships (relationship exists).
pub const POSITIVE: i32 = 1;

/// Label value for NEGATIVE relationships (relationship does not exist).
pub const NEGATIVE: i32 = 0;

/// Extract features and labels for link prediction training.
///
/// # Arguments
/// * `graph` - The graph containing relationships to extract from
/// * `feature_steps` - List of link feature steps to apply
/// * `concurrency` - Number of parallel workers
/// * `termination_flag` - Allows early termination
///
/// # Returns
/// Result containing `FeaturesAndLabels` or error.
pub fn extract_features_and_labels(
    graph: &dyn Graph,
    feature_steps: Vec<Box<dyn LinkFeatureStep>>,
    concurrency: Concurrency,
    termination_flag: &TerminationFlag,
) -> Result<FeaturesAndLabels, String> {
    // TODO: integrate negative sampling. For now, label existing relationships as positive.
    let features =
        LinkFeatureExtractor::extract_features(graph, feature_steps, concurrency, termination_flag);
    let labels = vec![POSITIVE; features.len()];
    Ok(FeaturesAndLabels::new(features, labels))
}

/// Estimate memory requirements for features and labels.
pub fn estimate_memory(
    _fudged_link_feature_dim: (usize, usize),
    _relationship_set_size: usize,
    _set_description: String,
) -> MemoryEstimate {
    // TODO: implement when memory range estimation is available.
    MemoryEstimate {
        description: "LinkFeaturesAndLabelsExtractor memory estimation not yet implemented"
            .to_string(),
        min_bytes: 0,
        max_bytes: 0,
    }
}

/// Memory estimation result.
#[derive(Debug, Clone)]
pub struct MemoryEstimate {
    /// Description of what is being estimated.
    pub description: String,

    /// Minimum memory required (bytes).
    pub min_bytes: usize,

    /// Maximum memory required (bytes).
    pub max_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_constants() {
        assert_eq!(POSITIVE, 1);
        assert_eq!(NEGATIVE, 0);
    }

    #[test]
    fn test_features_and_labels_creation() {
        let labels = vec![POSITIVE, NEGATIVE, POSITIVE, NEGATIVE];
        let fal = FeaturesAndLabels::new(vec![vec![0.0]; 4], labels.clone());

        assert_eq!(fal.size(), 4);
        assert_eq!(fal.labels(), &labels);
    }

    #[test]
    fn test_memory_estimate_placeholder() {
        let estimate = estimate_memory((10, 20), 1000, "test_set".to_string());

        assert!(estimate.description.contains("not yet implemented"));
        assert_eq!(estimate.min_bytes, 0);
        assert_eq!(estimate.max_bytes, 0);
    }
}
