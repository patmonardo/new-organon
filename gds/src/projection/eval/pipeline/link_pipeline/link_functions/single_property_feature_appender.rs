// Phase 2.7: SinglePropertyFeatureAppender - Base class for single-property appenders

use super::super::LinkFeatureAppender;
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

/// Base implementation for link feature appenders that operate on a single property.
pub struct SinglePropertyFeatureAppender {
    /// The node property values this appender operates on.
    props: Arc<dyn NodePropertyValues>,

    /// The feature dimension this appender produces.
    /// - Scalar properties: dimension = 1
    /// - Array properties: dimension = array length
    dimension: usize,

    /// Computes feature values for this property.
    compute: Box<dyn SinglePropertyCompute>,
}

impl SinglePropertyFeatureAppender {
    /// Creates a new SinglePropertyFeatureAppender.
    ///
    /// # Arguments
    ///
    /// * `props` - Node property values
    /// * `dimension` - Feature dimension
    /// * `compute` - Computation strategy for appending features
    pub fn new(
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
        compute: Box<dyn SinglePropertyCompute>,
    ) -> Self {
        Self {
            props,
            dimension,
            compute,
        }
    }

    /// Returns the property values.
    pub fn props(&self) -> &Arc<dyn NodePropertyValues> {
        &self.props
    }
}

/// Strategy for computing features for a single property.
pub trait SinglePropertyCompute: Send + Sync {
    fn append_features(
        &self,
        props: &dyn NodePropertyValues,
        dimension: usize,
        source: u64,
        target: u64,
        features: &mut [f64],
        offset: usize,
    );

    fn is_symmetric(&self) -> bool {
        true
    }
}

impl LinkFeatureAppender for SinglePropertyFeatureAppender {
    fn append_features(&self, source: u64, target: u64, features: &mut [f64], offset: usize) {
        self.compute.append_features(
            &*self.props,
            self.dimension,
            source,
            target,
            features,
            offset,
        );
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn is_symmetric(&self) -> bool {
        self.compute.is_symmetric()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::{PropertyValues, PropertyValuesError};
    use crate::types::ValueType;

    #[derive(Debug)]
    struct TestNodePropertyValues;

    impl PropertyValues for TestNodePropertyValues {
        fn value_type(&self) -> ValueType {
            ValueType::Double
        }

        fn element_count(&self) -> usize {
            0
        }
    }

    impl NodePropertyValues for TestNodePropertyValues {
        fn double_value(&self, _node_id: u64) -> Result<f64, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation("test double"))
        }

        fn long_value(&self, _node_id: u64) -> Result<i64, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation("test long"))
        }

        fn double_array_value(&self, _node_id: u64) -> Result<Vec<f64>, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation(
                "test double array",
            ))
        }

        fn float_array_value(&self, _node_id: u64) -> Result<Vec<f32>, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation(
                "test float array",
            ))
        }

        fn long_array_value(&self, _node_id: u64) -> Result<Vec<i64>, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation(
                "test long array",
            ))
        }

        fn get_object(&self, _node_id: u64) -> Result<Box<dyn std::any::Any>, PropertyValuesError> {
            Err(PropertyValuesError::unsupported_operation("test object"))
        }

        fn dimension(&self) -> Option<usize> {
            Some(1)
        }

        fn get_max_long_property_value(&self) -> Option<i64> {
            None
        }

        fn get_max_double_property_value(&self) -> Option<f64> {
            None
        }

        fn has_value(&self, _node_id: u64) -> bool {
            false
        }
    }

    struct ConstantCompute {
        value: f64,
        symmetric: bool,
    }

    impl SinglePropertyCompute for ConstantCompute {
        fn append_features(
            &self,
            _props: &dyn NodePropertyValues,
            dimension: usize,
            _source: u64,
            _target: u64,
            features: &mut [f64],
            offset: usize,
        ) {
            for i in 0..dimension {
                features[offset + i] = self.value;
            }
        }

        fn is_symmetric(&self) -> bool {
            self.symmetric
        }
    }

    #[test]
    fn test_single_property_creation() {
        let appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            10,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );
        assert_eq!(appender.dimension(), 10);
    }

    #[test]
    fn test_single_property_dimension() {
        let scalar_appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            1,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );
        let array_appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            128,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );

        assert_eq!(scalar_appender.dimension(), 1);
        assert_eq!(array_appender.dimension(), 128);
    }

    #[test]
    fn test_props_access() {
        let appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            5,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );
        let _props = appender.props();
    }

    #[test]
    fn test_single_property_dimension_one() {
        let musical_appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            1,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );
        assert_eq!(musical_appender.dimension(), 1);
    }

    #[test]
    fn test_single_property_dimension_multiple() {
        let appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            8,
            Box::new(ConstantCompute {
                value: 0.0,
                symmetric: true,
            }),
        );
        assert_eq!(appender.dimension(), 8);
    }

    #[test]
    fn test_single_property_appender_executes_compute() {
        let appender = SinglePropertyFeatureAppender::new(
            Arc::new(TestNodePropertyValues),
            3,
            Box::new(ConstantCompute {
                value: 2.5,
                symmetric: false,
            }),
        );

        let mut features = vec![0.0; 5];
        appender.append_features(0, 1, &mut features, 1);

        assert_eq!(features[1], 2.5);
        assert_eq!(features[2], 2.5);
        assert_eq!(features[3], 2.5);
        assert!(!appender.is_symmetric());
    }
}
