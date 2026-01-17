// Phase 2.3: L2FeatureStep - Euclidean distance between node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use super::abstract_link_feature_appender_factory::AbstractLinkFeatureAppenderFactory;
use super::union_link_feature_appender::UnionLinkFeatureAppender;
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// L2 distance (Euclidean distance) link feature.
///
/// Returns squared differences per dimension.
#[derive(Debug, Clone)]
pub struct L2FeatureStep {
    /// Node properties to compute L2 distance on
    node_properties: Vec<String>,
}

impl L2FeatureStep {
    /// Creates a new L2FeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for L2FeatureStep {
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        let factory = L2LinkFeatureAppenderFactory;
        let appenders = factory
            .create_appenders(graph, &self.node_properties)
            .expect("Failed to create L2 appenders");
        Box::new(UnionLinkFeatureAppender::new(
            appenders,
            "L2".to_string(),
            self.node_properties.clone(),
        ))
    }

    fn name(&self) -> &str {
        "L2"
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

/// Factory for creating L2 appenders.
struct L2LinkFeatureAppenderFactory;

impl AbstractLinkFeatureAppenderFactory for L2LinkFeatureAppenderFactory {
    fn double_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(L2DoubleArrayAppender { props, dimension })
    }

    fn float_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(L2FloatArrayAppender { props, dimension })
    }

    fn long_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(L2LongArrayAppender { props, dimension })
    }

    fn long_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(L2LongAppender {
            props,
            _dimension: dimension,
        })
    }

    fn double_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(L2DoubleAppender {
            props,
            _dimension: dimension,
        })
    }
}

/// L2 appender for f64[] properties.
struct L2DoubleArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for L2DoubleArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_vec = match self.props.double_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };
        let target_vec = match self.props.double_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                for i in 0..self.dimension {
                    link_features[offset + i] = 0.0;
                }
                return;
            }
        };

        // Compute squared differences: (source[i] - target[i])²
        for i in 0..self.dimension {
            let diff = source_vec[i] - target_vec[i];
            link_features[offset + i] = diff * diff;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// L2 appender for f32[] properties.
struct L2FloatArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for L2FloatArrayAppender {
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
            let diff = source_vec[i] as f64 - target_vec[i] as f64;
            link_features[offset + i] = diff * diff;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// L2 appender for i64[] properties.
struct L2LongArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for L2LongArrayAppender {
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
            let diff = source_vec[i] as f64 - target_vec[i] as f64;
            link_features[offset + i] = diff * diff;
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// L2 appender for i64 scalar properties.
struct L2LongAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for L2LongAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.long_value(source) {
            Ok(val) => val as f64,
            Err(_) => 0.0,
        };
        let target_val = match self.props.long_value(target) {
            Ok(val) => val as f64,
            Err(_) => 0.0,
        };

        let diff = source_val - target_val;
        link_features[offset] = diff * diff;
    }

    fn dimension(&self) -> usize {
        1
    }
}

/// L2 appender for f64 scalar properties.
struct L2DoubleAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for L2DoubleAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let source_val = match self.props.double_value(source) {
            Ok(val) => val,
            Err(_) => 0.0,
        };
        let target_val = match self.props.double_value(target) {
            Ok(val) => val,
            Err(_) => 0.0,
        };

        let diff = source_val - target_val;
        link_features[offset] = diff * diff;
    }

    fn dimension(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2_creation() {
        let step = L2FeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_l2_name() {
        let step = L2FeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_l2_configuration() {
        let step = L2FeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = L2FeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_multiple_properties() {
        let step = L2FeatureStep::new(vec![
            "pos_x".to_string(),
            "pos_y".to_string(),
            "pos_z".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_clone() {
        let step1 = L2FeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }

    #[test]
    fn test_semantic_meaning() {
        // L2 measures geometric distance
        // Smaller squared differences = closer nodes = more similar
        let step = L2FeatureStep::new(vec!["position".to_string()]);
        assert_eq!(step.name(), "L2");
    }
}
