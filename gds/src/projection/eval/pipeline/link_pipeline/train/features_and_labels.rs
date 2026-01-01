// Phase 1.4: FeaturesAndLabels - Training data container for link prediction

/// Features type: vector of feature vectors (one per relationship)
pub type Features = Vec<Vec<f64>>;

/// Labels type: vector of binary labels (0 = negative, 1 = positive)
pub type Labels = Vec<i32>;

/// Pairs extracted features with their corresponding labels for training.
///
/// Used during link prediction training to bundle:
/// - **Features**: Computed link features (e.g., Hadamard, Cosine) for relationship pairs
/// - **Labels**: Binary labels (1 = positive/exists, 0 = negative/doesn't exist)
///
/// # Link Prediction Training Data
///
/// Link prediction is binary classification:
/// - **Positive examples**: Existing relationships (label = 1)
/// - **Negative examples**: Non-existent relationships from negative sampling (label = 0)
///
/// Features are computed for both positive and negative examples using link functions
/// on node properties.
///
/// # Example Flow
///
/// ```text
/// 1. Split relationships → train/test sets
/// 2. Generate negative samples → train_positive + train_negative
/// 3. Extract link features → Features (for all train pairs)
/// 4. Assign labels → Labels (1 for positive, 0 for negative)
/// 5. Bundle → FeaturesAndLabels
/// 6. Train classifier → Logistic Regression, Random Forest, etc.
/// ```
#[derive(Debug, Clone)]
pub struct FeaturesAndLabels {
    /// Extracted link features for training examples
    features: Features,

    /// Binary labels: 1 = relationship exists, 0 = doesn't exist
    labels: Labels,
}

impl FeaturesAndLabels {
    /// Creates a new FeaturesAndLabels from features and labels.
    ///
    /// # Arguments
    ///
    /// * `features` - Extracted link features for training examples
    /// * `labels` - Binary labels (1 = positive, 0 = negative)
    ///
    /// # Panics
    ///
    /// Panics if features and labels have different lengths.
    pub fn new(features: Features, labels: Labels) -> Self {
        assert_eq!(
            features.len(),
            labels.len(),
            "Features and labels must have the same length"
        );

        Self { features, labels }
    }

    /// Returns the features.
    pub fn features(&self) -> &Features {
        &self.features
    }

    /// Returns the labels.
    pub fn labels(&self) -> &Labels {
        &self.labels
    }

    /// Returns the number of training examples.
    ///
    /// Equal to the number of relationship pairs (positive + negative).
    pub fn size(&self) -> usize {
        self.features.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_and_labels_creation() {
        let features = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let labels = vec![1, 0];
        let data = FeaturesAndLabels::new(features, labels);

        assert_eq!(data.size(), 2);
    }

    #[test]
    fn test_accessors() {
        let features = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let labels = vec![1, 0];
        let data = FeaturesAndLabels::new(features.clone(), labels.clone());

        assert_eq!(data.features(), &features);
        assert_eq!(data.labels(), &labels);
        assert_eq!(data.size(), 2);
    }

    #[test]
    fn test_clone() {
        let features = vec![vec![1.0, 2.0]];
        let labels = vec![1];
        let data1 = FeaturesAndLabels::new(features, labels);
        let data2 = data1.clone();

        assert_eq!(data1.size(), data2.size());
    }

    #[test]
    fn test_zero_size() {
        let features: Features = vec![];
        let labels: Labels = vec![];
        let data = FeaturesAndLabels::new(features, labels);

        assert_eq!(data.size(), 0);
    }

    #[test]
    fn test_large_size() {
        let features: Features = (0..1000).map(|i| vec![i as f64, (i + 1) as f64]).collect();
        let labels: Labels = (0..1000).map(|i| (i % 2) as i32).collect();
        let data = FeaturesAndLabels::new(features, labels);

        assert_eq!(data.size(), 1000);
    }

    #[test]
    fn test_typical_sizes() {
        // Typical link prediction dataset sizes
        let small_features: Features = (0..10000).map(|i| vec![i as f64]).collect();
        let small_labels: Labels = (0..10000).map(|i| (i % 2) as i32).collect();
        let small = FeaturesAndLabels::new(small_features, small_labels);

        let medium_features: Features = (0..100000).map(|i| vec![i as f64]).collect();
        let medium_labels: Labels = (0..100000).map(|i| (i % 2) as i32).collect();
        let medium = FeaturesAndLabels::new(medium_features, medium_labels);

        let large_features: Features = (0..1000000).map(|i| vec![i as f64]).collect();
        let large_labels: Labels = (0..1000000).map(|i| (i % 2) as i32).collect();
        let large = FeaturesAndLabels::new(large_features, large_labels);

        assert_eq!(small.size(), 10_000);
        assert_eq!(medium.size(), 100_000);
        assert_eq!(large.size(), 1_000_000);
    }
}
