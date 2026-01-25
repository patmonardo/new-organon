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
}
