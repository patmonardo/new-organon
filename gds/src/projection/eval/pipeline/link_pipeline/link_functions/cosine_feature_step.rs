// Phase 2.2: CosineFeatureStep - Angular similarity via cosine distance

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::projection::eval::pipeline::feature_step_util::property_dimension;
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::collections::HashMap;
use std::sync::Arc;

/// Cosine similarity link feature.
///
/// Computes cosine similarity between node property vectors.
#[derive(Debug, Clone)]
pub struct CosineFeatureStep {
    /// Node properties to compute cosine similarity on
    node_property_names: Vec<String>,
}

impl CosineFeatureStep {
    /// Creates a new CosineFeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self {
            node_property_names: node_properties,
        }
    }
}

impl LinkFeatureStep for CosineFeatureStep {
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        let computers = self
            .node_property_names
            .iter()
            .map(|property_name| create_computer(graph, property_name))
            .collect::<Result<Vec<_>, String>>()
            .expect("Failed to create cosine computers");

        Box::new(CosineAppender::new(computers))
    }

    fn name(&self) -> &str {
        "COSINE"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperties".to_string(),
            serde_json::json!(self.node_property_names),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_property_names.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

#[derive(Default, Debug)]
struct CosineComputationResult {
    source_square_norm: f64,
    target_square_norm: f64,
    dot_product: f64,
}

trait PartialL2WithNormsComputer: Send + Sync {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult);
}

struct CosineAppender {
    computers: Vec<Box<dyn PartialL2WithNormsComputer>>,
}

impl CosineAppender {
    fn new(computers: Vec<Box<dyn PartialL2WithNormsComputer>>) -> Self {
        Self { computers }
    }
}

impl LinkFeatureAppender for CosineAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let mut result = CosineComputationResult::default();

        for computer in &self.computers {
            computer.compute(source, target, &mut result);
        }

        let norm_product = (result.source_square_norm * result.target_square_norm).sqrt();
        let cosine = if norm_product > 0.0 {
            result.dot_product / norm_product
        } else {
            0.0
        };

        link_features[offset] = if cosine.is_finite() { cosine } else { 0.0 };
    }

    fn dimension(&self) -> usize {
        1
    }
}

fn create_computer(
    graph: &dyn Graph,
    property_name: &str,
) -> Result<Box<dyn PartialL2WithNormsComputer>, String> {
    let props = graph
        .node_properties(property_name)
        .ok_or_else(|| format!("Property '{}' not found in graph", property_name))?;

    let dimension = property_dimension(&*props, property_name).map_err(|e| {
        format!(
            "Failed to get dimension for property '{}': {:?}",
            property_name, e
        )
    })?;

    match props.value_type() {
        ValueType::DoubleArray => Ok(Box::new(DoubleArrayComputer { props, dimension })),
        ValueType::FloatArray => Ok(Box::new(FloatArrayComputer { props, dimension })),
        ValueType::LongArray => Ok(Box::new(LongArrayComputer { props, dimension })),
        ValueType::Double => Ok(Box::new(DoubleComputer { props })),
        ValueType::Long => Ok(Box::new(LongComputer { props })),
        other => Err(format!(
            "Unsupported ValueType {:?} for property '{}'",
            other, property_name
        )),
    }
}

struct DoubleArrayComputer {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl PartialL2WithNormsComputer for DoubleArrayComputer {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult) {
        let source_vec = match self.props.double_array_value(source) {
            Ok(vec) => vec,
            Err(_) => return,
        };
        let target_vec = match self.props.double_array_value(target) {
            Ok(vec) => vec,
            Err(_) => return,
        };

        for i in 0..self.dimension {
            let s_val = source_vec[i];
            let t_val = target_vec[i];
            result.dot_product += s_val * t_val;
            result.source_square_norm += s_val * s_val;
            result.target_square_norm += t_val * t_val;
        }
    }
}

struct FloatArrayComputer {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl PartialL2WithNormsComputer for FloatArrayComputer {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult) {
        let source_vec = match self.props.float_array_value(source) {
            Ok(vec) => vec,
            Err(_) => return,
        };
        let target_vec = match self.props.float_array_value(target) {
            Ok(vec) => vec,
            Err(_) => return,
        };

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            result.dot_product += s_val * t_val;
            result.source_square_norm += s_val * s_val;
            result.target_square_norm += t_val * t_val;
        }
    }
}

struct LongArrayComputer {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl PartialL2WithNormsComputer for LongArrayComputer {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult) {
        let source_vec = match self.props.long_array_value(source) {
            Ok(vec) => vec,
            Err(_) => return,
        };
        let target_vec = match self.props.long_array_value(target) {
            Ok(vec) => vec,
            Err(_) => return,
        };

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            result.dot_product += s_val * t_val;
            result.source_square_norm += s_val * s_val;
            result.target_square_norm += t_val * t_val;
        }
    }
}

struct DoubleComputer {
    props: Arc<dyn NodePropertyValues>,
}

impl PartialL2WithNormsComputer for DoubleComputer {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult) {
        let source_val = match self.props.double_value(source) {
            Ok(val) => val,
            Err(_) => return,
        };
        let target_val = match self.props.double_value(target) {
            Ok(val) => val,
            Err(_) => return,
        };

        result.dot_product += source_val * target_val;
        result.source_square_norm += source_val * source_val;
        result.target_square_norm += target_val * target_val;
    }
}

struct LongComputer {
    props: Arc<dyn NodePropertyValues>,
}

impl PartialL2WithNormsComputer for LongComputer {
    fn compute(&self, source: u64, target: u64, result: &mut CosineComputationResult) {
        let source_val = match self.props.long_value(source) {
            Ok(val) => val as f64,
            Err(_) => return,
        };
        let target_val = match self.props.long_value(target) {
            Ok(val) => val as f64,
            Err(_) => return,
        };

        result.dot_product += source_val * target_val;
        result.source_square_norm += source_val * source_val;
        result.target_square_norm += target_val * target_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_creation() {
        let step = CosineFeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_property_names.len(), 1);
    }

    #[test]
    fn test_cosine_name() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_cosine_configuration() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = CosineFeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_dimension_is_one() {
        // Cosine similarity always returns single scalar value
        let appender = CosineAppender::new(Vec::new());
        assert_eq!(appender.dimension(), 1);
    }

    #[test]
    fn test_multiple_properties() {
        // Cosine can combine multiple properties
        // (computes overall cosine across concatenated vectors)
        let step = CosineFeatureStep::new(vec!["embedding1".to_string(), "embedding2".to_string()]);

        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_clone() {
        let step1 = CosineFeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }
}
