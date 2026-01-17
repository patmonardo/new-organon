// Phase 2.8: UnionLinkFeatureAppender - Combines multiple property appenders into one

use super::super::LinkFeatureAppender;

/// Combines multiple link feature appenders into one feature vector.
///
/// The union appender:
/// - invokes each appender sequentially,
/// - advances offsets by each appender's dimension,
/// - validates the computed range for invalid values.
pub struct UnionLinkFeatureAppender {
    /// Array of appenders, one per property
    appender_per_property: Vec<Box<dyn LinkFeatureAppender>>,

    /// Name of the feature step (for error messages)
    feature_step_name: String,

    /// Input node properties (for error messages)
    input_node_properties: Vec<String>,

    /// Total dimension (sum of all appender dimensions)
    dimension: usize,
}

impl UnionLinkFeatureAppender {
    /// Creates a new UnionLinkFeatureAppender.
    ///
    /// # Arguments
    ///
    /// * `appender_per_property` - Array of appenders (one per property)
    /// * `feature_step_name` - Name of the feature step (e.g., "L2", "HADAMARD")
    /// * `input_node_properties` - List of property names
    ///
    /// # Returns
    ///
    /// Union appender with total dimension = sum of component dimensions.
    pub fn new(
        appender_per_property: Vec<Box<dyn LinkFeatureAppender>>,
        feature_step_name: String,
        input_node_properties: Vec<String>,
    ) -> Self {
        // Calculate total dimension
        let dimension = appender_per_property
            .iter()
            .map(|appender| appender.dimension())
            .sum();

        Self {
            appender_per_property,
            feature_step_name,
            input_node_properties,
            dimension,
        }
    }

    /// Returns the feature step name.
    pub fn feature_step_name(&self) -> &str {
        &self.feature_step_name
    }

    /// Returns the input node properties.
    pub fn input_node_properties(&self) -> &[String] {
        &self.input_node_properties
    }
}

impl LinkFeatureAppender for UnionLinkFeatureAppender {
    fn append_features(&self, source: u64, target: u64, features: &mut [f64], offset: usize) {
        let mut local_offset = offset;

        // Call each appender sequentially
        for appender in &self.appender_per_property {
            appender.append_features(source, target, features, local_offset);
            local_offset += appender.dimension();
        }

        // Validate computed features (NaN / Infinity) and include context if invalid.
        self.validate_features(features, offset, local_offset, source, target);
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl UnionLinkFeatureAppender {
    /// Validates that computed features are finite.
    fn validate_features(
        &self,
        features: &[f64],
        offset: usize,
        end_offset: usize,
        source: u64,
        target: u64,
    ) {
        for (index, feature) in features.iter().take(end_offset).skip(offset).enumerate() {
            if !feature.is_finite() {
                panic!(
                    "Invalid value (index {}, value {}) in {} feature computation for properties {:?} (source={}, target={})",
                    offset + index,
                    feature,
                    self.feature_step_name,
                    self.input_node_properties,
                    source,
                    target
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConstantAppender {
        value: f64,
        dimension: usize,
    }

    impl LinkFeatureAppender for ConstantAppender {
        fn append_features(&self, _source: u64, _target: u64, features: &mut [f64], offset: usize) {
            for i in 0..self.dimension {
                features[offset + i] = self.value;
            }
        }

        fn dimension(&self) -> usize {
            self.dimension
        }
    }

    #[test]
    fn test_union_multiple_appenders() {
        let appender_a: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0,
            dimension: 2,
        });
        let appender_b: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 2.0,
            dimension: 1,
        });
        let appender_c: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 3.0,
            dimension: 3,
        });

        let union = UnionLinkFeatureAppender::new(
            vec![appender_a, appender_b, appender_c],
            "TEST".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );

        let mut features = vec![0.0; 10];
        union.append_features(0, 1, &mut features, 0);

        assert_eq!(union.dimension(), 6);
        assert_eq!(features[0], 1.0);
        assert_eq!(features[1], 1.0);
        assert_eq!(features[2], 2.0);
        assert_eq!(features[3], 3.0);
        assert_eq!(features[4], 3.0);
        assert_eq!(features[5], 3.0);
    }

    #[test]
    fn test_union_with_offset() {
        let appenders: Vec<Box<dyn LinkFeatureAppender>> = vec![Box::new(ConstantAppender {
            value: 3.0,
            dimension: 1,
        })];

        let union =
            UnionLinkFeatureAppender::new(appenders, "TEST".to_string(), vec!["prop".to_string()]);

        let mut features = vec![0.0; 10];
        union.append_features(0, 1, &mut features, 5);

        assert_eq!(features[4], 0.0);
        assert_eq!(features[5], 3.0);
        assert_eq!(features[6], 0.0);
    }

    #[test]
    fn test_union_two_appenders() {
        let appender_a: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0,
            dimension: 1,
        });
        let appender_b: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0,
            dimension: 1,
        });

        let union = UnionLinkFeatureAppender::new(
            vec![appender_a, appender_b],
            "TEST".to_string(),
            vec!["a".to_string(), "b".to_string()],
        );

        assert_eq!(union.dimension(), 2);

        let mut features = vec![0.0; 5];
        union.append_features(0, 0, &mut features, 0);

        assert_eq!(features[0], 1.0);
        assert_eq!(features[1], 1.0);
    }

    #[test]
    fn test_union_three_appenders() {
        let appender_a: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0,
            dimension: 2,
        });
        let appender_b: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 2.0,
            dimension: 3,
        });
        let appender_c: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 3.0,
            dimension: 1,
        });

        let union = UnionLinkFeatureAppender::new(
            vec![appender_a, appender_b, appender_c],
            "TEST".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );

        assert_eq!(union.dimension(), 6);

        let mut feature_space = vec![0.0; 10];
        union.append_features(0, 1, &mut feature_space, 0);

        assert_eq!(feature_space[0], 1.0);
        assert_eq!(feature_space[1], 1.0);
        assert_eq!(feature_space[2], 2.0);
        assert_eq!(feature_space[3], 2.0);
        assert_eq!(feature_space[4], 2.0);
        assert_eq!(feature_space[5], 3.0);
    }
}
