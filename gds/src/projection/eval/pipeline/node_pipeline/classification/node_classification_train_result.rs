use crate::collections::long_multiset::LongMultiSet;
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::models::Classifier;
use crate::ml::training::statistics::TrainingStatistics;

/// Result of training a node classification model.
///
/// Contains the trained classifier, training statistics, class ID mappings, and class counts.
#[derive(Debug)]
pub struct NodeClassificationTrainResult {
    classifier: Box<dyn Classifier>,
    training_statistics: TrainingStatistics,
    class_id_map: LocalIdMap,
    class_counts: LongMultiSet,
}

impl NodeClassificationTrainResult {
    pub fn new(
        classifier: Box<dyn Classifier>,
        training_statistics: TrainingStatistics,
        class_id_map: LocalIdMap,
        class_counts: LongMultiSet,
    ) -> Self {
        Self {
            classifier,
            training_statistics,
            class_id_map,
            class_counts,
        }
    }

    pub fn classifier(&self) -> &dyn Classifier {
        &*self.classifier
    }

    pub fn into_classifier(self) -> Box<dyn Classifier> {
        self.classifier
    }

    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }

    pub fn class_id_map(&self) -> &LocalIdMap {
        &self.class_id_map
    }

    pub fn class_counts(&self) -> &LongMultiSet {
        &self.class_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::metrics::classification::GlobalAccuracy;
    use crate::ml::metrics::Metric;
    use crate::ml::models::Features;
    use crate::ml::core::tensor::Matrix;
    use crate::ml::models::base::{BaseModelData, ClassifierData};
    use crate::ml::models::training_method::TrainingMethod;
    use std::any::Any;

    #[derive(Debug)]
    struct TestClassifierData;

    impl BaseModelData for TestClassifierData {
        fn trainer_method(&self) -> TrainingMethod {
            TrainingMethod::LogisticRegression
        }

        fn feature_dimension(&self) -> usize {
            1
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ClassifierData for TestClassifierData {
        fn number_of_classes(&self) -> usize {
            2
        }
    }

    #[derive(Debug)]
    struct TestClassifier;

    impl Classifier for TestClassifier {
        fn data(&self) -> &dyn ClassifierData {
            &TestClassifierData
        }

        fn predict_probabilities(&self, _features: &[f64]) -> Vec<f64> {
            vec![0.5, 0.5]
        }

        fn predict_probabilities_batch(
            &self,
            batch: &[usize],
            _features: &dyn Features,
        ) -> Matrix {
            Matrix::new(vec![0.5; batch.len() * 2], batch.len(), 2)
        }
    }

    #[test]
    fn test_new_train_result() {
        let classifier = Box::new(TestClassifier) as Box<dyn Classifier>;
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let training_statistics = TrainingStatistics::new(&metrics);
        let class_id_map = LocalIdMap::of(&[0, 1, 2]);
        let class_counts = LongMultiSet::new();

        let result = NodeClassificationTrainResult::new(
            classifier,
            training_statistics,
            class_id_map,
            class_counts,
        );

        assert_eq!(result.class_id_map().size(), 3);
    }

    #[test]
    fn test_accessors() {
        let classifier = Box::new(TestClassifier) as Box<dyn Classifier>;
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let training_statistics = TrainingStatistics::new(&metrics);
        let class_id_map = LocalIdMap::of(&[10, 20, 30]);
        let class_counts = LongMultiSet::new();

        let result = NodeClassificationTrainResult::new(
            classifier,
            training_statistics,
            class_id_map.clone(),
            class_counts,
        );

        // Verify accessors return correct references
        assert_eq!(result.class_id_map().original_ids_list(), &[10, 20, 30]);
    }
}
