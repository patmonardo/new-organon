use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;

/// Base configuration trait for node property pipeline training.
///
/// This trait combines training configuration with graph-specific settings
/// for node property prediction pipelines (classification and regression).
pub trait NodePropertyPipelineBaseTrainConfig {
    /// Returns the name of the pipeline to train.
    fn pipeline(&self) -> &str;

    /// Returns the target node labels for training.
    ///
    /// Default is to project all node labels.
    fn target_node_labels(&self) -> Vec<String> {
        vec!["*".to_string()]
    }

    /// Returns the target property to predict.
    fn target_property(&self) -> &str;

    /// Returns the random seed for reproducibility.
    fn random_seed(&self) -> Option<u64>;

    /// Resolves target node labels from the graph store.
    ///
    /// This handles the special case of "*" (all labels) and validates
    /// that the specified labels exist in the graph.
    fn target_node_label_identifiers(
        &self,
        graph_store: &Arc<DefaultGraphStore>,
    ) -> HashSet<NodeLabel> {
        let target_labels = self.target_node_labels();

        if target_labels.len() == 1 && target_labels[0] == "*" {
            // Project all node labels
            graph_store.node_labels()
        } else {
            // Convert specified labels
            target_labels
                .iter()
                .map(|label| NodeLabel::of(label.as_str()))
                .collect()
        }
    }

    /// Returns node labels for filtering (same as target node labels).
    fn node_labels(&self) -> Vec<String> {
        self.target_node_labels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTrainConfig {
        pipeline_name: String,
        target_labels: Vec<String>,
        target_prop: String,
        seed: Option<u64>,
    }

    impl NodePropertyPipelineBaseTrainConfig for MockTrainConfig {
        fn pipeline(&self) -> &str {
            &self.pipeline_name
        }

        fn target_node_labels(&self) -> Vec<String> {
            self.target_labels.clone()
        }

        fn target_property(&self) -> &str {
            &self.target_prop
        }

        fn random_seed(&self) -> Option<u64> {
            self.seed
        }
    }

    #[test]
    fn test_default_target_node_labels() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let labels = config.target_node_labels();
        assert_eq!(labels, vec!["*"]);
    }

    #[test]
    fn test_node_labels_returns_target_labels() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string(), "Company".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let labels = config.node_labels();
        assert_eq!(labels, vec!["Person", "Company"]);
    }

    #[test]
    fn test_pipeline_name() {
        let config = MockTrainConfig {
            pipeline_name: "my-classification-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        assert_eq!(config.pipeline(), "my-classification-pipeline");
    }

    #[test]
    fn test_target_property() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "fraudScore".to_string(),
            seed: Some(42),
        };

        assert_eq!(config.target_property(), "fraudScore");
    }

    #[test]
    fn test_random_seed() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(12345),
        };

        assert_eq!(config.random_seed(), Some(12345));
    }

    #[test]
    fn test_no_random_seed() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: None,
        };

        assert_eq!(config.random_seed(), None);
    }
}
