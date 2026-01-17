// Phase 2.1: HadamardFeatureStep - Element-wise multiplication of node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use super::abstract_link_feature_appender_factory::AbstractLinkFeatureAppenderFactory;
use super::union_link_feature_appender::UnionLinkFeatureAppender;
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Hadamard product (element-wise multiplication) link feature.
#[derive(Debug, Clone)]
pub struct HadamardFeatureStep {
    /// Node properties to compute Hadamard product on
    node_properties: Vec<String>,
}

impl HadamardFeatureStep {
    /// Creates a new HadamardFeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for HadamardFeatureStep {
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        let factory = HadamardLinkFeatureAppenderFactory;
        let appenders = factory
            .create_appenders(graph, &self.node_properties)
            .expect("Failed to create Hadamard appenders");
        Box::new(UnionLinkFeatureAppender::new(
            appenders,
            "HADAMARD".to_string(),
            self.node_properties.clone(),
        ))
    }

    fn name(&self) -> &str {
        "HADAMARD"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperties".to_string(),
            serde_json::json!(self.node_properties),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

/// Factory for creating Hadamard appenders.
struct HadamardLinkFeatureAppenderFactory;

impl AbstractLinkFeatureAppenderFactory for HadamardLinkFeatureAppenderFactory {
    fn double_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(HadamardDoubleArrayAppender { props, dimension })
    }

    fn float_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(HadamardFloatArrayAppender { props, dimension })
    }

    fn long_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(HadamardLongArrayAppender { props, dimension })
    }

    fn long_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(HadamardLongAppender {
            props,
            _dimension: dimension,
        })
    }

    fn double_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(HadamardDoubleAppender {
            props,
            _dimension: dimension,
        })
    }
}

/// Hadamard appender for f64[] properties.
struct HadamardDoubleArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for HadamardDoubleArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_vec = match self.props.double_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                // Fill with zeros if property not found
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };
        let target_vec = match self.props.double_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                // Fill with zeros if property not found
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };

        // Compute Hadamard product: element-wise multiplication
        for i in 0..self.dimension {
            let s_val = source_vec[i];
            let t_val = target_vec[i];
            link_features[offset + i] = s_val * t_val;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Hadamard appender for f32[] properties.
struct HadamardFloatArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for HadamardFloatArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_vec = match self.props.float_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };
        let target_vec = match self.props.float_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            link_features[offset + i] = s_val * t_val;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Hadamard appender for i64[] properties.
struct HadamardLongArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for HadamardLongArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_vec = match self.props.long_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };
        let target_vec = match self.props.long_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            link_features[offset + i] = s_val * t_val;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Hadamard appender for i64 scalar properties.
struct HadamardLongAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for HadamardLongAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.long_value(source) {
            Ok(val) => val as f64,
            Err(_) => 0.0,
        };
        let target_val = match self.props.long_value(target) {
            Ok(val) => val as f64,
            Err(_) => 0.0,
        };

        link_features[offset] = source_val * target_val;
    }

    fn dimension(&self) -> usize {
        1
    }
}

/// Hadamard appender for f64 scalar properties.
struct HadamardDoubleAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for HadamardDoubleAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.double_value(source) {
            Ok(val) => val,
            Err(_) => 0.0,
        };
        let target_val = match self.props.double_value(target) {
            Ok(val) => val,
            Err(_) => 0.0,
        };

        link_features[offset] = source_val * target_val;
    }

    fn dimension(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_creation() {
        let step = HadamardFeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_hadamard_name() {
        let step = HadamardFeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_hadamard_configuration() {
        let step = HadamardFeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = HadamardFeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_multiple_properties() {
        let step = HadamardFeatureStep::new(vec![
            "embedding1".to_string(),
            "embedding2".to_string(),
            "embedding3".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_clone() {
        let step1 = HadamardFeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }
}
